# Sync Server API Types

Provides the Rust types for the Sync Server API, serde derives are gated behind 2 features `client` and `server`, you must enable them depending on which side of the API you are implementing. Note that some API endpoints only have request (or response) types, in this case, some parameters are provided via path params. If an endpoint does not have a response type, then the response type is `()` and it will likely return a 204.
