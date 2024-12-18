pub mod sign_up;

pub trait SubmitForm {
    fn submit_form<T, E>(
        &self,
        endpoint: &'static str,
    ) -> impl std::future::Future<Output = Result<T, E>> + Send;
}
