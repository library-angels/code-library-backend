use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct Jwt {
    pub sub: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug)]
pub enum JwtError {
    TokenInvalid,
}

impl Jwt {
    pub fn new(
        sub: Uuid,
        given_name: String,
        family_name: String,
        picture: String,
        creation_time: DateTime<Utc>,
        validity: Duration,
    ) -> Jwt {
        Jwt {
            sub,
            given_name,
            family_name,
            picture,
            iat: creation_time.timestamp(),
            exp: (creation_time + validity).timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    pub fn decode(secret: &str, token: &str) -> Result<Jwt, JwtError> {
        match decode::<Jwt>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(val) => Ok(val.claims),
            Err(_) => Err(JwtError::TokenInvalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn jwt_creation() {
        let id = Uuid::new_v4();
        let creation_time = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let validity = Duration::seconds(3600);
        let jwt = Jwt::new(
            id,
            "given_name".to_string(),
            "family_name".to_string(),
            "picture".to_string(),
            creation_time,
            validity,
        );

        assert_eq!(id, jwt.sub);
        assert_eq!("given_name", jwt.given_name);
        assert_eq!("family_name", jwt.family_name);
        assert_eq!("picture", jwt.picture);
        assert_eq!(1577836800, jwt.iat);
        assert_eq!(3600, jwt.exp - jwt.iat);
    }

    #[test]
    fn jwt_encoding_decoding() {
        let jwt = Jwt::new(
            Uuid::new_v4(),
            "given_name".to_string(),
            "family_name".to_string(),
            "picture".to_string(),
            Utc::now(),
            Duration::seconds(3600),
        );

        let jwt_encoded = jwt.encode("super_secret");
        let jwt_decoded = Jwt::decode("super_secret", &jwt_encoded).unwrap();

        assert_eq!(jwt, jwt_decoded);
    }
}
