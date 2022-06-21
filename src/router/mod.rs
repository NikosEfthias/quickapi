use std::collections::HashMap;

struct Route<B, R, H>
where
	H: FnOnce(http::request::Request<B>) -> http::response::Response<R>,
{
	subroutes: HashMap<String, Box<Route<B, R, H>>>,
	handler: Option<H>,
	_r: std::marker::PhantomData<R>,
	_b: std::marker::PhantomData<B>,
}
pub struct Router<B, R, H>
where
	H: FnOnce(http::request::Request<B>) -> http::response::Response<R>,
{
	get: HashMap<String, Route<B, R, H>>,
	post: HashMap<String, Route<B, R, H>>,
	put: HashMap<String, Route<B, R, H>>,
	delete: HashMap<String, Route<B, R, H>>,
	patch: HashMap<String, Route<B, R, H>>,
	head: HashMap<String, Route<B, R, H>>,
	options: HashMap<String, Route<B, R, H>>,
	trace: HashMap<String, Route<B, R, H>>,
	connect: HashMap<String, Route<B, R, H>>,
}
impl<B, R, H> Router<B, R, H>
where
	H: FnOnce(http::request::Request<B>) -> http::response::Response<R>,
{
	pub fn new() -> Self
//{{{
	{
		Router {
			..Default::default()
		}
	} //}}}
	pub fn get(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.get.get_mut(path) {
			if route.handler.is_some() {
				panic!("Get Route already exists for path: {}", path);
			}
			return;
		}
		self.get.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn post(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.post.get_mut(path) {
			if route.handler.is_some() {
				panic!("Post Route already exists for path: {}", path);
			}
			return;
		}
		self.post.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn put(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.put.get_mut(path) {
			if route.handler.is_some() {
				panic!("Put Route already exists for path: {}", path);
			}
			return;
		}
		self.put.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn delete(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.delete.get_mut(path) {
			if route.handler.is_some() {
				panic!("Delete Route already exists for path: {}", path);
			}
			return;
		}
		self.delete.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn patch(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.patch.get_mut(path) {
			if route.handler.is_some() {
				panic!("Patch Route already exists for path: {}", path);
			}
			return;
		}
		self.patch.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn head(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.head.get_mut(path) {
			if route.handler.is_some() {
				panic!("Head Route already exists for path: {}", path);
			}
			return;
		}
		self.head.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn options(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.options.get_mut(path) {
			if route.handler.is_some() {
				panic!("Options Route already exists for path: {}", path);
			}
			return;
		}
		self.options.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn trace(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.trace.get_mut(path) {
			if route.handler.is_some() {
				panic!("Trace Route already exists for path: {}", path);
			}
			return;
		}
		self.trace.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
	pub fn connect(&mut self, path: &str, handler: H)
	//{{{
	{
		if let Some(route) = self.connect.get_mut(path) {
			if route.handler.is_some() {
				panic!("Connect Route already exists for path: {}", path);
			}
			return;
		}
		self.connect.insert(
			path.to_string(),
			Route {
				subroutes: HashMap::new(),
				handler: Some(handler),
				_r: std::marker::PhantomData,
				_b: std::marker::PhantomData,
			},
		);
	} //}}}
}

impl<B, R, H> Default for Router<B, R, H>
where
	H: FnOnce(http::request::Request<B>) -> http::response::Response<R>,
{
	fn default() -> Self
	{
		Self::new()
	}
}
