mod error;
pub mod http;
mod router;
pub use error::*;

pub fn start()
{
	let l = std::net::TcpListener::bind("localhost:8080").unwrap();
	while let Ok((mut socket, _)) = l.accept() {
		let req = http::parser::parse_req(&mut socket).unwrap();
		dbg!(req);
	}
}
