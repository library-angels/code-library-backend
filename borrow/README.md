# CODE Library backend - BORROW

This is the BORROW service of the CODE Library backend.
The purpose of this service is to provide methods for book borrow requests and the management of associated processes.
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

| Variable name              | Default   | Data type               | Description                                                   |
| -------------------------- | --------- | ----------------------- | ------------------------------------------------------------- |
| DB_HOST_IP                 | 127.0.0.1 | IPv4 address            | IP address on which the service expects the database service. |
| DB_HOST_PORT               | 5432      | Unsigned integer 16-bit | IP port on which the service expects the database service.    |
| DB_NAME                    | postgres  | String                  | Name of the database on the database server.                  |
| DB_USER                    | postgres  | String                  | Name of the database user on the database server.             |
| DB_SECRET                  | password  | String                  | Secret of the database user on the database server.           |
| RPC_HOST_IP                | 127.0.0.1 | IPv4 address            | IP address on which the service listens for RPC requests.     |
| RPC_HOST_PORT              | 8082      | Unsigned integer 16-bit | IP port on which the service listens for RPC requests.        |
