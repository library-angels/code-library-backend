# CODE Library - Backend Architecture Documentation

## General Information

The backend uses microservices to separate its business logic into related parts.

**This document is subject to change.**

For more detailed information about each service, refer to the README file in the service directory.


```
                                                                               +--------------+
                                                  +--------------+             |              |
                                                  |              | +---------> | GOOGLE API   |
                                      +---------> | IDENTITY     |             |              |
                                      |           |              | +----+      +--------------+
                                      |           +--------------+      |
                                      |                                 |
                                      |           +--------------+      |
                                      |           |              |      |
                                      +      +--> | BOOK         | +----+
+---------------+        +--------------+    |    |              |      |      +--------------+
|               |        |              | +--+    +--------------+      |      |              |
| REVERSE-PROXY | +----> | API          |                               +----> | DATABASE     |
|               |        |              | +--+    +--------------+      |      |              |
+---------------+        +--------------+    |    |              |      |      +--------------+
                                      +      +--> | BORROW       | +----+
                                      |           |              |      |      +--------------+
                                      |           +--------------+      |      |              |
                                      |                   +             |  +-> | AWS SES API  |
                                      |                   |             |  |   |              |
                                      |                   v             |  |   +--------------+
                                      |           +--------------+      |  |
                                      |           |              | +----+  |   +--------------+
                                      +---------> | NOTIFICATION |         |   |              |
                                                  |              | +-------+-> | SLACK API    |
                                                  +--------------+             |              |
                                                                               +--------------+
```
*Backend schema, with communication, external API dependencies and deployment.*

## Service Structure

The backend consists of five microservices.
While the API service acts as an interface for clients, the IDENTITY, BOOK, BORROW and NOTIFICATION services implement the business logic.
The backend uses a relational database management system to store its business data.
Operations on the database are only performed by the logic implementing microservices.
All services operate on a single database.

|Service|Business Logic|External APIs|
|---|---|---|
|API|REST API for client access<br>Forward requests, requiring backend logic|*No external API required*|
|IDENTITY|OAuth 2.0 authentication<br>Session management<br>User management<br>Role management|Google OAuth 2.0 and OpenID|
|BOOK|Book management<br>Book copy management<br>Global book attribute management<br>Book listing|*No external API required*|
|BORROW|Book copy availability listing<br>Borrow process initiation<br>Borrow ending<br>Borrow overdue management<br>Borrow reminders|*No external API required*|
|NOTIFICATION|Notification transfer to AWS SES (Email)<br>Notification transfer to Slack|AWS SES and Slack|


### Service Discovery

Service discovery is currently not part of the architecture. Services are deployed with a static configuration.


## Communication

The backend uses RPC with serialized Rust data structures for internal communication.
External APIs are utilized through HTTP.
The API microservice exposes a HTTP based REST API to offer the backend services to clients.


### Encryption and Authentication

Backend communication is currently unauthenticated and not encrypted.


## Deployment

The API microservice does not implement encryption to client connections.
Thus, the deployment requires a reverse proxy, which implements TLS support.
