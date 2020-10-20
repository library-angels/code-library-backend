table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        access_manage_books -> Bool,
        access_manage_roles -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        sub -> Varchar,
        email -> Varchar,
        given_name -> Varchar,
        family_name -> Varchar,
        picture -> Varchar,
        oauth_access_token -> Varchar,
        oauth_access_token_valid -> Timestamp,
        oauth_refresh_token -> Varchar,
        active -> Bool,
    }
}

table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(roles, users, users_roles,);
