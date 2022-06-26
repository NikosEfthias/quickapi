pub(crate) mod parser;
pub use http::*;
use std::{fmt::Debug, io::Read};
pub struct Body<'a>(Box<dyn Read + 'a>);
impl Debug for Body<'_>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "[ Body ]")
	}
}
