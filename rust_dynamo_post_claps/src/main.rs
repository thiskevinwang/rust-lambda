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
    pub domain_name: String,
    pub identity: Identity,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub source_ip: String,
}

#[derive(Deserialize, Clone)]
struct QueryString {
    #[serde(rename = "slug")]
    slug: Option<String>,
}

#[derive(Deserialize, Clone)]
struct Body {
    #[serde(rename = "slug")]
    slug: Option<String>,
}

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "queryStringParameters")]
    query_string_parameters: Option<QueryString>,
    body: Option<Body>,
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

// init helper
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: ::serde_json::Value::Bool(false),
            status_code: 200,
            body: ::serde_json::Value::String(body),
        }
    }
}

const TABLE_NAME: &'static str = "claps";
async fn func(e: CustomEvent) -> Result<CustomOutput, Error> {
    let client = DynamoDbClient::new(Region::UsEast1);

    // Slug to be shared between
    // - POST#<slug> #CLAPS#<ip>
    // - POST#<slug> #TOTAL
    let mut slug: String = "".to_string();
    if let Some(bd) = e.body {
        if let Some(bd_slug) = bd.slug {
            slug = bd_slug;
        }
    }

    let mut ip: String = "0.0.0.0".to_string();
    if let Some(rc) = e.request_context {
        ip = rc.identity.source_ip
    }

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":inc".to_string(),
        AttributeValue {
            n: Some(format!("1")),
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
            n: Some(format!("10")),
            ..Default::default()
        },
    );

    // Key of a User's claps
    let mut key = HashMap::new();
    key.insert(
        "PK".to_string(),
        AttributeValue {
            s: Some(format!("POST#{}", slug)),
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

    let mut totals_key = HashMap::new();
    totals_key.insert(
        "PK".to_string(),
        AttributeValue {
            s: Some(format!("POST#{}", slug)),
            ..Default::default()
        },
    );
    totals_key.insert(
        "SK".to_string(),
        AttributeValue {
            s: Some(format!("#TOTAL")),
            ..Default::default()
        },
    );

    let mut totals_expression_attribute_values = HashMap::new();
    totals_expression_attribute_values.insert(
        ":inc".to_string(),
        AttributeValue {
            n: Some(format!("1")),
            ..Default::default()
        },
    );
    totals_expression_attribute_values.insert(
        ":zero".to_string(),
        AttributeValue {
            n: Some(format!("0")),
            ..Default::default()
        },
    );

    let update_total_input: UpdateItemInput = UpdateItemInput {
        table_name: String::from(TABLE_NAME),
        key: totals_key.clone(),
        update_expression: Some("SET claps = if_not_exists(claps, :zero) + :inc".to_string()),
        expression_attribute_values: Some(totals_expression_attribute_values),
        // condition_expression: Some("attribute_not_exists(claps) OR (claps < :limit)".to_string()),
        return_values: Some("UPDATED_NEW".to_string()),
        ..Default::default()
    };

    let mut body: ::serde_json::Value = json!({});

    // Update POST#<slug> #CLAPS#<ip>
    match client.update_item(update_item_input).await {
        Ok(output) => {
            println!(
                "Successfully incremented: {:?} {:?}",
                key.get("PK"),
                key.get("SK")
            );
            body = json!(output);
        }
        Err(error) => {
            // println!("Error: {:?}", error);
            panic!("Error: {:?}", error);
            // TODO: Create proper response shape
        }
    };

    // ------------------------------
    // Currently there are 2 updates
    // The 2nd (updates a slug's total claps)
    // waits for the first (updates a users claps for a slug)
    //
    // the first will "cap" at 10, and panic
    //
    // TODO: figure out the best way to handle this
    // ------------------------------

    // Update POST#<slug> #TOTAL
    match client.update_item(update_total_input).await {
        Ok(output) => {
            println!(
                "Successfully incremented: {:?} {:?}",
                totals_key.get("PK"),
                totals_key.get("SK")
            );
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };

    let response = CustomOutput::new(body.to_string());
    Ok(response)
}

// https://forums.aws.amazon.com/message.jspa?messageID=629222
