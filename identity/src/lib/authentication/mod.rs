pub mod oauth;

use diesel::result::{Error, QueryResult};

use crate::db::models::User;

#[derive(Debug, PartialEq)]
pub enum AccountStatus {
    Active,
    Inactive,
    New,
}

pub fn check_account_status(query_result: QueryResult<User>) -> Result<AccountStatus, Error> {
    unimplemented!();
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
}
