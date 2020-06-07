use lambda::handler_fn;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RequestContext {
    pub identity: Identity,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub source_ip: String,
}

#[derive(Deserialize, Clone)]
struct QueryString {
    slug: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Body {
    slug: String,
    claps: u32,
}

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<String>,
    #[serde(rename = "requestContext")]
    request_context: Option<RequestContext>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: ::serde_json::Value,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: ::serde_json::Value,
    headers: ::serde_json::Value,
}

// init helper
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: ::serde_json::Value::Bool(false),
            status_code: 200,
            body: ::serde_json::Value::String(body),
            headers: json!({
                "Access-Control-Allow-Credentials": true,
                "Access-Control-Allow-Origin": "*",
                "Content-Type": "application/json",
            }),
        }
    }
}
async fn func(e: CustomEvent) -> Result<CustomOutput, Error> {
    let mut e_body: Body = Body {
        claps: 0,
        slug: "init".to_string(),
    };

    if let Some(json_string) = e.body {
        let byte_vector = json_string.into_bytes();
        e_body = serde_json::from_slice(&byte_vector).unwrap();
    }

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(rc) = e.request_context {
        ip = rc.identity.source_ip
    }

    let body: ::serde_json::Value = json!({
        "slug": e_body.slug,
        "claps": e_body.claps
    });

    let response = CustomOutput::new(body.to_string());
    Ok(response)
}

// https://forums.aws.amazon.com/message.jspa?messageID=629222
