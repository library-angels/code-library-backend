# CODE Library backend - NOTIFICATION

This is the NOTIFICATION service of the CODE Library backend.
The purpose of this service is to provide methods for notification functionalities.
It exposes a JSON-RPC-API, which is used by other backend services to utilize its services.

# 1. Service Execution

There are two supported ways to run the service, either locally or inside a Docker container.
Please refer to the repositores root README file to learn more about how to build the service for the respective target.

## 1.1 Service Configuration

The service can be configured by environment variables.

Depending on if the service is run locally or inside a Docker container the specification of them changes.
For running it locally, please refer to your shells documentation on how to specify them.
For Docker, the environment variables can be passed to `docker run` with the `-e` argument.

Variables with a value in the default column are only required to set if the value needs to be changed.

| Variable name    | Default          | Data type         | Description                                                       |
| ---------------- | ---------------- | ------------------| ----------------------------------------------------------------- |
| `SERVICE_SOCKET` | `127.0.0.1:8080` | IP socket address | IP socket address on which the listens for RPC requests.          |
| `DB_SOCKET`      | `127.0.0.1:5432` | Socket address    | Socket address on which the service expects the database service. |
| `DB_NAME`        | `postgres`       | String            | Name of the database on the database server.                      |
| `DB_USERNAME`    | `postgres`       | String            | Name of the database username on the database server.             |
| `DB_PASSWORD`    | `password`       | String            | Password of the database user on the database server.             |
