use std::str::FromStr;

use http_types::Method as HttpTypesMethod;
use hyper::http::method::Method as HyperMethod;

use crate::*;

impl TryFromHttp<HttpTypesMethod> for HyperMethod {
    fn try_from_http(method: HttpTypesMethod) -> anyhow::Result<Self> {
        HyperMethod::from_str(method.as_ref()).map_err(anyhow::Error::msg)
    }
}

#[cfg(test)]
mod http_types_into_hyper_method_tests {
    use super::*;

    #[test]
    fn should_map_get() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Get), HyperMethod::GET);
    }

    #[test]
    fn should_map_post() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Post), HyperMethod::POST);
    }

    #[test]
    fn should_map_put() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Put), HyperMethod::PUT);
    }

    #[test]
    fn should_map_delete() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Delete), HyperMethod::DELETE);
    }

    #[test]
    fn should_map_patch() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Patch), HyperMethod::PATCH);
    }

    #[test]
    fn should_map_head() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Head), HyperMethod::HEAD);
    }

    #[test]
    fn should_map_options() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Options), HyperMethod::OPTIONS);
    }

    #[test]
    fn should_map_connect() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Connect), HyperMethod::CONNECT);
    }

    #[test]
    fn should_map_trace() {
        assert_eq!(HyperMethod::from_http(HttpTypesMethod::Trace), HyperMethod::TRACE);
    }
}