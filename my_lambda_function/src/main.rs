#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

// #[derive(Deserialize, Clone)]
// struct CustomEvent {
//     #[serde(rename = "firstName")]
//     first_name: String,
//     #[serde(rename = "lastName")]
//     last_name: String,
// }
#[derive(Serialize, Clone, Debug)]
struct Response {
    body: String,
    #[serde(rename = "statusCode")]
    status_code: i32,
    headers: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: Root, c: lambda::Context) -> Result<Response, HandlerError> {
    // if e.first_name == "" {
    //     error!("Empty first name in request {}", c.aws_request_id);
    //     return Err(c.new_error("Empty first name"));
    // }

    println!("TEST TEST TEST");
    println!("{:?}", e);

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let response = Response {
        body: e.path,
        status_code: 200,
        headers: headers,
    };
    println("RES RES RES");
    println!("{:?}", response);
    Ok(response)
}

// https://transform.tools/json-to-rust-serde

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub resource: String,
    pub path: String,
    pub http_method: String,
    pub headers: ::serde_json::Value,
    pub multi_value_headers: ::serde_json::Value,
    pub query_string_parameters: ::serde_json::Value,
    pub multi_value_query_string_parameters: ::serde_json::Value,
    pub path_parameters: ::serde_json::Value,
    pub stage_variables: ::serde_json::Value,
    pub request_context: RequestContext,
    pub body: ::serde_json::Value,
    pub is_base64_encoded: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub resource_id: String,
    pub resource_path: String,
    pub http_method: String,
    pub extended_request_id: String,
    pub request_time: String,
    pub path: String,
    pub account_id: String,
    pub protocol: String,
    pub stage: String,
    pub domain_prefix: String,
    pub request_time_epoch: i64,
    pub request_id: String,
    pub identity: Identity,
    pub domain_name: String,
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub cognito_identity_pool_id: ::serde_json::Value,
    pub cognito_identity_id: ::serde_json::Value,
    pub api_key: String,
    pub principal_org_id: ::serde_json::Value,
    pub cognito_authentication_type: ::serde_json::Value,
    pub user_arn: String,
    pub api_key_id: String,
    pub user_agent: String,
    pub account_id: String,
    pub caller: String,
    pub source_ip: String,
    pub access_key: String,
    pub cognito_authentication_provider: ::serde_json::Value,
    pub user: String,
}
