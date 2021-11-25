use std::str::FromStr;

use http_types::Method as HttpTypesMethod;
use hyper::http::method::Method as HyperMethod;

use crate::*;

impl TryFromHttp<HyperMethod> for HttpTypesMethod {
    fn try_from_http(method: HyperMethod) -> anyhow::Result<Self> {
        HttpTypesMethod::from_str(method.as_str()).map_err(anyhow::Error::msg)
    }
}

#[cfg(test)]
mod hyper_into_http_types_method_tests {
    use super::*;

    #[test]
    fn should_map_get() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::GET), HttpTypesMethod::Get);
    }

    #[test]
    fn should_map_post() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::POST), HttpTypesMethod::Post);
    }

    #[test]
    fn should_map_put() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::PUT), HttpTypesMethod::Put);
    }

    #[test]
    fn should_map_delete() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::DELETE), HttpTypesMethod::Delete);
    }

    #[test]
    fn should_map_patch() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::PATCH), HttpTypesMethod::Patch);
    }

    #[test]
    fn should_map_head() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::HEAD), HttpTypesMethod::Head);
    }

    #[test]
    fn should_map_options() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::OPTIONS), HttpTypesMethod::Options);
    }

    #[test]
    fn should_map_connect() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::CONNECT), HttpTypesMethod::Connect);
    }

    #[test]
    fn should_map_trace() {
        assert_eq!(HttpTypesMethod::from_http(HyperMethod::TRACE), HttpTypesMethod::Trace);
    }
}