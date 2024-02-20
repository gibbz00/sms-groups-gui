use std::{ops::Deref, sync::LazyLock};

use anyhow::Context;
use hmac::{digest::KeyInit, Hmac};
use jwt::SignWithKey;
use poem_openapi::{param::Query, payload::PlainText, OpenApi};
use serde::{Deserialize, Serialize};

pub struct RestService;

/*

    1. Create a client at the IDM:
    ```sh
    kanidm system oauth2 create sms_groups_client "SMS Grupper API" "https://redirectmeto.com"
    ```

    2. Add the respective scopes:
    ```sh
    kanidm system oauth2 update-scope-map sms_groups_client idm_all_accounts openid user
    ```

    3. Disable PKCE
    ```sh
    kanidm system oauth2 warning-insecure-client-disable-pkce sms_groups_client
    ```

    4. Request the client password from IDM:
    ```
    kanidm system oauth2 show-basic-secret sms_groups_client
    ```
*/

// IMPROVEMENT: could be created by only using the OpenID discover endpoint.
const OAUTH2_AUTH_ENDPOINT: &str = "https://localhost/ui/oauth2";
const OAUTH2_TOKEN_ENDPOINT: &str = "https://localhost/oauth2/token";

const CLIENT_ID: &str = "sms_groups_client";
const CLIENT_PASS: &str = "vZHcY8HX0b6dgNAGDZqeAvFwgC9srgQVtHfKZu9d4Qv74Fhx";

// WORKAROUND: localhost:443 (https) currently occupied by Kanidm
const CLIENT_ORIGIN: &str = "https://redirectmeto.com";
const REAL_ORIGIN: &str = "http://localhost:3000";

const JWT_SIGNATURE_SECRET: &[u8] = b"super_secret";

#[OpenApi]
impl RestService {
    #[oai(path = "/user/login", method = "get")]
    async fn login(&self) -> PlainText<String> {
        let url = generate_authorization_request_url();
        return PlainText(format!("Please login at: {url}"));

        fn generate_authorization_request_url() -> String {
            format!(
                "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&state={}",
                OAUTH2_AUTH_ENDPOINT,
                CLIENT_ID,
                redirect_uri(),
                scopes_string(&["openid", "user"]),
                state()
            )
        }
    }

    #[oai(path = "/redirect", method = "get", hidden)]
    async fn redirect(&self, Query(state): Query<String>, Query(code): Query<String>) -> poem::Result<PlainText<String>> {
        println!("state: {}", state);

        let response = REQWEST_CLIENT
            .post(OAUTH2_TOKEN_ENDPOINT)
            .basic_auth(CLIENT_ID, Some(CLIENT_PASS))
            .form(&token_request_form(code))
            .send()
            .await
            .context("unable to send token request")?
            .text()
            .await
            .context("unable to convert token request to text")?;

        tracing::info!("Recieved token request response: {}", response);

        validate_response(&response)?;

        let sms_groups_jwt = create_sms_groups_jwt(&response)?;

        return Ok(PlainText(format!("Bearer token: {}", sms_groups_jwt)));

        fn token_request_form(code: String) -> impl Serialize {
            [
                ("grant_type", "authorization_code".to_string()),
                ("code", code),
                // Must be identical to the authorization request, but isn't being used by the token endpoint for redirection.
                ("redirect_uri", redirect_uri()),
            ]
        }
    }
}

static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("invalid request builder setup")
});

fn redirect_uri() -> String {
    format!("{CLIENT_ORIGIN}/{REAL_ORIGIN}/redirect")
}

fn scopes_string(scopes: &[&str]) -> String {
    scopes.join(" ")
}

fn state() -> String {
    "hejs".to_string()
}

fn validate_response(_response: &str) -> anyhow::Result<()> {
    // Usual procedure... also verify that the user scope exists?
    Ok(())
}

static JWT_SIGNING_KEY: LazyLock<Hmac<Sha256>> =
    LazyLock::new(|| Hmac::<Sha256>::new_from_slice(JWT_SIGNATURE_SECRET).expect("unable to create JWT signing key"));

fn create_sms_groups_jwt(_response: &str) -> anyhow::Result<String> {
    // TODO: map expriy etc.
    SmsGroupsJwt::mock().sign_with_key(JWT_SIGNING_KEY.deref()).map_err(Into::into)
}

use sha2::Sha256;
pub use sms_groups_jwt::SmsGroupsJwt;
mod sms_groups_jwt {
    use bson::oid::ObjectId;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Group {
        Admin,
        User,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SmsGroupsJwt {
        group: Group,
        id: ObjectId,
    }

    impl SmsGroupsJwt {
        pub fn mock() -> Self {
            Self {
                group: Group::User,
                id: ObjectId::new(),
            }
        }
    }
}
