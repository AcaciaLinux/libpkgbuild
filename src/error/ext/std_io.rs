/// This trait allows any Result that returns an error to use some common functions
pub trait StdIOErrorExt<T> {
    /// This function can take a Result and if it is an `Err`, it appends the
    /// supplied message
    /// # Arguments
    /// * `message` - The message to append to the error
    fn err_append(self, message: &str) -> Result<T, std::io::Error>;

    /// This function can take a Result and if it is an `Err`, it prepends the
    /// supplied message
    /// # Arguments
    /// * `message` - The message to prepend to the error
    fn err_prepend(self, message: &str) -> Result<T, std::io::Error>;
}

/// Implement the trait
impl<T> StdIOErrorExt<T> for Result<T, std::io::Error> {
    fn err_append(self, message: &str) -> Result<T, std::io::Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(
                e.kind(),
                format!("{}: {}", e.to_string(), message),
            )),
        }
    }

    fn err_prepend(self, message: &str) -> Result<T, std::io::Error> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(
                e.kind(),
                format!("{}: {}", message, e.to_string()),
            )),
        }
    }
}
