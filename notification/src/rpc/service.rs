#[tarpc::service]
pub trait NotificationService {
    async fn notification();
}
