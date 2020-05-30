// using the correct Cargo package
// https://github.com/awslabs/aws-lambda-rust-runtime/issues/216

// example
// https://github.com/awslabs/aws-lambda-rust-runtime/pull/111
use lambda::handler_fn;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

async fn func(event: String) -> Result<String, Error> {
    Ok(event)
}

// Test this in the lambda console with a plain STRING as the
// `event` payload
