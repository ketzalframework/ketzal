#[macro_export]
macro_rules! HTTPException {
    (
        status_code = $status:expr,
        detail = $detail:expr $(,)?
    ) => {{
        $crate::HTTPException::new($status, $detail).response()
    }};

    (
        status_code = $status:expr $(,)?
    ) => {{
        $crate::HTTPException::new(
            $status,
            $crate::http::StatusCode::from_u16($status)
                .unwrap()
                .canonical_reason()
                .unwrap_or("HTTP Error"),
        )
        .response()
    }};
}
