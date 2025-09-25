pub mod conn;
pub mod grpc;
pub mod proxy_connector;
pub mod retry;

pub fn create_request<T>(param_string: String, into_request: impl grpc::IntoRequest<T>) -> grpc::Request<T> {
    let mut request = into_request.into_request();
    let target = request.metadata_mut();
    if !param_string.is_empty() {
        target.append("x-goog-request-params", param_string.parse().unwrap());
    }
    tracing::debug!("USING CUSTOM authority");
    target.append("host", "pubsub.googleapis.com".parse().unwrap());
    request
}
