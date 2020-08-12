use serde::{Deserialize, Serialize};
use std::time::*;
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct Jwt {
    pub sub: u32,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug)]
pub enum JwtError {
    TokenInvalid,
}

impl Jwt {
    pub fn new(
        sub: u32,
        given_name: String,
        family_name: String,
        picture: String,
        jwt_validity: u64
    ) -> Jwt {
        let iat = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let exp = iat + Duration::from_secs(jwt_validity);
        Jwt {
            sub,
            given_name,
            family_name,
            picture,
            iat: iat.as_secs(),
            exp: exp.as_secs()
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
            &token,
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
    use super::*;

    #[test]
    fn jwt_creation() {
        let validity = 3600;
        let jwt = Jwt::new(
            1,
            "given_name".to_string(),
            "family_name".to_string(),
            "picture".to_string(),
            validity
        );

        assert_eq!(1, jwt.sub);
        assert_eq!("given_name", jwt.given_name);
        assert_eq!("family_name", jwt.family_name);
        assert_eq!("picture", jwt.picture);
        assert_eq!(validity, jwt.exp - jwt.iat);
    }

    #[test]
    fn jwt_encoding_decoding() {
        let jwt = Jwt::new(
            1,
            "given_name".to_string(),
            "family_name".to_string(),
            "picture".to_string(),
            3600
        );

        let jwt_encoded = jwt.encode("super_secret");
        let jwt_decoded = Jwt::decode("super_secret", &jwt_encoded).unwrap();

        assert_eq!(jwt, jwt_decoded);
    }
}
