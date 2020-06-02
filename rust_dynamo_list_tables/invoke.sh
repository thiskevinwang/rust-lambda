#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# LAMBDA_NAME="test-rust"
LAMBDA_NAME="rust_dynamo_list_tables"

# Note: --cli-binary-format raw-in-base64-out is a required
# argument when using the AWS CLI version 2.

aws lambda invoke \
  --function-name ${LAMBDA_NAME} \
  --payload '{"firstName": "world"}' \
  --cli-binary-format raw-in-base64-out \
  output.json