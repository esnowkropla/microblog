extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate handlebars_iron;

extern crate microblog;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use handlebars_iron::{HandlebarsEngine, DirectorySource, Watchable, Template};
use iron::prelude::Chain;
use iron::headers::{ContentType, Location};
use iron::{Set, Iron, Handler, status, IronResult, Response, Request};
use router::Router;
use staticfile::Static;

use std::path::Path;
use std::sync::Arc;

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

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    id: usize,
}

struct IndexHandler;

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let x = User {
            name: String::from("Fred"),
            id: 13,
        };
        let template = Template::new("index", x);
        let mut response = Response::with(status::Ok);
        response.headers.set(ContentType::html());
        response.set_mut(template);
        return Ok(response);
    }
}

struct UserHandler;

impl Handler for UserHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let user = get_http_param!(req, "name");
        let mut response = Response::with((status::Ok, format!("Hello <i>{}</i>", user)));
        response.headers.set(ContentType::html());
        return Ok(response);
    }
}

struct Custom404;

impl Handler for Custom404 {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        return Ok(Response::with((status::NotFound, "Custom 404")));
    }
}

struct PostHandler;

impl Handler for PostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut resp = Response::new();
        resp.set_mut(status::Found);
        let url = url_for!(req, "index");
        println!("{}", url);
        resp.headers.set(Location(format!("{}", url)));
        return Ok(resp);
    }
}

fn main() {
    let views_ext = ".hbs";
    let views_path = "./templates/";
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new(views_path, views_ext)));
    if let Err(e) = hbse.reload() {
        panic!("{:?}", e.cause);
    }
    let hbse_ref = Arc::new(hbse);
    hbse_ref.watch(views_path);

    let mut router = Router::new();
    router.get("/", IndexHandler, "index");
    router.post("/", PostHandler, "post");
    router.get("/user/:name", UserHandler, "user");
    //router.get("/feed", handlers.feed, "feed");
    //router.post("/post", handlers.make_post, "make_post");
    //router.get("/post/:id", handlers.post, "post");
    //router.get("/", Static::new(Path::new("static/index.html")), "home");
    router.get("/*", Static::new(Path::new("static/")), "static");
    //router.get("*", Custom404, "404");

    let mut chain = Chain::new(router);
    chain.link_after(hbse_ref);

    Iron::new(chain).http("localhost:3000").unwrap();
}
