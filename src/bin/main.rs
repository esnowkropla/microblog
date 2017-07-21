extern crate iron;
extern crate router;
//extern crate staticfile;

use iron::prelude::Chain;
use iron::headers::ContentType;
use iron::{Iron, Handler, status, IronResult, Response, Request};
use router::Router;

macro_rules! get_http_param {
    ( $r: expr, $e: expr ) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(val) => val,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            }
            None => return Ok(Response::with(status::InternalServerError)),
        }
    }
}

struct IndexHandler;

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = r"<h1>Hello!</h1>";
        let mut response = Response::with((status::Ok, payload));
        response.headers.set(ContentType::html());
        return Ok(response);
    }
}

struct UserHandler;

impl Handler for UserHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let user = get_http_param!(req, "name");
        Ok(Response::with(
            (status::Ok, format!("Hello <i>{}</i>", user)),
        ))
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", IndexHandler, "index");
    router.get("/user/:name", UserHandler, "user");
    //router.get("/feed", handlers.feed, "feed");
    //router.post("/post", handlers.make_post, "make_post");
    //router.get("/post/:id", handlers.post, "post");
    //router.get("/", Static::new(Path::new("static/index.html")), "home");
    //router.get("/*", Static::new(Path::new("static/")), "static");

    let chain = Chain::new(router);

    Iron::new(chain).http("localhost:3000").unwrap();
}
