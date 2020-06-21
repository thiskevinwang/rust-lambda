use lambda::handler_fn;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, QueryError, QueryInput, QueryOutput,
};
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

#[derive(Deserialize, Clone)]
struct QueryString {
    #[serde(rename = "slug")]
    slug: Option<String>,
}

// #[derive(Deserialize, Clone)]
// struct Body {
//     #[serde(rename = "firstName")]
//     first_name: Option<String>,
// }

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
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: ::serde_json::Value,
    headers: ::serde_json::Value,
}

// Just a static method to help us build the `CustomOutput`.
impl CustomOutput {
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
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

const TABLE_NAME: &'static str = "claps";
async fn func(e: CustomEvent) -> Result<CustomOutput, Error> {
    let client = DynamoDbClient::new(Region::UsEast1);

    let mut slug: String = "".to_string();
    if let Some(qs) = e.query_string_parameters {
        if let Some(qs_slug) = qs.slug {
            slug = qs_slug;
        }
    }

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":pk".to_string(),
        AttributeValue {
            s: Some(format!("POST#{}", slug)),
            ..Default::default()
        },
    );
    expression_attribute_values.insert(
        ":sk".to_string(),
        AttributeValue {
            s: Some(format!("#CLAP#")),
            ..Default::default()
        },
    );

    // ip of the viewer
    let ip: String = if let Some(rc) = e.request_context {
        rc.identity.source_ip
    } else {
        "0.0.0.0".to_string()
    };

    let query_input: QueryInput = QueryInput {
        table_name: String::from(TABLE_NAME),
        key_condition_expression: Some("PK = :pk AND begins_with(SK, :sk)".to_string()),
        exclusive_start_key: None,
        expression_attribute_values: Some(expression_attribute_values),
        ..Default::default()
    };

    let body: ::serde_json::Value;
    let dynamo_output: QueryOutput = client.query(query_input).await?;
    let items = dynamo_output.items.unwrap();
    let voter_count = dynamo_output.count.unwrap();

    let mut total: i32 = 0;
    let mut viewer_clap_count: i32 = 0;

    for item in &items {
        // accumulate total
        let claps_attribute_value: &AttributeValue = item.get("claps").unwrap();
        let claps_string: String = claps_attribute_value.n.clone().unwrap();
        let claps: i32 = claps_string.parse::<i32>().unwrap();
        total = total + claps;

        // update viewer_clap_count
        // - match IP
        // SK = #CLAP#151.205.100.100
        let sk_attribute_value: &AttributeValue = item.get("SK").unwrap();
        let sk_string: String = sk_attribute_value.s.clone().unwrap();

        let sk_ip = sk_string[6..].to_string();
        if sk_ip == ip {
            viewer_clap_count = claps
        }
    }
    body = json!({
        "slug": slug,
        "total": total,
        "viewerClapCount": viewer_clap_count,
        "voterCount": voter_count,
        // "ip": ip
    });

    let response = CustomOutput::new(body.to_string());
    Ok(response)
}
