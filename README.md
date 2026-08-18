# sync_server

## Storage Options

Currently there are 2 supported storage backends local storage and S3

### Local Storage

Config looks like:

```json
  "storage_backend": {
    "type": "Local",
    "config": {
      "path": "./storage"
    }
  }
```

### S3

Pulls auth in from env the rest of the config looks like:

```json
  "storage_backend": {
    "type": "S3",
    "config": {
      "bucket": "sync-server",
      "region": "us-west-garage",
      "endpoint_url": "https://s3.garage.internal"
    }
  }
```

## Features

### `response-compression`

Enables compression of response bodies using one of the following encodings:

- brotli
- gzip
- zstd

### `log-to-file`

Enables logging to `./sync_server.log` as well as to stderr
