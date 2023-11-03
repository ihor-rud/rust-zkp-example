# Description

This is an example implementation of server and client applications that use ZKP for authentication. Server code can be found in [src/server](src/server). Client code can be found in [src/client](src/client).

# Execution and running

Run `docker compose up` to build and run code in docker. It should print something like that:

```shell
client  | Authentication verified: session_id = session/OF1aZ78YksCuWgkx
```

# Server code file structure

Server code is structured in the way to make bussines logic independent form API and persistence layers.

- [src/server/src/api](src/server/src/api) - api related code such as handlers for RPC methods
- [src/server/src/domain](src/server/src/domain) - contains bussines logic of the app
- [src/server/src/gateways](src/server/src/gateways) - concrete implementation of persistence layer
- [src/server/src/main.rs](src/server/src/main.rs) - execution entry point

# ZKP

Code related to ZKP can be found in [src/server/src/domain/auth/verifier.rs](src/server/src/domain/auth/verifier.rs) and [src/client/src/api/grpc.rs](src/client/src/api/grpc.rs).
