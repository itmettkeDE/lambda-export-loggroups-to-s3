#!/usr/bin/env bash
set -e

cross build --release --target x86_64-unknown-linux-musl "${@}"
strip "./target/x86_64-unknown-linux-musl/release/lambda-export-loggroups-to-s3"
cp "./target/x86_64-unknown-linux-musl/release/lambda-export-loggroups-to-s3" ./target/x86_64-unknown-linux-musl/release/bootstrap 
zip -r9 -j "./lambda-export-loggroups-to-s3.zip" ./target/x86_64-unknown-linux-musl/release/bootstrap
