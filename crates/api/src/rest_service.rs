use std::{ops::Deref, sync::LazyLock};

use anyhow::Context;
use bson::{doc, oid::ObjectId};
use hmac::{digest::KeyInit, Hmac};
use jwt::SignWithKey;
use poem_openapi::{
    param::{Path, Query},
    payload::PlainText,
    OpenApi,
};
use serde::Serialize;
use sha2::Sha256;
use sms_groups_common::*;
use url::Url;

pub struct RestService {
    pub db: MongoDbClient,
}

/*
    1. Create a client at the IDM:
    ```sh
    kanidm system oauth2 create sms_groups_client "SMS Grupper API" "https://redirectmeto.com"
    ```

    2. Add the respective scopes:
    ```sh
    kanidm system oauth2 update-scope-map sms_groups idm_all_accounts openid user
    kanidm system oauth2 update-scope-map sms_groups idm_admins openid admin
    ```

    3. Disable PKCE
    ```sh
    kanidm system oauth2 warning-insecure-client-disable-pkce sms_groups
    ```

    4. Request the client password from IDM:
    ```
    kanidm system oauth2 show-basic-secret sms_groups
    ```
*/

const JWT_SIGNATURE_SECRET: &[u8] = b"super_secret";
static JWT_SIGNING_KEY: LazyLock<Hmac<Sha256>> =
    LazyLock::new(|| Hmac::<Sha256>::new_from_slice(JWT_SIGNATURE_SECRET).expect("unable to create JWT signing key"));

static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("invalid request builder setup")
});

#[OpenApi]
impl RestService {
    #[oai(path = "/user/login/:organization_name", method = "get")]
    async fn login(&self, Path(organization_name): Path<String>) -> poem::Result<PlainText<String>> {
        let found_organization = get_org_by_name(&self.db, &organization_name).await?;
        let url = generate_authorization_request_url(found_organization, Group::User).await?;

        return Ok(PlainText(format!("Please login at: {url}")));

        async fn generate_authorization_request_url(organization: Organization, group: Group) -> anyhow::Result<Url> {
            let mut authorization_endpoint = get_provider_metadata(organization.authorization_server.issuer_url)
                .await?
                .authorization_endpoint;
            authorization_endpoint
                .query_pairs_mut()
                .append_pair("cliend_it", &organization.authorization_server.client_id)
                .append_pair("response_type", "code")
                .append_pair("redirect_uri", redirect_uri(group)?.as_str())
                .append_pair("scope", &["openid", group.as_ref()].join(" "))
                .append_pair("state", &organization.name);

            Ok(authorization_endpoint)
        }
    }

    #[oai(path = "/user/redirect", method = "get", hidden)]
    // State query string could also be extracted.
    async fn redirect(&self, Query(code): Query<String>, Query(state): Query<String>) -> poem::Result<PlainText<String>> {
        let organization_name = state;
        let organization = get_org_by_name(&self.db, &organization_name).await?;

        let token_endpoint = get_provider_metadata(organization.authorization_server.issuer_url)
            .await?
            .token_endpoint;

        let group = Group::User;

        let response = REQWEST_CLIENT
            .post(token_endpoint)
            .basic_auth(
                organization.authorization_server.client_id,
                Some(organization.authorization_server.client_password),
            )
            .form(&token_request_form(code, group))
            .send()
            .await
            .context("unable to send token request")?
            .text()
            .await
            .context("unable to convert token request to text")?;

        tracing::info!("Recieved token request response: {}", response);

        validate_response(&response)?;

        let sms_groups_jwt = create_sms_groups_jwt(&response, group)?;

        return Ok(PlainText(format!("Bearer token: {}", sms_groups_jwt)));

        fn token_request_form(code: String, group: Group) -> impl Serialize {
            [
                ("grant_type", "authorization_code".to_string()),
                ("code", code),
                // Must be identical to the authorization request, but isn't being used by the token endpoint for redirection.
                ("redirect_uri", redirect_uri(group).unwrap().to_string()),
            ]
        }
    }

    #[oai(path = "/user/test_auth", method = "get")]
    async fn test_user_auth(&self, UserAuthentication(user_id): UserAuthentication) -> PlainText<String> {
        PlainText(format!("Logged in as a user with id: {}", user_id))
    }

    #[oai(path = "/admin/test_auth", method = "get")]
    async fn test_admin_auth(&self, AdminAuthentication(admin_id): AdminAuthentication) -> PlainText<String> {
        PlainText(format!("Logged in as an admin with id: {}", admin_id))
    }
}

async fn get_org_by_name(db: &MongoDbClient, organization_name: &str) -> anyhow::Result<Organization> {
    db.get_collection::<Organization>()
        .find_one(Some(doc! {"name": &organization_name}), None)
        .await
        .context("error communicating with database")?
        .ok_or_else(|| anyhow::anyhow!("No organization found with name: {}", organization_name))
}

fn redirect_uri(group: Group) -> anyhow::Result<Url> {
    SmsGroupsConfig::read()?
        .api
        .origin
        .join("redirect/")
        .expect("invalid url redirect concatenation")
        .join(group.as_ref())
        .map_err(Into::into)
}

fn validate_response(_response: &str) -> anyhow::Result<()> {
    // Usual procedure... also verify that the user scope exists?
    Ok(())
}

fn create_sms_groups_jwt(_response: &str, group: Group) -> anyhow::Result<String> {
    // TODO: map expriy etc.

    // TODO: retrieve username from response and lookup in user database
    // fn find_user_id(username: &str) -> ObjectId
    let mock_user_id = ObjectId::new();

    SmsGroupsJwt { group, id: mock_user_id }
        .sign_with_key(JWT_SIGNING_KEY.deref())
        .map_err(Into::into)
}

pub use sms_groups_jwt::{Group, SmsGroupsJwt};
mod sms_groups_jwt {
    use bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};
    use strum::AsRefStr;

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, AsRefStr)]
    #[strum(serialize_all = "kebab-case")]
    pub enum Group {
        Admin,
        User,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SmsGroupsJwt {
        pub group: Group,
        pub id: ObjectId,
    }
}

pub use openid::get_provider_metadata;
mod openid {
    use std::ops::Deref;

    use anyhow::Context;
    use serde::Deserialize;
    use url::Url;

    const OPENID_DISCOVERY_POSTFIX: &str = ".well-known/openid-configuration";

    pub async fn get_provider_metadata(issuer_url: Url) -> anyhow::Result<OpenIdProviderMetadatda> {
        let metadata_url = {
            let mut issuer_url = issuer_url.to_string();
            if !issuer_url.ends_with('/') {
                issuer_url.push('/')
            }
            issuer_url.push_str(OPENID_DISCOVERY_POSTFIX);
            issuer_url
        };

        let x = super::REQWEST_CLIENT.deref().get(metadata_url).send().await?;

        tracing::error!(?x, "got response");

        let body = x.text().await.unwrap();
        tracing::error!(body, "with body");

        serde_json::from_str(&body).context("unable to request provider metadata")
    }

    /// Currently only using fields of interest.
    /// https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
    #[derive(Deserialize)]
    pub struct OpenIdProviderMetadatda {
        pub issuer: Url,
        pub authorization_endpoint: Url,
        pub token_endpoint: Url,
        pub userinfo_endpoint: Option<Url>,
        pub jwks_uri: Url,
        pub scopes_supported: Vec<String>,
    }
}

pub use authentication::{AdminAuthentication, UserAuthentication};
mod authentication {
    use anyhow::anyhow;
    use jwt::VerifyWithKey;
    use poem::Request;
    use poem_openapi::{auth::Bearer, SecurityScheme};

    use super::*;

    #[derive(SecurityScheme)]
    #[oai(ty = "bearer", checker = "user_api_checker")]
    pub struct UserAuthentication(pub ObjectId);

    #[derive(SecurityScheme)]
    #[oai(ty = "bearer", checker = "admin_api_checker")]
    pub struct AdminAuthentication(pub ObjectId);

    async fn user_api_checker(_req: &Request, bearer: Bearer) -> poem::Result<ObjectId> {
        api_checker(Group::User, bearer).await
    }

    async fn admin_api_checker(_req: &Request, bearer: Bearer) -> poem::Result<ObjectId> {
        api_checker(Group::Admin, bearer).await
    }

    async fn api_checker(expected_group: Group, bearer: Bearer) -> poem::Result<ObjectId> {
        let jwt = bearer
            .token
            .verify_with_key(JWT_SIGNING_KEY.deref())
            .context("unable to verify SMS Groups JWT")?;

        let SmsGroupsJwt { group, id } = jwt;

        (expected_group == group)
            .then_some(id)
            .ok_or(anyhow!("group mismatch, expected {}, got {}", expected_group.as_ref(), group.as_ref()).into())
    }
}
