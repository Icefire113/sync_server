# sync_server

sync_server is the server side of [Config Share](https://github.com/Icefire113/config_share),
a tool for keeping configuration files in sync across several machines. You
track a file once on each machine, and the client uses this server as the
shared, authoritative copy of the contents and their current state.

The server stores file contents and records what each file should look like,
while a client on every machine compares that state with its local copy and
transfers only what has changed. It does not watch folders or push
notifications; clients check in on their own schedule, so machines that are
only intermittently online still catch up the next time they connect.

## What it does

- **Stores file contents centrally.** Configuration files live on the server,
  in either local disk storage or an S3-compatible bucket, so every machine can
  retrieve the same authoritative copy.
- **Tracks the state of each file.** Each file has an identity, a name, a
  content hash, the machine that last changed it, and timestamps. That state is
  what clients use to decide what to do next.
- **Transfers only differences.** Clients compare content hashes instead of
  file timestamps, so a file is only moved when its contents actually differ.
- **Protects against lost updates.** An upload based on an older version is
  rejected rather than silently overwriting a newer change made from another
  machine. The client can then re-read the file and retry.
- **Propagates deletions.** A deleted file disappears from normal listings, so
  other machines remove their copies, while the record is kept on the server
  for bookkeeping.
- **Keeps users separate.** Every account has its own files and its own area
  of the storage backend; one user cannot see or address another user's data.
- **Controls access.** Accounts have roles, new accounts start disabled and
  must be enabled by an administrator, and each device gets its own revocable
  access token.
- **Fits different deployments.** Storage can be local or S3-compatible, the
  maximum accepted file size is configurable, and schema upgrades are applied
  automatically when the server starts.
- **Stays observable.** A small healthcheck reports the running version, and
  optional response compression reduces bandwidth for clients that support it.

## How syncing works

1. A client registers each file it wants to track, supplying the file's name
   and current content hash.
2. On each check, the client asks the server for the current state of its
   files and compares the returned hashes with its local copies.
3. Files that are missing or out of date are downloaded. Local changes are
   uploaded along with the version the client started from; if another machine
   got there first, the upload is refused and the client retries from the new
   state.
4. Files deleted on one machine are deleted on the server, and therefore
   disappear from the other machines on their next check.

## Quick start

Requirements: Rust and PostgreSQL.

```sh
cp .env.example .env    # set DATABASE_URL
mkdir storage           # the local backend requires this directory to exist
cargo run
```

The server listens on `HOST`/`PORT` (`0.0.0.0:3000` by default). Alternatively,
run the sample stack with Docker:

```sh
docker compose up --build
```

After that, see [The first user](#the-first-user) to create an admin account,
then install the [config_share](https://github.com/Icefire113/config_share)
client on each machine and point it at the server.

## Deployment

sync_server only exposes a plain HTTP listener (`HOST`/`PORT`, default
`0.0.0.0:3000`). It does not terminate TLS or manage certificates itself.

If the server is reachable from outside a trusted network, it is strongly
recommended to run it behind a reverse proxy (nginx, Caddy, Traefik, ...) that
terminates TLS and forwards requests to it.

## The first user

There is currently no supported way to create/enable the first user. New
accounts are registered disabled with the `User` role, so they cannot use the
service until an admin enables them, and there is no admin yet.

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

## Configuration

On first start the server creates `config.json` in its working directory. It
sets the maximum accepted file size (default 512 KiB), whether HTTP requests
are logged, and which storage backend is used. The file is gitignored and is
read when the server starts.

`max_file_size` accepts a byte count or a human-readable value (`b`, `k`, `m`,
`g`, case-insensitive; `k`/`m`/`g` are KiB/MiB/GiB).

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

The directory must already exist when the server starts.

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

At startup the server checks that the bucket exists and that it can write,
read, and delete an object in it. Credentials are read from the standard AWS
environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`,
`AWS_REGION`).

## Features

### `response-compression`

Enables compression of response bodies using one of the following encodings:

- brotli
- gzip
- zstd
