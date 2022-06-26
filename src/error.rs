pub type Result<T> = std::result::Result<T, Error>;
#[derive(thiserror::Error, Debug)]
pub enum Error
{
	#[error("invalid_request")]
	InvalidRequest,
	#[error("unsupported_http_version")]
	UnsupportedHttp,
	#[error("invalid_path")]
	InvalidPath,
	#[error("io_error")]
	IoError(#[from] std::io::Error),
	#[error("invalid_header")]
	InvalidHeader,
	#[error("invalid_method__{0}")]
	InvalidMethod(String),
	#[error("http__{0}")]
	HttpError(#[from] http::Error),
}
