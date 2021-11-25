use hyper::Request as HyperRequest;

use mahttp::*;

mod into_http_types {
    use http_types::Request as HttpTypesRequest;

    use super::*;

    #[test]
    fn should_map_req() {
        let uri = "http://localhost";
        let a = HyperRequest::get(uri).body(()).unwrap();
        let b = HyperRequest::<()>::from_http(HttpTypesRequest::from_http(HyperRequest::get(uri).body(()).unwrap()));
        assert_eq!(a.method(), b.method())
    }
}