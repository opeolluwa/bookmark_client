pub trait CommandError {
    fn from(message: &str) -> String;
}
