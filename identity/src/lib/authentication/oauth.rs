use hyper::{Body, Client, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use jsonwebtoken::dangerous_insecure_decode;
use serde::{Deserialize, Serialize, Serializer};

type ClientIdentifier = String;
type ClientSecret = String;

#[derive(Debug, PartialEq)]
pub enum Error {
    AuthorizationCodeLength,
    AuthorizationCodeInvalidCharacter,
    TokenRequestEndpointInvalidResponse,
    TokenRequestEndpointNotReachable,
    TokenRequestContentInvalid,
    TokenRequestDeserialization,
    IdTokenInvalid,
}

pub struct DiscoveryDocument {}

impl DiscoveryDocument {
    pub fn get_token_endpoint() -> Uri {
        "https://oauth2.googleapis.com/token".parse().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct AuthorizationCode {
    code: String,
}

impl Serialize for AuthorizationCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.code)
    }
}

impl AuthorizationCode {
    pub fn new(code: String) -> Result<Self, Error> {
        if code.is_empty() || code.len() > 256 {
            return Err(Error::AuthorizationCodeLength);
        }

        for i in code.chars() {
            if !(i.is_ascii_alphanumeric() || i == '-' || i == '/' || i == '_') {
                return Err(Error::AuthorizationCodeInvalidCharacter);
            }
        }
        Ok(Self { code })
    }
}

#[derive(Debug)]
pub enum RedirectUri {
    PostMessage,
}

impl Serialize for RedirectUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &*self {
            RedirectUri::PostMessage => serializer.serialize_str("postmessage"),
        }
    }
}

#[derive(Debug)]
pub enum GrantType {
    AuthorizationCode,
}

impl Serialize for GrantType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            GrantType::AuthorizationCode => serializer.serialize_str("authorization_code"),
        }
    }
}

#[derive(Serialize)]
pub struct TokenRequest {
    #[serde(rename = "client_id")]
    client_identifier: ClientIdentifier,
    client_secret: ClientSecret,
    #[serde(rename = "code")]
    authorization_code: AuthorizationCode,
    redirect_uri: RedirectUri,
    grant_type: GrantType,
}

impl TokenRequest {
    pub fn new(
        authorization_code: AuthorizationCode,
        client_identifier: ClientIdentifier,
        client_secret: ClientSecret,
        redirect_uri: RedirectUri,
        grant_type: GrantType,
    ) -> Self {
        TokenRequest {
            authorization_code,
            client_identifier,
            client_secret,
            redirect_uri,
            grant_type,
        }
    }

    pub async fn exchange_code(&self, token_endpoint: Uri) -> Result<TokenSet, Error> {
        let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

        let request = Request::post(token_endpoint)
            .body(Body::from(serde_json::to_vec(self).unwrap()))
            .unwrap();

        let response = match client.request(request).await {
            Ok(val) if val.status() == StatusCode::OK => val,
            Ok(_) => return Err(Error::TokenRequestEndpointInvalidResponse),
            Err(_) => return Err(Error::TokenRequestEndpointNotReachable),
        };

        let body = match hyper::body::to_bytes(response).await {
            Ok(val) => val,
            Err(_) => return Err(Error::TokenRequestContentInvalid),
        };

        match serde_json::from_slice::<TokenSet>(&body) {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::TokenRequestDeserialization),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
    pub refresh_token: Option<String>,
    pub id_token: String,
}

#[derive(Debug, Deserialize)]
pub struct IdToken {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub hd: String,
    pub picture: String,
}

impl IdToken {
    pub fn new(token: &str) -> Result<Self, Error> {
        match dangerous_insecure_decode::<Self>(token) {
            Ok(val) => Ok(val.claims),
            Err(_) => Err(Error::IdTokenInvalid),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn authorization_code_length_too_short() {
        let too_short_code = String::new();
        assert_eq!(
            Result::Err(Error::AuthorizationCodeLength),
            AuthorizationCode::new(too_short_code)
        );
    }

    #[test]
    fn authorization_code_length_too_long() {
        let too_long_code = "a".repeat(257);
        assert_eq!(
            Result::Err(Error::AuthorizationCodeLength),
            AuthorizationCode::new(too_long_code)
        );
    }

    #[test]
    fn authorization_code_invalid_character() {
        assert_eq!(
            Result::Err(Error::AuthorizationCodeInvalidCharacter),
            AuthorizationCode::new("abc$".to_string())
        );
        assert_eq!(
            Result::Err(Error::AuthorizationCodeInvalidCharacter),
            AuthorizationCode::new("ab(c".to_string())
        );
        assert_eq!(
            Result::Err(Error::AuthorizationCodeInvalidCharacter),
            AuthorizationCode::new("a%bc".to_string())
        );
    }
}
