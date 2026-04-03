# Overview

## Routes

`GET /version` -> the version running

### Auth

`POST /api/auth/create_user` -> create a user

`POST /api/auth/login` -> log a user in, returns a token

`POST /api/auth/logout` -> log a user out

### Discriminators

`GET /api/synced_file/get_all` -> gets all of a users discriminators (authenticated)

Query params:

```txt
user: String
```

`GET /api/synced_file/get_last_updated/:discriminator` -> gets the last update time for a specific discriminator for a user (authenticated)

Query params:

```txt
user: String
```

`GET /api/synced_file/get_last_updated/` -> gets the last update time for all specific discriminator for a user (authenticated)

Query params:

```txt
user: String
```

`POST /api/synced_file/create` -> create a discriminator for a user (authenticated)

Request Body:

```json
{
    ...
}
```

`POST /api/synced_file/create_many` -> create many discriminators for a user (authenticated)

Request Body:

```json
{
    "discriminators": [DiscrimCreateReq, ...]
}
```

`DELETE /api/synced_file/:discrim` -> delete a discriminator for a user (authenticated)

Query params:

```txt
user: String
```

`PATCH /api/synced_file/update` -> update a discriminator for a user (authenticated)

Query params:

```txt
user: String
discrim: String
```

Request Body:

```json
{
    "discriminator": String,
    ...
}
```

## Authentication

All restricted endpoints must pass a valid login token via a `Authorization` header
