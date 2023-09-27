/// A way to write simple code while preserving the original errors is to Box them.
///
/// The drawback is that the underlying error type is only known at runtime and not statically determined.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type DynResult<T> = std::result::Result<T, BoxError>;
