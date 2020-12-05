pub mod oauth;

use chrono::{Duration, NaiveDateTime};
use diesel::result::{Error, QueryResult};

use crate::db::models::{User, UserAddUpdate};
use oauth::{IdToken, TokenSet};

#[derive(Debug, PartialEq)]
pub enum AccountStatus {
    Active,
    Inactive,
    New,
}

pub fn check_account_status(query_result: QueryResult<User>) -> Result<AccountStatus, Error> {
    match query_result {
        Ok(val) => {
            if val.active {
                Ok(AccountStatus::Active)
            } else {
                Ok(AccountStatus::Inactive)
            }
        }
        Err(Error::NotFound) => Ok(AccountStatus::New),
        Err(e) => Err(e),
    }
}

pub fn create_user_from_oauth_authentication(
    id_token: &IdToken,
    token_set: &TokenSet,
    creation_time: NaiveDateTime,
) -> UserAddUpdate {
    UserAddUpdate {
        sub: id_token.sub.clone(),
        email: id_token.email.clone(),
        given_name: id_token.given_name.clone(),
        family_name: id_token.family_name.clone(),
        picture: id_token.picture.clone(),
        oauth_access_token: token_set.access_token.clone(),
        oauth_access_token_valid: creation_time + Duration::seconds(token_set.expires_in.into()),
        oauth_refresh_token: token_set.refresh_token.clone(),
        active: true,
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    // check for active account
    #[test]
    fn check_active_account() {
        let query_result = Ok(User {
            id: 1,
            sub: "1".into(),
            email: "john.doe@example.net".into(),
            given_name: "John".into(),
            family_name: "Doe".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: true,
        });

        assert_eq!(
            Ok(AccountStatus::Active),
            check_account_status(query_result)
        );
    }

    // check for inactive account
    #[test]
    fn check_inactive_account() {
        let query_result = Ok(User {
            id: 1,
            sub: "1".into(),
            email: "john.doe@example.net".into(),
            given_name: "John".into(),
            family_name: "Doe".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: "refresh_token".into(),
            active: false,
        });

        assert_eq!(
            Ok(AccountStatus::Inactive),
            check_account_status(query_result)
        );
    }

    // check for new account
    #[test]
    fn check_new_account() {
        let query_result = Err(Error::NotFound);
        assert_eq!(Ok(AccountStatus::New), check_account_status(query_result));
    }

    // create new user from oauth authentication
    #[test]
    fn new_user_from_oauth_authentication() {
        let token_set = TokenSet {
            access_token: "access_token".into(),
            expires_in: 100,
            scope: "".into(),
            token_type: "".into(),
            refresh_token: Some("refresh_token".into()),
            id_token: "".into(),
        };

        let id_token = IdToken {
            iss: "".into(),
            aud: "".into(),
            sub: "sub".into(),
            iat: 0,
            exp: 0,
            email: "email".into(),
            given_name: "name".into(),
            family_name: "name".into(),
            hd: "".into(),
            picture: "picture".into(),
        };

        let creation_time = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0);

        let expected_result = UserAddUpdate {
            sub: id_token.sub.clone(),
            email: id_token.email.clone(),
            given_name: id_token.given_name.clone(),
            family_name: id_token.family_name.clone(),
            picture: id_token.picture.clone(),
            oauth_access_token: token_set.access_token.clone(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 1, 40),
            oauth_refresh_token: token_set.refresh_token.clone(),
            active: true,
        };

        let result = create_user_from_oauth_authentication(&id_token, &token_set, creation_time);

        assert_eq!(expected_result, result);
    }

    // create new user from oauth authentication
    #[test]
    fn existing_user_from_oauth_authentication() {
        let token_set = TokenSet {
            access_token: "access_token".into(),
            expires_in: 100,
            scope: "".into(),
            token_type: "".into(),
            refresh_token: None,
            id_token: "".into(),
        };

        let id_token = IdToken {
            iss: "".into(),
            aud: "".into(),
            sub: "sub".into(),
            iat: 0,
            exp: 0,
            email: "email".into(),
            given_name: "name".into(),
            family_name: "name".into(),
            hd: "".into(),
            picture: "picture".into(),
        };

        let creation_time = NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0);

        let expected_result = UserAddUpdate {
            sub: id_token.sub.clone(),
            email: id_token.email.clone(),
            given_name: id_token.given_name.clone(),
            family_name: id_token.family_name.clone(),
            picture: id_token.picture.clone(),
            oauth_access_token: token_set.access_token.clone(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 1, 40),
            oauth_refresh_token: token_set.refresh_token.clone(),
            active: true,
        };

        let result = create_user_from_oauth_authentication(&id_token, &token_set, creation_time);

        assert_eq!(expected_result, result);
    }
}
