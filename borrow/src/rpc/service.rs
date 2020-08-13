#[tarpc::service]
pub trait Borrow {
    async fn borrow();
}
