#![no_std]

#[cfg(feature = "alloc")]
pub mod alloc;
pub(crate) mod either;
mod error;
pub mod extract;
mod handler;
#[cfg(feature = "heapless")]
pub mod heapless;
pub mod http;
mod io;
pub(crate) mod macros;
mod parse;
mod path;
pub mod request;
pub mod response;
mod route;
mod router;
mod service;
mod utils;

pub use extract::{FromRef, FromRequest, FromRequestParts};
pub use io::{ErrorType, Read, Write};
pub use path::{PathSegments, Segment};
pub use request::{Headers, Parts, Request};
pub use response::{IntoResponse, Response};
pub use route::{connect, delete, get, head, options, patch, post, put, trace};
pub use router::Router;
pub use service::Service;
