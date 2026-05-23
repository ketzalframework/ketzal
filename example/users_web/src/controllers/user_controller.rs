use ketzal::{controller, HTTPException, Request, Response};

#[controller("/users")]
impl UserController {
    //#[post("/login")]
    //pub async fn login(_req: Request) -> Response {
    //   Response::ok("Hello, World!")
    //}
    #[get("/")]
    pub async fn index() -> Response {
        Response!({ content = {"message":"uses"} });
    }

    #[post("/")]
    pub async fn store(req: Request) -> Response {
        Response!({
            content = {
                "path": req.path,
                "method": req.method.as_str()
            }
        });
    }

    #[get("/:id")]
    pub async fn show(id: i32) -> Response {
        if id == 0 {
            return HTTPException!(status_code = 404, detail = "User not found");
        }
        Response!({
            content = {
                "message": id,
                "token": "jwt-token"
            }
        });
    }
}
