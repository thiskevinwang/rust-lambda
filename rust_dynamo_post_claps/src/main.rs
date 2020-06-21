use lambda::handler_fn;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub identity: Identity,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub source_ip: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Body {
    slug: String,
    claps: u32,
}

#[derive(Deserialize, Clone)]
struct CustomEvent {
    body: Option<String>,
    #[serde(rename = "requestContext")]
    request_context: Option<RequestContext>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: ::serde_json::Value,
    headers: ::serde_json::Value,
}

// init helper
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body: ::serde_json::Value::String(body),
            headers: json!({
                "Access-Control-Allow-Credentials": true,
                "Access-Control-Allow-Origin": "https://coffeecodeclimb.com",
                "Content-Type": "application/json",
            }),
        }
    }
    fn error(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 500,
            body: ::serde_json::Value::String(body),
            headers: json!({
                "Access-Control-Allow-Credentials": true,
                "Access-Control-Allow-Origin": "https://coffeecodeclimb.com",
                "Content-Type": "application/json",
            }),
        }
    }
}

const TABLE_NAME: &'static str = "claps";
async fn func(e: CustomEvent) -> Result<CustomOutput, Error> {
    let body: Body = if let Some(json_string) = e.body {
        let byte_vector = json_string.into_bytes();
        serde_json::from_slice(&byte_vector).unwrap()
    } else {
        Body {
            claps: 0,
            slug: "init".to_string(),
        }
    };

    let client = DynamoDbClient::new(Region::UsEast1);

    let ip: String = if let Some(rc) = e.request_context {
        rc.identity.source_ip
    } else {
        "0.0.0.0".to_string()
    };

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":inc".to_string(),
        AttributeValue {
            n: Some(format!("{}", body.claps)),
            ..Default::default()
        },
    );
    expression_attribute_values.insert(
        ":zero".to_string(),
        AttributeValue {
            n: Some(format!("0")),
            ..Default::default()
        },
    );
    expression_attribute_values.insert(
        ":limit".to_string(),
        AttributeValue {
            n: Some(format!("60")),
            ..Default::default()
        },
    );

    // Key of a User's claps
    let mut key = HashMap::new();
    key.insert(
        "PK".to_string(),
        AttributeValue {
            s: Some(format!("POST#{}", body.slug)),
            ..Default::default()
        },
    );
    key.insert(
        "SK".to_string(),
        AttributeValue {
            s: Some(format!("#CLAP#{}", ip)),
            ..Default::default()
        },
    );

    // Input to update a specific users's votes
    let update_item_input: UpdateItemInput = UpdateItemInput {
        table_name: String::from(TABLE_NAME),
        key: key.clone(),
        // 'SET #val = if_not_exists(#val, :zero) + :inc'
        update_expression: Some("SET claps = if_not_exists(claps, :zero) + :inc".to_string()),
        expression_attribute_values: Some(expression_attribute_values),
        // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html
        condition_expression: Some("attribute_not_exists(claps) OR (claps < :limit)".to_string()),
        return_values: Some("UPDATED_NEW".to_string()),
        ..Default::default()
    };

    let body: ::serde_json::Value;

    // Update POST#<slug> #CLAPS#<ip>
    match client.update_item(update_item_input).await {
        Ok(output) => {
            println!("Successfully incremented ITEM, {:?}", output);
            body = json!(output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
            // panic!("Error: {:?}", error);
            // TODO: Create proper response shape
            let error_response: ::serde_json::Value = json!({
                "Error": error.to_string(),
            });
            let response = CustomOutput::error(error_response.to_string());
            return Ok(response);
        }
    };

    let response = CustomOutput::new(body.to_string());
    Ok(response)
}

// https://forums.aws.amazon.com/message.jspa?messageID=629222
