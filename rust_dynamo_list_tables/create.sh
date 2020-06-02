#!/bin/bash
set -e

AWS_IAM_ROLE=$(grep AWS_IAM_ROLE .env | cut -d '=' -f2)
LAMBDA_NAME="rust_dynamo_list_tables"
FILE_NAME="rust.zip"

echo "AWS_IAM_ROLE = ${AWS_IAM_ROLE}"

echo "🏗 building \`${LAMBDA_NAME}\`"
cargo build --release --target x86_64-unknown-linux-musl
echo "✅ finished building"

echo "🤐 zipping \`${LAMBDA_NAME}\` -> \`${FILE_NAME}\`"
zip -j ${FILE_NAME} ./target/x86_64-unknown-linux-musl/release/bootstrap
echo "✅ finished zipping"

echo "⬆️ creating new function \`${LAMBDA_NAME}\`"

aws lambda create-function \
  --function-name ${LAMBDA_NAME} \
  --runtime provided \
  --handler doesnt.matter \
  --zip-file fileb://./${FILE_NAME} \
  --role ${AWS_IAM_ROLE} \
  --environment Variables={RUST_BACKTRACE=1}

# An error occurred (InvalidParameterValueException) when calling the
# CreateFunction operation: The provided execution role does not have
# permissions to call PutTraceSegments on XRAY
  # --tracing-config Mode=Active

echo "🎉 Successfully created \`${LAMBDA_NAME}\`"