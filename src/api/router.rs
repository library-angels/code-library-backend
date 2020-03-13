use warp::{Filter, filters::BoxedFilter, Reply};
use std::collections::HashMap;


pub fn root() -> BoxedFilter<(impl Reply,)> {
    // GET - /
    warp::path::end()
        .and(warp::get())
        .map(|| {format!("root")}).boxed()
}

pub fn identity() -> BoxedFilter<(impl Reply,)> {
    warp::path("identity").and(
        // GET - /identity/users
        warp::path("users")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("users index")})

        // GET - /identity/users/<u32>
        .or(warp::path("users")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|user_id| {format!("users id: {}", user_id)}))

        // GET - /identity/roles
        .or(warp::path("roles")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("roles index")}))

        // GET - /identity/roles/<u32>
        .or(warp::path("roles")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|role_id| {format!("roles id: {}", role_id)}))

        .or(warp::path("oauth").and(
            // GET - /identity/oauth/client_identifier
            warp::path("client_identifier")
                .and(warp::path::end())
                .and(warp::get())
                .map(|| {format!("client_identifier")})

            // GET - /identity/oauth/authorization_code_exchange
            .or(warp::path("authorization_code_exchange")
                .and(warp::path::end())
                .and(warp::get())
                .and(warp::query())
                .map(|query: HashMap<String, String>| {format!("auth_code_exchange")}))
            )
        )

        .or(warp::path("jwt").and(
            // GET - /identity/jwt/info
            warp::path("info")
                .and(warp::path::end())
                .and(warp::get())
                .map(|| {format!("info")})

            // POST - /identity/jwt/refresh
            .or(warp::path("refresh")
                .and(warp::path::end())
                .and(warp::post())
                .and(warp::body::json())
                .map(|body: HashMap<String, String>| {format!("refresh")}))
            )
        )
    ).boxed()
}

pub fn book() -> BoxedFilter<(impl Reply,)> {
    warp::path("book").and(
        // GET - /book/
        warp::path::end()
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("book index")})
        
        // GET - /book/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {}", book_id)}))

        // GET - /book/<u32>/designations
        .or(warp::path::param::<u32>()
            .and(warp::path("designations"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - designations index", book_id)}))

        // GET - /book/<u32>/designations/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("designations"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, designation_id| {format!("book id: {} - designation id: {}", book_id, designation_id)}))

        // GET - /book/<u32>/tags
        .or(warp::path::param::<u32>()
            .and(warp::path("tags"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - tags index", book_id)}))
        
        // GET - /book/<u32>/tags/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("tags"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, tag_id| {format!("book id: {} - tag id: {}", book_id, tag_id)}))

        // GET - /book/<u32>/publishers
        .or(warp::path::param::<u32>()
            .and(warp::path("publishers"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - publishers index", book_id)}))

        // GET - /book/<u32>/publishers/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("publishers"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, publisher_id| {format!("book id: {} - publishers id: {}", book_id, publisher_id)}))

        // GET - /book/<u32>/authors
        .or(warp::path::param::<u32>()
            .and(warp::path("authors"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - authors index", book_id)}))

        // GET - /book/<u32>/authors/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("authors"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, author_id| {format!("book id: {} - author id: {}", book_id, author_id)}))

        // GET - /book/<u32>/editors
        .or(warp::path::param::<u32>()
            .and(warp::path("editors"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - editors index", book_id)}))

        // GET - /book/<u32>/editors/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("editors"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, editor_id| {format!("book id: {} - editor id: {}", book_id, editor_id)}))

        // GET - /book/<u32>/series
        .or(warp::path::param::<u32>()
            .and(warp::path("series"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - series index", book_id)}))

        // GET - /book/<u32>/series/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("series"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, series_id| {format!("book id: {} - series id: {}", book_id, series_id)}))

        // GET - /book/<u32>/languages
        .or(warp::path::param::<u32>()
            .and(warp::path("languages"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - languages index", book_id)}))

        // GET - /book/<u32>/languages/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("languages"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, language_id| {format!("book id: {} - language id: {}", book_id, language_id)}))

        // GET - /book/<u32>/physical_sizes
        .or(warp::path::param::<u32>()
            .and(warp::path("physical_sizes"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - physical_sizes index", book_id)}))

        // GET - /book/<u32>/physical_sizes/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("physical_sizes"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, physical_size_id| {format!("book id: {} - physical_size id: {}", book_id, physical_size_id)}))

        // GET - /book/<u32>/subject_areas
        .or(warp::path::param::<u32>()
            .and(warp::path("subject_areas"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - subject_areas index", book_id)}))

        // GET - /book/<u32>/subject_areas/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("subject_areas"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, subject_area_id| {format!("book_id: {} - subject_area id: {}", book_id, subject_area_id)}))

        // GET - /book/<u32>/copies
        .or(warp::path::param::<u32>()
            .and(warp::path("copies"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id| {format!("book id: {} - copies", book_id)}))
        
        // GET - /book/<u32>/copies/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path("copies"))
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|book_id, copy_id| {format!("book id: {} - copy id: {}", book_id, copy_id)}))

        // GET - /book/designations
        .or(warp::path("designations")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("designations index")}))

        // GET - /book/designations/<u32>
        .or(warp::path("designations")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|designation_id| {format!("designations id: {}", designation_id)}))

        // GET - /book/tags
        .or(warp::path("tags")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("tags index")}))

        // GET - /book/tags/<u32>
        .or(warp::path("tags")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|tag_id| {format!("tags id: {}", tag_id)}))

        // GET - /book/publishers
        .or(warp::path("publishers")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("publishers index")}))
        
        // GET - /book/publishers/<u32>
        .or(warp::path("publishers")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|publisher_id| {format!("publishers id: {}", publisher_id)}))

        // GET - /book/authors
        .or(warp::path("authors")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("authors index")}))
        
        // GET - /book/authors/<u32>
        .or(warp::path("authors")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|author_id| {format!("authors id: {}", author_id)}))

        // GET - /book/editors
        .or(warp::path("editors")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("editors index")}))

        // GET - /book/editors/<u32>
        .or(warp::path("editors")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|editor_id| {format!("editors id: {}", editor_id)}))

        // GET - /book/series
        .or(warp::path("series")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("series index")}))

        // GET - /book/series/<u32>
        .or(warp::path("series")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|series_id| {format!("series id: {}", series_id)}))

        // GET - /book/languages
        .or(warp::path("languages")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("languages index")}))

        // GET - /book/languages/<u32>
        .or(warp::path("languages")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|language_id| {format!("languages id: {}", language_id)}))

        // GET - /book/physical_sizes
        .or(warp::path("physical_sizes")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("physical_sizes index")}))

        // GET - /book/physical_sizes/<u32>
        .or(warp::path("physical_sizes")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|physical_size_id| {format!("physical_sizes id: {}", physical_size_id)}))

        // GET - /book/subject_areas
        .or(warp::path("subject_areas")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|query: HashMap<String, String>| {format!("subject_areas index")}))
        
        // GET - /book/subject_areas/<u32>
        .or(warp::path("subject_areas")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|subject_area_id| {format!("subject_areas id: {}", subject_area_id)}))
    ).boxed()
}

pub fn borrow() -> BoxedFilter<(impl Reply,)> {
    warp::path("borrow").and(
        // GET - /borrow/active
        warp::path("active")
            .and(warp::path::end())
            .and(warp::get())
            .map(|| {format!("active index")})

        // POST /borrow/active
        .or(warp::path("active")
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .map(|body: HashMap<String, String>| {format!("active index post")}))

        // GET - /borrow/active/<u32>
        .or(warp::path("active")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|borrow_id| {format!("active id: {}", borrow_id)}))

        // GET - /borrow/history
        .or(warp::path("history")
            .and(warp::path::end())
            .and(warp::get())
            .map(|| {format!("history index")}))

        // GET - /borrow/history/<u32>
        .or(warp::path("history")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .map(|borrow_id| {format!("history id: {}", borrow_id)}))
    ).boxed()
}

pub fn notification() -> BoxedFilter<(impl Reply,)> {
    warp::path("notification")
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {format!("notification")}).boxed()
}
