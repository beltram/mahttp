use http_types::Request as HttpTypesRequest;
use hyper::Request as HyperRequest;

use crate::*;

impl<T> TryFromHttp<HttpTypesRequest> for HyperRequest<T> {
    fn try_from_http(_: HttpTypesRequest) -> anyhow::Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod http_types_into_hyper_tests {
    use super::*;

    #[test]
    fn should_map_method() {
        let uri = "http://localhost";
        assert_eq!(
            HyperRequest::get(uri).body(()).unwrap().method(),
            HyperRequest::<()>::from_http(HttpTypesRequest::get(uri)).method()
        );
    }
}