use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TauriCommandArgument<T>
where
    T: Serialize,
{
    pub payload: T,
}
