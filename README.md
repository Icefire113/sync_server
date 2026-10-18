# sync_server

## Deployment

sync_server only exposes a plain HTTP listener (`HOST`/`PORT`, default
`0.0.0.0:3000`). It does not terminate TLS or manage certificates itself.

If the server is reachable from outside a trusted network, it is strongly
recommended to run it behind a reverse proxy (nginx, Caddy, Traefik, ...) that
terminates TLS and forwards requests to it.

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
