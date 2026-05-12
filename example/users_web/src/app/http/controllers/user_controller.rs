use ketzal::{controller, validate_form, Request, Response};

#[controller("/users")]
impl UserController {
    #[post("/login")]
    pub async fn login(_req: Request) -> Response {
        Response::ok("Hello, World!")
    }

    #[post("/")]
    pub async fn store(req: Request) -> Response {
        let validated = validate_form!(req => {
            "name"     => "required|string|max:255",
            "email"    => "required|email",
            "password" => "required|min:8|confirmed",
        });

        let safe = validated.except(["password"]).all();
        Response::json(safe)
    }

    #[get("/:id")]
    pub async fn show(id: i32) -> Response {
        Response::ok(format!("Showing user with id: {}", id))
    }
}
