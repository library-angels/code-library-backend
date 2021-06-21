use serde::Serialize;
use warp::reply::{json, Json};

#[derive(Serialize)]
pub struct JsonObjectReply<T>
where
    T: Serialize,
{
    data: T,
}

pub fn json_object_reply<T>(data: &T) -> Json
where
    T: Serialize,
{
    json(&JsonObjectReply { data })
}

#[derive(Serialize)]
pub struct JsonVectorReply<'a, T>
where
    T: Serialize,
{
    data: &'a [T],
}

pub fn json_vector_reply<T>(data: &[T]) -> Json
where
    T: Serialize,
{
    json(&JsonVectorReply { data })
}
