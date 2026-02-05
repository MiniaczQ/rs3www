# rs3www

An `s3www`-like service, but written in Rust.

## Config

```toml
# Service exposure.
[service]
ip = "0.0.0.0"
port = 1234

# S3 credentials.
[s3]
access_key = "dev-access-key"
secret_key = "dev-secret-key"
url = "http://localhost:9000"
region = "us-east-1"
bucket = "dev-bucket"

# MIME derivation from file extension. Takes precedence over other options.
[content_type.extension_mapping]
# <extension> = <MIME type>
png = "image/png"

[content_type]
# Whether `Content-Type` returned by S3 should be used.
forward_s3_type = true
# Default MIME type.
fallback = "application/octet-stream"

[directory.explorer]
# How long is the directory listing cached for.
cache_seconds = 60
# Whether to show directory contents, when no index file is available.
enabled = true

[directory.index]
# How long is a hit cached for.
cache_seconds = 60
# Files to display when accessing a directory, checked in order.
files = [
    "index.html",
    "index.pdf"
]
```
