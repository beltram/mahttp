use http_types::Request as HttpTypesRequest;
use hyper::Request as HyperRequest;

use crate::*;

impl<T> TryFromHttp<HyperRequest<T>> for HttpTypesRequest {
    fn try_from_http(_: HyperRequest<T>) -> anyhow::Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod hyper_into_http_types_tests {
    use super::*;

    #[test]
    fn should_map_method() {
        let uri = "http://localhost";
        assert_eq!(
            HttpTypesRequest::get(uri).method(),
            HttpTypesRequest::from_http(HyperRequest::get(uri).body(()).unwrap()).method()
        );
    }
}