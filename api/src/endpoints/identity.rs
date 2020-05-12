use bytes::buf::BufExt;
use hyper::{header::CONTENT_TYPE, Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use jsonwebtoken::{
    dangerous_unsafe_decode, decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use log::{debug, error};
use serde::{Deserialize, Serialize};

use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use warp::http::{Response, StatusCode};

#[derive(Serialize)]
struct OauthClientIdentifier {
    client_identifier: String,
}

#[derive(Deserialize)]
pub struct OauthAuthorizationCode {
    code: String,
}

impl OauthAuthorizationCode {
    pub fn is_valid(&self) -> bool {
        if self.code.is_empty() || self.code.len() > 256 {
            return false;
        }

        for i in self.code.chars() {
            if !(i.is_ascii_alphanumeric() || i == '-' || i == '/' || i == '_') {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[derive(Serialize)]
pub struct OauthTokenRequest {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

impl OauthTokenRequest {
    pub async fn exchange_code(&self) -> Result<OauthTokenSet, OauthError> {
        let https = HttpsConnector::new();
        let https_client = Client::builder().build::<_, hyper::Body>(https);

        let response = https_client
            .get(Uri::from_static(
                "https://accounts.google.com/.well-known/openid-configuration",
            ))
            .await
            .unwrap();
        if response.status() != 200 {
            error!("Could not reach discovery document");
            return Err(OauthError::DiscoveryDocumentNotReachable);
        }
        let body = hyper::body::aggregate(response).await.unwrap();
        let discovery_document: Result<OauthDiscoveryDocument, serde_json::error::Error> =
            serde_json::from_reader(body.reader());
        let discovery_document = match discovery_document {
            Ok(val) => val,
            Err(_) => {
                error!("Could not deserialize discovery document");
                return Err(OauthError::DiscoveryDocumentDeserializeError);
            }
        };

        let token_request = Request::post(discovery_document.token_endpoint)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(serde_urlencoded::to_string(self).unwrap()))
            .unwrap();

        let token_set_response = https_client.request(token_request).await.unwrap();
        if token_set_response.status() != 200 {
            debug!("Could not exchange authorization code");
            return Err(OauthError::TokenExchangeFailed);
        }

        let token_set_response_body = hyper::body::aggregate(token_set_response).await.unwrap();
        let token_set: Result<OauthTokenSet, serde_json::error::Error> =
            serde_json::from_reader(token_set_response_body.reader());
        match token_set {
            Ok(val) => Ok(val),
            Err(_) => {
                error!("Could not deserialize token set");
                Err(OauthError::TokenSetDeserializeError)
            }
        }
    }
}

pub enum OauthError {
    DiscoveryDocumentNotReachable,
    DiscoveryDocumentDeserializeError,
    TokenExchangeFailed,
    TokenSetDeserializeError,
    IdTokenDeserializeError,
}

#[derive(Deserialize)]
pub struct OauthDiscoveryDocument {
    //    issuer: String,
    token_endpoint: String,
    //    jwks_uri: String,
}

#[derive(Deserialize)]
pub struct OauthTokenSet {
    //    access_token: String,
    //    expires_in: u32,
    //    scope: String,
    //    token_type: String,
    id_token: String,
}

impl OauthTokenSet {
    pub fn id_token(&self) -> Result<OauthIdToken, OauthError> {
        let id_token = dangerous_unsafe_decode::<OauthIdToken>(&self.id_token);
        match id_token {
            Ok(val) => {
                if val.claims.is_valid() {
                    Ok(val.claims)
                } else {
                    Err(OauthError::IdTokenDeserializeError)
                }
            }
            Err(_) => Err(OauthError::IdTokenDeserializeError),
        }
    }
}

#[derive(Deserialize)]
pub struct OauthIdToken {
    iss: String,
    //    aud: String,
    sub: String,
    //    iat: u64,
    //    exp: u64,
    email: String,
    name: String,
    given_name: String,
    family_name: String,
    hd: String,
    picture: String,
}

impl OauthIdToken {
    fn is_valid(&self) -> bool {
        self.iss == "https://accounts.google.com"
            || self.iss == "accounts.google.com" && self.hd == "code.berlin"
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    sub: String,
    email: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    iat: u64,
    exp: u64,
}

impl Jwt {
    pub fn new(
        sub: String,
        email: String,
        name: String,
        given_name: String,
        family_name: String,
        picture: String,
        jwt_validity: u64,
    ) -> Jwt {
        Jwt {
            sub,
            email,
            name,
            given_name,
            family_name,
            picture,
            iat: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            exp: SystemTime::now()
                .checked_add(Duration::from_secs(jwt_validity))
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn encode(&self, secret: String) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }
}

#[derive(Serialize)]
pub struct JwtSet {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshToken {
    token: String,
}

pub async fn users_index(_query: String) -> Result<impl warp::Reply, Infallible> {
    Ok("users_index".to_string())
}

pub async fn users_id(user_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("users id: {}", user_id))
}

pub async fn roles_index(_query: String) -> Result<impl warp::Reply, Infallible> {
    Ok("roles index".to_string())
}

pub async fn roles_id(role_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("roles id: {}", role_id))
}

pub async fn oauth_client_identifier(
    config: Arc<Box<super::super::config::Config>>,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&OauthClientIdentifier {
        client_identifier: config.oauth_client_identifier.clone(),
    }))
}

pub async fn oauth_authorization_code_exchange(
    config: Arc<Box<super::super::config::Config>>,
    body: OauthAuthorizationCode,
) -> Result<Response<Body>, Infallible> {
    if !body.is_valid() {
        let error_message = serde_json::to_string(&ErrorMessage {
            message: "Invalid query string".to_string(),
        })
        .unwrap();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(error_message))
            .unwrap());
    }

    let token_request = OauthTokenRequest {
        code: body.code,
        client_id: config.oauth_client_identifier.clone(),
        client_secret: config.oauth_client_secret.clone(),
        redirect_uri: config.oauth_client_redirect.clone(),
        grant_type: String::from("authorization_code"),
    };

    let token_set = match token_request.exchange_code().await {
        Ok(val) => val,
        Err(_) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                message: "Failed to exchange authorization code".to_string(),
            })
            .unwrap();
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(error_message))
                .unwrap());
        }
    };

    let id_token = match token_set.id_token() {
        Ok(val) => val,
        Err(_) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                message: "Couldn't deserialize Google id_token".to_string(),
            })
            .unwrap();
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(error_message))
                .unwrap());
        }
    };

    let jwt_set = JwtSet {
        access_token: Jwt::new(
            id_token.sub.clone(),
            id_token.email.clone(),
            id_token.name.clone(),
            id_token.given_name.clone(),
            id_token.family_name.clone(),
            id_token.picture.clone(),
            3600,
        )
        .encode(config.jwt_secret.clone()),
        refresh_token: Jwt::new(
            id_token.sub.clone(),
            id_token.email.clone(),
            id_token.name.clone(),
            id_token.given_name.clone(),
            id_token.family_name.clone(),
            id_token.picture,
            86400,
        )
        .encode(config.jwt_secret.clone()),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&jwt_set).unwrap()))
        .unwrap())
}

pub async fn jwt_info() -> Result<impl warp::Reply, Infallible> {
    Ok("jwt_info".to_string())
}

pub async fn jwt_refresh(
    config: Arc<Box<super::super::config::Config>>,
    body: RefreshToken,
) -> Result<impl warp::Reply, Infallible> {
    let refresh_token = match decode::<Jwt>(
        &body.token,
        &DecodingKey::from_secret(config.jwt_secret.to_owned().as_ref()),
        &Validation::default(),
    ) {
        Ok(val) => val,
        Err(_) => {
            let error_message = serde_json::to_string(&ErrorMessage {
                message: "Invalid refresh token".to_string(),
            })
            .unwrap();
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(error_message))
                .unwrap());
        }
    };

    let access_token = Jwt::new(
        refresh_token.claims.sub.to_owned(),
        refresh_token.claims.email.to_owned(),
        refresh_token.claims.name.to_owned(),
        refresh_token.claims.given_name.to_owned(),
        refresh_token.claims.family_name.to_owned(),
        refresh_token.claims.picture,
        3600,
    )
    .encode(config.jwt_secret.to_owned());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&access_token).unwrap()))
        .unwrap())
}
