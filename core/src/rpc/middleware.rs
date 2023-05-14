use jsonrpc_core::futures_util::future::{Either, Future};
use jsonrpc_core::{
    middleware, Error, ErrorCode, FutureResponse, Metadata, Middleware, Request, Response, Version,
};

#[derive(Debug, PartialEq)]
enum Service {
    VPN,
    P2P,
    Blockchain,
    Core,
}

#[derive(Debug, PartialEq)]
enum RequestFormatError {
    EmptyRequest,
    MethodNotProvided,
    ServiceNotProvided,
    UnknownService,
}

impl std::str::FromStr for Service {
    type Err = RequestFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vpn" => Ok(Service::VPN),
            "p2p" => Ok(Service::P2P),
            "blockchain" => Ok(Service::Blockchain),
            "core" => Ok(Service::Core),
            "" => Err(RequestFormatError::ServiceNotProvided),
            _ => Err(RequestFormatError::UnknownService),
        }
    }
}

impl std::fmt::Display for RequestFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestFormatError::EmptyRequest => write!(f, "Empty request"),
            RequestFormatError::MethodNotProvided => write!(f, "Service method is not provided"),
            RequestFormatError::ServiceNotProvided => write!(f, "Service name is not provided"),
            RequestFormatError::UnknownService => write!(f, "Unknown service name"),
        }
    }
}

fn select_request_prefix_service(request: &str) -> Result<Service, RequestFormatError> {
    match request.split_once(".") {
        Some((service_name, _)) => service_name.parse::<Service>(),
        None => Err(RequestFormatError::EmptyRequest),
    }
}

fn select_request_method(request: &str) -> Result<&str, RequestFormatError> {
    match request.split_once(".") {
        Some((_, method_name)) => {
            if !method_name.is_empty() {
                Ok(method_name)
            } else {
                Err(RequestFormatError::MethodNotProvided)
            }
        }
        None => Err(RequestFormatError::EmptyRequest),
    }
}

fn validate_request(request: &str) -> Result<(Service, &str), RequestFormatError> {
    let service: Result<Service, RequestFormatError> = select_request_prefix_service(request);
    let method: Result<&str, RequestFormatError> = select_request_method(request);

    service.and_then(|s: Service| method.map(|m: &str| (s, m)))
}

#[derive(Clone, Debug, Default)]
pub struct ValidationMiddlewareMeta;
impl Metadata for ValidationMiddlewareMeta {}
#[derive(Default)]
pub struct ValidationMiddleware;
impl Middleware<ValidationMiddlewareMeta> for ValidationMiddleware {
    type Future = FutureResponse;
    type CallFuture = middleware::NoopCallFuture;

    fn on_request<F, X>(
        &self,
        request: Request,
        meta: ValidationMiddlewareMeta,
        next: F,
    ) -> Either<Self::Future, X>
    where
        F: Fn(Request, ValidationMiddlewareMeta) -> X + Send + Sync,
        X: Future<Output = Option<Response>> + Send + 'static,
    {
        // ToDo: rewrite
        let mut method_name = &"vpn.connection".to_string();
        if let Request::Single(call) = &request {
            if let jsonrpc_core::types::request::Call::MethodCall(call) = call {
                method_name = &call.method;
            }
        }

        match validate_request(method_name.as_str()) {
            Ok((_, _)) => Either::Left(Box::pin(next(request, meta))),
            Err(e) => Either::Left(Box::pin(async move {
                let response = Response::from(
                    Error {
                        code: ErrorCode::InternalError,
                        message: e.to_string(),
                        data: None,
                    },
                    Some(Version::V2),
                );
                Some(response)
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_request_prefix_service_returns_correct_service_type() {
        assert_eq!(
            Ok(Service::VPN),
            select_request_prefix_service("vpn.connect")
        );
        assert_eq!(Ok(Service::VPN), select_request_prefix_service("vpn."));
    }

    #[test]
    fn select_request_prefix_service_error_empty_request() {
        assert_eq!(
            Err(RequestFormatError::EmptyRequest),
            select_request_prefix_service("")
        );
    }

    #[test]
    fn select_request_prefix_service_error_service_not_provided() {
        assert_eq!(
            Err(RequestFormatError::ServiceNotProvided),
            select_request_prefix_service(".")
        );
        assert_eq!(
            Err(RequestFormatError::ServiceNotProvided),
            select_request_prefix_service(".connect")
        );
    }

    #[test]
    fn select_request_prefix_service_error_unknown_service() {
        assert_eq!(
            Err(RequestFormatError::UnknownService),
            select_request_prefix_service("depn.connect")
        );
    }

    #[test]
    fn select_request_method_returns_correct_method() {
        assert_eq!(Ok("connect"), select_request_method("vpn.connect"));
    }

    #[test]
    fn select_request_method_error_empty_request() {
        assert_eq!(
            Err(RequestFormatError::EmptyRequest),
            select_request_method("")
        );
    }

    #[test]
    fn select_request_method_error_method_not_provided() {
        assert_eq!(
            Err(RequestFormatError::MethodNotProvided),
            select_request_method(".")
        );
        assert_eq!(
            Err(RequestFormatError::MethodNotProvided),
            select_request_method("vpn.")
        );
    }
}
