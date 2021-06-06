# CODE Library backend

The aim of this project is to provide a library system for the management
and borrowing of books. This repository contains the backend components of the system.


## 1. Repository structure

The backend consists of five microservices and a helpers library. The source code of each microservice resides in its own sub-directory, which is named after the microservice.

The table below provides an overview of the services.

| Service                           | Description                                        |
| --------------------------------- | -------------------------------------------------- |
| [`api`](./api/)                   | REST API for user access.                          |
| [`book`](./book/)                 | Book management functionalities.                   |
| [`borrow`](./borrow/)             | Borrow management functionalities.                 |
| [`identity`](./identity/)         | Authentication and authorization functionalities.  |
| [`notification`](./notification/) | Notification functionalities (e.g., email, Slack). |

The helpers library resides in [`helpers/`](./helpers/).

For more detailed information about how the services work together, refer to [`ARCHITECTURE.md`](./ARCHITECTURE.md).
Specific implementation details about an individual service and how to run it can be found in the `README.md`, of the respective service directory.

For more details about our cyber security and privacy efforts please refer to [`SECURITY.md`](./SECURITY.md).

## 2. Building

There are two supported ways to build the services, either locally (see [Chapter: 2.1 Building from Source](#21-building-from-source)) or
inside a Docker container (see [Chapter: 2.2 Building with Docker](#22-building-with-docker)).


### 2.1 Building from Source

To build from source, please clone this repository and make sure you have a working
installation of the Rust compiler and Cargo package manager, minimum version is `1.41`. Please refer to the Rust
[website](https://www.rust-lang.org/tools/install), to learn more about how to download and install Rust.
Furthermore, there are required external dependencies (see [Chapter: 2.1.1 External Dependencies](#211-external-dependencies)).

Each command has to be executed in the repository root directory.

Execute `cargo build` to build all services or `cargo build --bin=<service>` command to build a specific one, while `<service>` is the name of the respective service. The helpers library will be automatically included, if necessary.
To only build the helpers library, execute `cargo build --lib=helpers`.

#### 2.1.1 External Dependencies

The table below, provides an overview of the required build dependencies.

| Dependency name | Minimum version | Description                   |
| --------------- | --------------- | ----------------------------- |
| libssl          | 1.1.1           | OpenSSL TLS library           |
| libcrypto       | 1.1.1           | OpenSSL cryptographic library |
| libpq           | 9.5             | Postgres client library       |

Please refer to the individual dependency documentation, to learn more about how it is made available on your platform.


### 2.2 Building with Docker

To build a Docker image, please make sure you have obtained a copy of the repository and a working
installation of Docker. Please refer to the Docker [website](https://docs.docker.com/), to learn more about how to download and install Docker.

Each command has to be executed in the repository root directory.

Execute `docker build -f <service>/Dockerfile -t code-library-<service> .` command, while `<service>` is the name of the respective service to build.


## 3. Development

All development has to happen inside of separate branches. Every branch name is prefixed with its purpose of creation.
The used prefixes are the same as the Github labels for pull requests.
The `master` branch is protected and requires pull request and an approval of at least one reviewer.
The used merge method is rebasing.

## 4. Contributors

| User                                  | Work done                                                                                                                |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| [`alexeybi`](github.com/alexeybi)     | Database access, book endpoints, tests, CI/CD, GKE                                                                       |
| [`code-mm`](github.com/code-mm)       | Database schema, endpoint setup, logging, CORS, RPC setup, identity endpoints, OAuth 2.0, JWT, tests, CI/CD, refactoring |
| [`urhengulas`](github.com/urhengulas) | Database schema, book endpoints, refactoring, CI/CD                                                                      |
