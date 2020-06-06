#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# The folder name will be our function name
LAMBDA_NAME=${PWD##*/}
echo ${LAMBDA_NAME}

# Note: --cli-binary-format raw-in-base64-out is a required
# argument when using the AWS CLI version 2.

aws lambda invoke \
  --function-name ${LAMBDA_NAME} \
  --payload '{"firstName": "world"}' \
  --cli-binary-format raw-in-base64-out \
  output.json