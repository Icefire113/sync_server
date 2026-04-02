# Overview

## Routes

`GET /version` -> the version running

### Auth

`POST /api/auth/create_user` -> create a user

`POST /api/auth/login` -> log a user in, returns a token

`POST /api/auth/logout` -> log a user out

### Discriminators

`GET /api/discrim/get_all` -> gets all of a users discriminators (authenticated)

`GET /api/discrim/get_last_updated/:discriminator` -> gets the last update time for a specific discriminator for a user (authenticated)

`GET /api/discrim/get_last_updated/` -> gets the last update time for all specific discriminator for a user (authenticated)

`POST /api/discrim/create` -> create a discriminator for a user (authenticated)

`POST /api/discrim/create_many` -> create many discriminators for a user (authenticated)

`DELETE /api/discrim/:discrim` -> delete a discriminator for a user (authenticated)

`POST /api/discrim/update` -> update a discriminator for a user (authenticated)

## Authentication

All restricted endpoints must pass a valid login token via a `Authorization` header
