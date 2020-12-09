# Security concept 🔒

Developing the CODE library we try to adopt security- and privacy-by-design to our best knowledge and ability.

<details>
<summary><b>Table of contents</b></summary>

- [Goals](#goals)
- [Approach](#approach)
- [Personas](#personas)
	- [Debora](#debora)
	- [Eve](#eve)
- [Threat model](#threat-model)
	- [Access to ECR](#access-to-ecr)
	- [Access to infrastructure](#access-to-infrastructure)
	- [Broken Access Control](#broken-access-control)
	- [DoS Attack](#dos-attack)
	- [Extensive analyzing of personal usage data](#extensive-analyzing-of-personal-usage-data)
	- [Insufficient Logging and Monitoring](#insufficient-logging-and-monitoring)
	- [Security issues in dependencies](#security-issues-in-dependencies)
	- [Sensitive Data Exposure](#sensitive-data-exposure)
	- [SQL injection](#sql-injection)
	- [Stolen book](#stolen-book)
	- [Stolen JWT Session Token](#stolen-jwt-session-token)
	- [Stolen OAuth Access 2.0 Token](#stolen-oauth-access-20-token)
	- [Stolen OAuth Refresh 2.0 Token](#stolen-oauth-refresh-20-token)
	- [Too verbose error messages](#too-verbose-error-messages)
- [Foodnotes](#foodnotes)
</details>

## Goals
In securing our system we focus on three high-level goals:
* Availability
* Confidentially
* Integrity

## Approach
To improve the security of our system in a structured approach we combine two methods: _Threat modelling_[¹](#foodnotes) and _Abuser stories_[²](#foodnotes).

We try to analyze each threat from three different angles:
* How to prevent incidents from happening? (Protection)
* How to detect (past and present) incidents? (Detection)
* How to respond to incidents, because at some point they will happen? (Response)

Because of the physical nature of our library we also face threats of physical nature. We include some of them in this document, but mainly focus on cyber security threats.

Additionally we checked the [OWASP Top Ten](https://owasp.org/www-project-top-ten/) to avoid common security risks.

## Personas

### Debora
Debora is an external person and has no legitimate access to the library system. She can only access the frontend.

### Eve
Eve is a student with a `@code.berlin`-email address and can
* read all book data
* read and write her personal profile information
* borrow new books, as well as return them

## Threat model

### Access to ECR
* "Debora or Eve would like to push a malicious container image to ECR. _(e.g. by using vulnerability in GitHub Actions)_"
* Protection
	- [x] Token to push images
* Detection
	- [ ] Status messages in slack channel on-deployment
* Response
    * Manually delete potentially malicious images

### Access to infrastructure
* "Eve and Debora would like to directly access server (API, RPC services) and storage (Database)."
* Protection
    - [x] Encapsulate cluster in VPC (only frontend and API accessible from public internet via proxy)
    - [ ] Network policy which IP can access database
    - [x] Restrictive policies on proxy ([hsts](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security), [enforce tls 1.2 or higher](https://developer.mozilla.org/en-US/docs/Web/Security/Transport_Layer_Security))
* Detection
	- [ ] Include IP in logs on API, RPCs and database
* Response

### Broken Access Control
* "Eve would like to access data and functionality she should have no access to (data of other users, modify book data)."
* Protection
	- [x] Authorization for restricted endpoints
* Detection
	- [ ] Log all requests and database events
* Response
	* Inform users directly if personal data was exposed
    * Revert all modifiations she did based on the logs

### DoS Attack
* "Eve and Debora would like to make the serivce unavailable to other users, by creating a high load of requests."
* Protection
    - [ ] Autoscaling of resources
    - [ ] Blocking of IPs sending "garbage requests"
    - [ ] Rate limit API
    - [ ] Replication + Load balancing
* Detection
	- [ ] Monitor request volume and ressource usage
    	- [ ] Alert in case of unusual situation
* Response
	* Communicate situation to users (CODE Slack)
	* Increase rate limit
	* Block IP range of attacker

### Extensive analyzing of personal usage data
* "As an library manager or administrator I'd like to analyze the usage data to create interest profiles of the users."
* Protection
	- [ ] Pseudonomize or anonomize borrow data after book is returned
* Detection
    * Wistleblowing of working students
* Response
    * Townhall discussion

### Insufficient Logging and Monitoring
* "Eve and Debora want to be not discovered while attacking the system and leave litte to no traces of their malicious activities."
* Protection
	- [ ] Logging and monitoring on proxy, API, RPC, database
	- [ ] Aggregate logs to make them better analyzable
* Detection
* Response

### Security issues in dependencies
* "Eve and Debora would like to attack using a known vulnerability in a (transitive) dependency [, which is not closed due to outdated dependencies]."
* Protection
    - [x] Automatically check for vulnerabilites in dependencies
    - [x] Automatically update dependencies
* Detection
* Response
    * Update dependency of concern + check if vulnerability was exploited

### Sensitive Data Exposure
* "Debora would like to access non-public data and functionality."
* Protection
	- [x] Authentication for all endpoints (except the ones for authentication)
* Detection
* Response
	* Inform users directly if personal data was exposed
    * Revert all modifiations she did based on the logs

### SQL injection
* "Eve and Debora would like to execute malicious SQL code by sending it as part of a query."
* Protection
    - [x] Prevented by ORM, using prepared SQL statements ([see here](https://github.com/diesel-rs/diesel/issues/229))
* Detection
	- [ ] Regularly check database log for unusual activities
* Response
	* Inform users directly if personal data was exposed
    * Revert malicious SQL activity based on logs

### Stolen book
* "Eve would like to steal a book, by just taking it from the shelf and not submitting this to the system."
* Protection
    - [ ] System which enforces user to authenticate before taking a book (camera, automatic door, ...)
* Detection
	- [x] Regular inventory
* Response
	* Communicate the values of our community to the student body and suggest to bring books back

### Stolen JWT Session Token
* "Debora or Eve would like to steal the JWT Session Token from a legitimate user, manager or administrator."
* Protection
	- [x] Token automatically expires after 1 hour
* Detection
* Response
    - [ ] Deny list specific token
	- [ ] Invalidate ALL tokens by changing `JWT_SECRET`

### Stolen OAuth Access 2.0 Token
* "Debora or Eve would like to steal the OAuth Access Token from a legitimate user, manager or administrator."
* Protection
	- [x] Token automatically expires after 1 hour
* Detection
* Response
    - [ ] Deny list specific token
    - [ ] Invalidate token [with google API](https://developers.google.com/identity/protocols/oauth2/web-server#httprest_8)

### Stolen OAuth Refresh 2.0 Token
* "Debora or Eve would like to steal the OAuth Refresh Token from a legitimate user, manager or administrator."
* Protection
	- [ ] Encrypt token with pub-key + Only decrypt while token refreshing with private-key stored on hardware security module
* Detection
* Response
    - [ ] Deny list specific user
    - [ ] Invalidate tokens [with google API](https://developers.google.com/identity/protocols/oauth2/web-server#httprest_8)

### Too verbose error messages
* "Eve and Debora would like to get knowledge about the system (architecture, frameworks, ...) from HTTP responses."
* Protection
    - [x] Log extensive debug messages, but only send abstract responses to client
* Detection
* Response

## Foodnotes
1. "[R]isk-based approach to designing secure systems [...], based on identifying threats in order to develop mitigations to them." _([source](https://martinfowler.com/articles/agile-threat-modelling.html))_
1. "[A] user story from the point of view of a malicious adversary" _([source](https://rietta.com/blog/what-is-an-abuser-story-software/))_, used to "identify how attackers may abuse the system and jeopardize stakeholders' assets." _([source](https://handouts.secappdev.org/handouts/2008/abuser%20stories.pdf))_
