# CODE Library backend - IDENTITY

This is the IDENTITY service of the CODE Library backend.
The purpose of this service is to provide methods for user authentication, authorization and user/role management.
It exposes a JSON-RPC-API, which is used by other backend services to utilize its services.

# 1. Service Execution

There are two supported ways to run the service, either locally or inside a Docker container.
Please refer to the repositores root README file to learn more about how to build the service for the respective target.

## 1.1 Service Configuration

The service can be configured by environment variables.

Depending on if the service is run locally or inside a Docker container the specification of them changes.
For running it locally, please refer to your shells documentation on how to specify them.
For Docker, the environment variables can be passed to `docker run` with the `-e` argument.

For configuration during development the use of an environment configuration file is supported.
This file has the name `.env` and must contain the environment variables for configuration.
The service will automatically look for such a file in the work directory and apply it.
The configuration from the `.env` file will superseed the configuration specified on the command line.

Variables with a value in the default column are only required to set if the value needs to be changed.

| Variable name             | Default          | Data type         | Description                                                       |
| ------------------------- | ---------------- | ------------------| ----------------------------------------------------------------- |
| `SERVICE_SOCKET`          | `127.0.0.1:8080` | IP socket address | IP socket address on which the listens for RPC requests.          |
| `DB_SOCKET`               | `127.0.0.1:5432` | Socket address    | Socket address on which the service expects the database service. |
| `DB_NAME`                 | `postgres`       | String            | Name of the database on the database server.                      |
| `DB_USERNAME`             | `postgres`       | String            | Name of the database username on the database server.             |
| `DB_PASSWORD`             | `password`       | String            | Password of the database user on the database server.             |
| `OAUTH_CLIENT_IDENTIFIER` | No default       | String            | OAuth 2.0 client identifier for OAuth authentication.             |
| `OAUTH_CLIENT_SECRET`     | No default       | String            | OAuth 2.0 client secret for OAuth authentication.                 |
| `JWT_SECRET`              | No default       | String            | Secret for signing session token JWTs.                            |
