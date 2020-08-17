#[tarpc::service]
pub trait BorrowService {
    async fn borrow();
}
