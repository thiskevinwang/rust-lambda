#!/bin/bash
set -e
# Any subsequent(*) commands which fail will cause the shell script to exit immediately

cargo build --release --target x86_64-unknown-linux-musl

zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

aws lambda update-function-code \
  --function-name test-rust \
  --zip-file fileb://rust.zip