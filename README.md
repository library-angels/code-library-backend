# CODE Library backend

The aim of this project is to provide a library system for the management
and borrowing of books. This repository contains the backend components of the system.


## 1. Repository structure

The backend consists of five microservices (**currently WIP**). The source code of each microservice resides in its own sub-directory, which is named after the microservice.

The table below provides an overview of the services.

| Service | Description |
| --- | --- |
| identity | Authentication and authorization functionalities. |
| book | Book management functionalities. |
| borrow | Borrow management functionalities. |
| notification | Notification functionalities (e.g., email, Slack). |
| api | REST API for user access. |

For more detailed information about each service, refer to the README file in the service directory.


## 2. Building

There are two supported ways to build the services, either locally (see Chapter: Building from Source) or
inside a Docker container (see Chapter: Building with Docker).


### 2.1 Building from Source

To build from source, please clone this repository and make sure you have a working
installation of the Rust compiler and Cargo package manager, minimum version is `1.41`. Please refer to the Rust
[website](https://www.rust-lang.org/tools/install), to learn more about how to download and install Rust.
Furthermore, there are required external dependencies (see Chapter: 2.2.1 External Dependencies).

Each command has to be executed in the repository root directory.

Execute `cargo build` to build all services or `cargo build --bin <service>` command to build a specific one, while `<service>` is the name of the respective service.

#### 2.1.1 External Dependencies

The table below, provides an overview of the required build dependencies. 

| Dependency name | Minimum version | Description |
| --- | --- | --- |
| libssl | 1.1.1 | OpenSSL TLS library |
| libcrypto | 1.1.1 | OpenSSL cryptographic library |
| libpq | 9.5 | Postgres client library |

Please refer to the individual dependency documentation, to learn more about how it's made available on your platform.


### 2.2 Building with Docker

To build a Docker image, please make sure you have obtained a copy of the repository and a working
installation of Docker. Please refer to the Docker [website](https://docs.docker.com/), to learn more about how to download and install Docker.

Each command has to be executed in the repository root directory.

Execute `docker build -f <service>/Dockerfile -t code-library-<service> .` command, while `<service>` is the name of the respective service to build.


## 3. Development

All development has to happen inside of separate branches. Every branch name is prefixed with its purpose of creation.
The used prefixes are the same as the Github labels for pull requests.
The `master` branch is protected and requires pull request and an approval of at least one reviewer.
The used merge method is squash and merge.
