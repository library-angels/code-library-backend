#[tarpc::service]
pub trait Notification {
    async fn notification();
}
