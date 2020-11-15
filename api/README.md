# CODE Library backend - API

This is the API service of the CODE Library backend.
The purpose of this service is to provide an interface for interacting with the backend services.
It exposes a REST-API, which is used by clients to access the services of the library.

For more detailed information about the REST API, refer to `REST-API.md`.

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
| HTTP_HOST_IP               | 127.0.0.1 | IPv4 address            | IP address on which the service listens for HTTP requests.    |
| HTTP_HOST_PORT             | 8080      | Unsigned integer 16-bit | IP port on which the service listens for HTTP requests.       |
| IDENTITY_SERVICE_HOST_IP   | 127.0.0.1 | IPv4 address            | IP address on which the service expects the identity service. |
| IDENTITY_SERVICE_HOST_PORT | 8081      | Unsigned integer 16-bit | IP port on which the service expects the identity service.    |
| BOOK_SERVICE_HOST_IP       | 127.0.0.1 | IPv4 address            | IP address on which the service expects the book service.     |
| BOOK_SERVICE_HOST_PORT     | 8082      | Unsigned integer 16-bit | IP port on which the service expects the identity service.    |
