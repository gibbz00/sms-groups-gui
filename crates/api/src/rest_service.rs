use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};
use sms_groups_common::*;

pub struct RestService;

#[OpenApi]
impl RestService {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    #[oai(path = "/", method = "post")]
    async fn create_organization(&self) -> poem::Result<Json<Organization>> {
        todo!()
    }
}
