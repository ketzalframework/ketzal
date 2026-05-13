pub mod http_exception;
pub use http_exception::HTTPException;

#[macro_export]
macro_rules! HTTPException {
    (
        status_code = $status:expr,
        detail = $detail:expr $(,)?
    ) => {{
        HTTPException::new($status, $detail).response()
    }};

    (
        status_code = $status:expr $(,)?
    ) => {{
        ketzal_http::HTTPException::new(
            $status,
            http::StatusCode::from_u16($status).unwrap().canonical_reason().unwrap_or("HTTP Error"),
        )
        .response()
    }};
}
