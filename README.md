# CODE Library backend

This repository contains the backend components of the library system at the CODE University of 
Applied Sciences, Berlin. The aim of this project is, to provide a library system for the management 
and borrowing of books. This includes notifications to users about available and overdue books.

## System Architecture

The backend consists of five microservices. Each microservice resides in its own sub-directory.
The directories are named accordingly to the name of the microservice.
Please refer to the table below, to get more details.

| Service | Description |
| --- | --- |
| identity | Implements the authentication and authorization functionalities |
| book | Implements the book management functionalities |
| borrow | Implements the borrow management functionalities |
| notification | Implements the user notification functionalities |
| api | Implements the REST API for user access |

The services communicate through remote-procedure-calls (RPC) among each other. However, only the
`api` microservice has a strong dependency to the other services.

## Building

This project uses the Cargo package manager from the Rust ecosystem.
The minimum version for the Rust compiler and Cargo package manager is `1.41`.
Please refer to the Rust [website](https://www.rust-lang.org/tools/install), to learn more about how to download and install Rust.

Furthermore, there are external dependencies. They are listed in the chapter "External Dependencies".

### External Dependencies

The following table provides a list of mandatory dependencies. Currently, they are required for all microservices.
Please refer to your operating system and the documentation of the individual dependency, to learn more about how to obtain and build them on your platform.

| Dependency name | Minimum version | Description |
| --- | --- | --- |
| libssl | 1.1.1 | OpenSSL TLS library and header files |
| libcrypto | 1.1.1 | OpenSSL cryptographic library and header files |
| libpq | 9.5 | Postgres client library and header files |

### Build from Source

To build from source, please make sure you have obtained a copy of the repository.
Each command has to be executed relative to the repository root directory.

To build all microservices execute the `cargo build` command.
An individual service can be build with `cargo build --bin <service>` command,
which requires the substitution of `<service>` with the respective service name.

### Docker

To build from source, please make sure you have obtained a copy of the repository.
Each command has to be executed relative to the repository root directory.

To build a Docker image of a microservice, execute `docker build -f <service>/Dockerfile -t code-library-<service> .`,
which requires the substitution of `<service>` with the respective service name.
