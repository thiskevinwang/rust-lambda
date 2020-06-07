#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

# The folder name will be our function name
LAMBDA_NAME=${PWD##*/}
echo ${LAMBDA_NAME}

echo 🏗 building \`${LAMBDA_NAME}\`
cargo build --release --target x86_64-unknown-linux-musl
echo ✅ finished building

echo 🤐 zipping \`${LAMBDA_NAME}\`
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
echo ✅ finished zipping

echo ⬆️ uploading \`${LAMBDA_NAME}\` to AWS ☁️ 

aws lambda update-function-code \
  --function-name ${LAMBDA_NAME} \
  --zip-file fileb://rust.zip

echo 🎉 Successfully uploaded to \`${LAMBDA_NAME}\`