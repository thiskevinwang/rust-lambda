// using the correct Cargo package
// https://github.com/awslabs/aws-lambda-rust-runtime/issues/216

// example
// https://github.com/awslabs/aws-lambda-rust-runtime/pull/111
use lambda::handler_fn;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

// https://github.com/rusoto/rusoto
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput};

use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

// ---------------------------------------
//
// ---------------------------------------

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

#[derive(Deserialize, Clone)]
struct QueryString {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Body {
    #[serde(rename = "firstName")]
    first_name: Option<String>,
}

#[derive(Deserialize, Clone)]
struct CustomEvent {
    // note that we're using serde to help us to change
    // the names of parameters accordingly to conventions.
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
}

// Just a static method to help us build the `CustomOutput`.
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: ::serde_json::Value::Bool(false),
            status_code: 200,
            body: ::serde_json::Value::String(body),
        }
    }
}
// TableName: TableNames.SNACKS,
// KeyConditionExpression: "PK = :pk and begins_with(SK, :sk)",
// // KeyConditionExpression: "#PK = :pk and #SK = :sk",
// // ExpressionAttributeNames: {
// //   "#PK": "PK",
// //   "#SK": "SK",
// // },
// ExclusiveStartKey: exclusiveStartKey,
// ExpressionAttributeValues: {
//   ":pk": `TEAM#${teamName}`,
//   ":sk": `#CHANNEL#${channelName}` + `#MESSAGE#`,
//   // ":value": "Z",
// },
// // ProjectionExpression: "PK, Tastes",
// Limit: 10,
const TABLE_NAME: &'static str = "github-webhook-events";
async fn func(e: CustomEvent) -> Result<CustomOutput, Error> {
    let client = DynamoDbClient::new(Region::UsEast1);

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":pk".to_string(),
        AttributeValue {
            s: Some("REPO_NAME#thiskevinwang/rust-lambda".to_string()),
            ..Default::default()
        },
    );
    // expression_attribute_values.insert(
    //     ":sk".to_string(),
    //     AttributeValue {
    //         s: Some("".to_string()),
    //         ..Default::default()
    //     },
    // );

    let query_input: QueryInput = QueryInput {
        table_name: String::from(TABLE_NAME),
        key_condition_expression: Some("PK = :pk".to_string()),
        exclusive_start_key: None,
        expression_attribute_values: Some(expression_attribute_values),
        limit: Some(10),
        ..Default::default()
    };

    let mut body: ::serde_json::Value = json!({});
    match client.query(query_input).await {
        Ok(output) => {
            // [dependencies.rusoto_dynamodb]
            // version = "0.43.0"
            // default-features = false
            // features = ["rustls", "serialize_structs"]
            //                        ^^^
            // If you don't specify this feature, you get an error
            body = json!(output);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    // https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no
    // the trait bound `std::option::NoneError: std::error::Error` is not satisfied
    // the trait `std::error::Error` is not implemented for `std::option::NoneError`
    // note: required because of the requirements on the impl of `std::convert::From<std::option::NoneError>`
    // for `std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>`

    // let response: CustomOutput = CustomOutput {
    //     is_base64_encoded: ::serde_json::Value::Bool(false),
    //     status_code: 200,
    //     // The body field, if you're returning JSON, must be converted to a string to prevent further problems with the response.
    //     // You can use JSON.stringify to handle this in Node.js functions.
    //     // Other runtimes require different solutions, but the concept is the same.
    //     // https://aws.amazon.com/premiumsupport/knowledge-center/malformed-502-api-gateway/
    //     // body: body,
    //     // body: ::serde_json::Value::String(format!(
    //     //     "Hello from Rust, my dear default user! No parameters"
    //     // )),
    //     body: ::serde_json::Value::String(body.to_string()),
    // };

    let response = CustomOutput::new(body.to_string());
    Ok(response)
}

// Test this in the lambda console with a plain STRING as the
// `event` payload
