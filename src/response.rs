use crate::{http::StatusCode, Read};

pub struct Response<Body> {
    pub status_code: StatusCode,
    pub content_type: Option<&'static [u8]>,
    // TODO: generalize headers
    pub body: Body,
}

impl<Body> Response<Body> {
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn into_body(self) -> Body {
        self.body
    }

    pub(crate) fn with_content_type(self, content_type: Option<&'static [u8]>) -> Self {
        Self {
            status_code: self.status_code,
            content_type,
            body: self.body,
        }
    }

    pub(crate) fn map_body<F, T>(self, map: F) -> Response<T>
    where
        F: FnOnce(Body) -> T,
    {
        Response {
            status_code: self.status_code,
            content_type: self.content_type,
            body: map(self.body),
        }
    }
}

pub trait IntoResponse {
    type Body: Read; // TODO: this should probably be a Body trait (with content-length?)

    fn into_response(self) -> Response<Self::Body>;
}

impl IntoResponse for core::convert::Infallible {
    type Body = &'static [u8];

    fn into_response(self) -> Response<Self::Body> {
        match self {}
    }
}

impl<Body: Read> IntoResponse for Response<Body>
where
    Body: 'static,
{
    type Body = Body;

    fn into_response(self) -> Response<Self::Body> {
        self
    }
}

impl IntoResponse for &'static str {
    type Body = &'static [u8];

    fn into_response(self) -> Response<Self::Body> {
        Response {
            status_code: StatusCode::OK,
            content_type: Some(b"text/plain"),
            body: self.as_bytes(),
        }
    }
}

impl<T: IntoResponse> IntoResponse for (StatusCode, T) {
    type Body = T::Body;

    fn into_response(self) -> Response<Self::Body> {
        let mut response = self.1.into_response();
        response.status_code = self.0;
        response
    }
}

impl IntoResponse for () {
    type Body = &'static [u8];

    fn into_response(self) -> Response<Self::Body> {
        (StatusCode::OK, "").into_response()
    }
}
