#[tarpc::service]
pub trait Book {
    async fn book();
}
