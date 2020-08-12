# CODE Library - Public REST API Documentation

## General Information
---

This document describes the public REST API, of the api microservice.
The API uses HTTP content type `application/json`, for requests and responses.
If an endpoint requires authentication, a session token must be sent in the `Authorization` HTTP header.
Session tokens can be obtained from the `/identity/oauth/authentication` endpoint, by sending a valid OAuth 2.0 authorization code.

The API uses HTTP status codes to indicate the success of a request.
Every response with a status code unequal to 2xx can be considered failed.
Failed requests contain a `error` field in the response body, with information about the specific error.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

## Endpoints
---

### GET - /book
---

Returns a list of book objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|
|sort|No|String|alpha_asc|Sorting of results, possible values are `alpha_asc`, `alpha_desc`, `releasedate_asc`, `releasedate_desc`, `codeidentifier_asc` and `codeidentifier_desc`|
|designation|No|Array(string)|*|
|modules|No|Array(String)|*|
|tags|No|Array(String)|*|
|series|No|Array(String)|*|
|publisher|No|Array(String|*|
|search_fields|No|String|all|Search fields `all`, `title` and `author`|
|search_keywords|No|Array(String)|None|Search term for the search fields|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|

### POST - /book
---

Creates a book object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|Description|
|---|---|---|
|isbn_13|String|International Standard Book Number|
|issn|String|International Standard Serial Number|
|title|String|Title|
|subtitle|String|Subtitle|
|description|String|Description|
|edition|String|Edition|
|release_year|Integer|Release year|
|pages|Integer|Pages|
|code_identifier|String|CODE book identifier (without copy id)|
|publisher|String|Publisher|
|designation|String|Designation|
|series|String|Series|
|language|String|Language|
|physical_size|String|Physical size|

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/{book_id}
---

Returns a book object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### POST - /book/{book_id}
---

Updates a book object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|Description|
|---|---|---|
|isbn_13|String|International Standard Book Number|
|issn|String|International Standard Serial Number|
|title|String|Title|
|subtitle|String|Subtitle|
|description|String|Description|
|edition|String|Edition|
|release_year|Integer|Release year|
|pages|Integer|Pages|
|code_identifier|String|CODE book identifier (without copy id)|
|publisher|String|Publisher|
|designation|String|Designation|
|series|String|Series|
|language|String|Language|
|physical_size|String|Physical size|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Resource does not exist|
|500|Internal error|


### DELETE - /book/{book_id}
---

Deletes a book object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|204|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/authors
---

Returns a list of author objects for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/copies
---

Returns a list of copy objects for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### POST - /book/{book_id}/copies
---

Creates a copy objects for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|
|---|---|
|status|String|

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/copies/{copy_id}
---

Returns a copy object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|
|copy_id|Integer|Id of a copy|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### POST - /book/{book_id}/copies/{copy_id}
---

Updates a copy object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|
|copy_id|Integer|Id of a copy|

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|
|---|---|
|status|String|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### DELETE - /book/{book_id}/copies/{copy_id}
---

Deletes a copy object of a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Manager

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|
|copy_id|Integer|Id of a copy|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|204|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/designations
---

Returns the designation object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/editors
---

Returns a list of editor objects for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/languages
---

Returns the language object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/physical_sizes
---

Returns the physical size object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/publishers
---

Returns the publisher object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/series
---

Returns the series object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/subject_areas
---

Returns the subject area object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/{book_id}/tags
---

Returns the tag object for a book.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|book_id|Integer|Id of a book|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /book/authors
---

Returns a list of author objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/designations
---

Returns a list of designation objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/editors
---

Returns a list of editor objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/languages
---

Returns a list of language objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/physical_sizes
---

Returns a list of physical size objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/publishers
---

Returns a list of publisher objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/series
---

Returns a list of series objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/subject_areas
---

Returns a list of subject area objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /book/tags
---

Returns a list of tag objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /borrow/notifications
---

Returns a list of notification objects, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### POST - /borrow/notifications
---

Creates a notification object, for the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|Description|
|---|---|---|
|copy_id|Integer|Id of a book copy|

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /borrow/notification/{notification_id}
---

Returns a notification object, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|notification_id|Integer|Id of a notification|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### DELETE - /borrow/notifications/{notification_id}
---

Deletes a notification object, for the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|
|notification_id|Integer|Id of a notification|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|204|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /borrow/active
---

Returns a list of active borrows, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### POST - /borrow/active
---

Creates an active borrow object, for the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|Description|
|---|---|---|
|copy_id|Integer|Id of a book copy|
|duration|Integer|Borrow duration in days|

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /borrow/active/{borrow_id}
---

Returns a active borrow object, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|borrow_id|Integer|Id of a borrow|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### POST - /borrow/active/{borrow_id}/return
---

Updates an active borrow object (returns borrow), for the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|borrow_id|Integer|Id of a borrow|

**Query Parameters**

*No query parameters*

**Request Body**

*No request body*

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### POST - /borrow/active/{borrow_id}/return/confirm
---

Updates an active borrow object (confirms return), for the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|borrow_id|Integer|Id of a borrow|

**Query Parameters**

*No query parameters*

**Request Body**

*No request body*

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /borrow/history
---

Returns a list of historic borrow objects, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /borrow/history/{borrow_id}
---

Returns a historic borrow object, of the authenticated user.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|borrow_id|Integer|Id of a borrow|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|400|Request malformed|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Requested object not found|
|500|Internal error|


### GET - /identity/oauth/client_identifier
---

Returns the OAuth 2.0 client identifier.

**Authentication required:** No<br>
**Authorization level (minimum):** *No authorization level*

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|500|Internal error|


### POST - /identity/oauth/authentication
---

Creates a user account and returns a session token.

**Authentication required:** No<br>
**Authorization level (minimum):** *No authorization level*

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Request Body**

|Key|Data Type|Description|
|---|---|---|
|code|String|OAuth 2.0 authorization code|

**Responses**

|HTTP Code|Description|
|---|---|
|201|Request successful|
|400|Request malformed|
|500|Internal error|


### GET - /identity/roles
---

Returns a list of roles.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Admin

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /identity/roles/{role_id}
---

Returns a role object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Admin

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|role_id|Integer|Id of a role|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Resource not found|
|500|Internal error|


### GET - /identity/users
---

Returns a list of user objects.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Admin

**Path Parameters**

*No path parameters*

**Query Parameters**

|Parameter|Required|Data Type|Default Value|Description|
|---|---|---|---|---|
|offset|No|Integer|0|Offset to first object|
|limit|No|Integer|10|Quantity of returned objects|

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|


### GET - /identity/users/{user_id}
---

Returns an user object.

**Authentication required:** Yes<br>
**Authorization level (minimum):** Admin

**Path Parameters**

|Parameter|Data Type|Description|
|---|---|---|
|user_id|Integer|Id of an user|

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|404|Resource not found|
|500|Internal error|


### GET - /identity/session/info
---

Returns information about the current session token.

**Authentication required:** Yes<br>
**Authorization level (minimum):** User

**Path Parameters**

*No path parameters*

**Query Parameters**

*No query parameters*

**Responses**

|HTTP Code|Description|
|---|---|
|200|Request successful|
|401|Client is not authenticated|
|403|Client is not allowed to see the resource|
|500|Internal error|
