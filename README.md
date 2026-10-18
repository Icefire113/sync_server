# sync_server

## Deployment

sync_server only exposes a plain HTTP listener (`HOST`/`PORT`, default
`0.0.0.0:3000`). It does not terminate TLS or manage certificates itself.

If the server is reachable from outside a trusted network, it is strongly
recommended to run it behind a reverse proxy (nginx, Caddy, Traefik, ...) that
terminates TLS and forwards requests to it.

## The first user

There is currently no supported way to create/enable the first user. New
accounts are registered disabled with the `User` role, so they cannot log in
until an admin enables them, and there is no admin yet.

After registering a user, promote it directly in the database. With the sample
Compose stack:

```sh
docker compose exec db psql -U sync -d sync_server \
  -c "update users set enabled = true, role = 'Admin' where id = 1;"
```

If the first user does not have `id = 1`, target it by username instead:

```sql
update users set enabled = true, role = 'Admin' where username = 'your_username';
```

`role` is the `userrole` Postgres enum, and `'Admin'` is case-sensitive.

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
