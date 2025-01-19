use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use super::{endpoints::Endpoint, Response};
pub type FormResponse<T> = Result<Response<T>, Response<()>>;
#[async_trait]
pub trait SubmitForm {
    async fn submit<T: Serialize + Debug + DeserializeOwned>(&self, endpoint: Endpoint) -> T;
}
