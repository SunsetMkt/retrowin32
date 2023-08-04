#!/bin/sh

exec cargo build --target x86_64-apple-darwin -p retrowin32 --no-default-features
