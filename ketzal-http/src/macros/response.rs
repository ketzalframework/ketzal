#[macro_export]
macro_rules! Response {

    // Response!({
    //     content = {
    //         "message": "Hello"
    //     }
    // })

    ({
        content = {
            $($key:literal : $value:expr),* $(,)?
        }
        $(,)?
    }) => {{

        return $crate::Response::json(

            $crate::serde_json::json!({
                $($key: $value),*
            })

        );

    }};

    // Response!({
    //     content = {
    //         "message": "Created"
    //     },
    //     status_code = 201
    // })

    ({
        content = {
            $($key:literal : $value:expr),* $(,)?
        },

        status_code = $status:expr $(,)?
    }) => {{

        return $crate::Response::json_with_status(

            $crate::http::StatusCode::from_u16($status)
                .unwrap_or($crate::http::StatusCode::OK),

            $crate::serde_json::json!({
                $($key: $value),*
            })

        );

    }};
}
