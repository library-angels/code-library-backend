use chrono::NaiveDate;

use identity::db::models::{UserAddUpdate, UserRoleAddUpdate};

pub fn users() -> Vec<UserAddUpdate> {
    vec![
        UserAddUpdate {
            sub: "1".into(),
            email: "john.doe@example.net".into(),
            given_name: "John".into(),
            family_name: "Doe".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "2".into(),
            email: "jack.kerr@example.net".into(),
            given_name: "Jack".into(),
            family_name: "Kerr".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "3".into(),
            email: "justin.wilkins@example.net".into(),
            given_name: "Justin".into(),
            family_name: "Wilkins".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "4".into(),
            email: "tim.jackson@example.net".into(),
            given_name: "Tim".into(),
            family_name: "Jackson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: false,
        },
        UserAddUpdate {
            sub: "5".into(),
            email: "richard.henderson@example.net".into(),
            given_name: "Richard".into(),
            family_name: "Henderson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "6".into(),
            email: "olivia.springer@example.net".into(),
            given_name: "Olivia".into(),
            family_name: "Springer".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "7".into(),
            email: "alexander.carr@example.net".into(),
            given_name: "Alexander".into(),
            family_name: "Carr".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "8".into(),
            email: "yvonne.thomson@example.net".into(),
            given_name: "Yvonne".into(),
            family_name: "Thomson".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "9".into(),
            email: "alan.hemmings@example.net".into(),
            given_name: "Alan".into(),
            family_name: "Hemmings".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "10".into(),
            email: "eric.vance@example.net".into(),
            given_name: "Eric".into(),
            family_name: "Vance".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
        UserAddUpdate {
            sub: "11".into(),
            email: "audrey.miller@example.net".into(),
            given_name: "Audrey".into(),
            family_name: "Miller".into(),
            picture: "https://example.com/avatar.jpg".into(),
            oauth_access_token: "access_token".into(),
            oauth_access_token_valid: NaiveDate::from_ymd(2020, 12, 31).and_hms(0, 0, 0),
            oauth_refresh_token: Some("refresh_token".into()),
            active: true,
        },
    ]
}

pub fn users_roles() -> Vec<UserRoleAddUpdate> {
    vec![
        UserRoleAddUpdate {
            user_id: 1,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 2,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 3,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 4,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 5,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 6,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 7,
            role_id: 1,
        },
        UserRoleAddUpdate {
            user_id: 8,
            role_id: 2,
        },
        UserRoleAddUpdate {
            user_id: 9,
            role_id: 2,
        },
        UserRoleAddUpdate {
            user_id: 10,
            role_id: 3,
        },
        UserRoleAddUpdate {
            user_id: 11,
            role_id: 1,
        },
    ]
}
