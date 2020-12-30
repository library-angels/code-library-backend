table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        sub -> Varchar,
        email -> Varchar,
        given_name -> Varchar,
        family_name -> Varchar,
        picture -> Varchar,
        oauth_access_token -> Varchar,
        oauth_access_token_valid -> Timestamp,
        oauth_refresh_token -> Varchar,
        active -> Bool,
        role_id -> Uuid,
    }
}

joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(roles, users,);
