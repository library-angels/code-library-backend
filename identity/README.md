# CODE Library backend - IDENTITY

This is the IDENTITY service of the CODE Library backend.
The task of this service is to provide methods for user authentication, authorization and user/role management.
It exposes a RPC API with serialized Rust data structures, which is used by other backend services to utilize its service methods.


## Configuration

The service requires the configuration submitted through environment variables or a `.env` configuration file.

### Environment Variables

The following variables are currently accepted.

|Variable Name|Data Type|Default Value|Description|
|---|---|---|---|
|DB_HOST_IP|String (Ipv4 address)||Database service IP|
|DB_PORT|Integer|5432|Database service port|
|DB_NAME|String||Database name|
|DB_USER|String||Database username|
|DB_SECRET|String||Database username password|
|RPC_HOST_IP|String (IPv4 address)|127.0.0.1|RPC interface IP|
|RPC_HOST_PORT|Integer|8080|RPC service port|
|OAUTH_CLIENT_IDENTIFIER|String||OAuth 2.0 client identifier|
|OAUTH_CLIENT_SECRET|String||OAuth 2.0 client secret|
|JWT_SECRET|String||Secret for signing JWTs|

## RPC Methods

The exposed RPC methods and data structures are listed in the source file `src/rpc/service.rs`.
