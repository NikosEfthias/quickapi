use std::{
	io::{BufRead, BufReader},
	net::TcpStream,
};
use tap::Pipe;

use http::{Request, Uri};

use crate::{http::Body, Error};

pub fn parse_req(sock: &mut TcpStream) -> crate::Result<Request<super::super::Body>>
where
{
	let mut r = BufReader::new(sock);
	let mut lines = (&mut r).lines();
	let init_line = lines.next().ok_or(Error::InvalidRequest)?;
	parse_init_line(&init_line?)?
		.pipe(|b| parse_headers(b, &mut lines))?
		.body(Body(Box::new(r)))?
		.pipe(Ok)
}
fn parse_headers(
	mut builder: http::request::Builder,
	lines: impl Iterator<Item = std::io::Result<String>>,
) -> crate::Result<http::request::Builder>
{
	for line in lines {
		let line = line?;
		if line.is_empty() {
			break;
		}
		if let Some((k, v)) = line.split_once(':') {
			builder = builder.header(k.trim(), v.trim());
		} else {
			return Err(Error::InvalidHeader);
		}
	}
	Ok(builder)
}
fn parse_init_line(line: &str) -> crate::Result<http::request::Builder>
//{{{
{
	let mut parts = line.split(' ');
	let method = parts.next().ok_or(Error::InvalidRequest)?;
	http::request::Builder::new()
		.method(match method.to_uppercase().as_str() {
			"GET" => http::Method::GET,
			"POST" => http::Method::POST,
			"PUT" => http::Method::PUT,
			"DELETE" => http::Method::DELETE,
			"HEAD" => http::Method::HEAD,
			"OPTIONS" => http::Method::OPTIONS,
			"CONNECT" => http::Method::CONNECT,
			"TRACE" => http::Method::TRACE,
			"PATCH" => http::Method::PATCH,
			_ => return Err(Error::InvalidMethod(method.to_string())),
		})
		.uri(
			parts
				.next()
				.ok_or(Error::InvalidRequest)?
				.parse::<Uri>()
				.map_err(|_| Error::InvalidRequest)?,
		)
		.version(match parts.next() {
			Some("HTTP/1.0") => http::Version::HTTP_10,
			Some("HTTP/1.1") => http::Version::HTTP_11,
			_ => return Err(Error::UnsupportedHttp),
		})
		.pipe(Ok)
} //}}}
