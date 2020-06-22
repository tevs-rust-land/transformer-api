#![feature(proc_macro_hygiene, decl_macro)]

use js_typify_gostruct;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::http::Method;

use rocket::response::status::BadRequest;
use rocket_contrib::json::{Json, JsonValue};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header};
use rocket::{Request, Response};
use std::io::Cursor;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TransformRequest {
    contents: String,
}

#[post("/gostruct/to/flow", format = "json", data = "<data>")]
fn transform_go_struct_to_flow(
    data: Json<TransformRequest>,
) -> Result<JsonValue, BadRequest<JsonValue>> {
    match js_typify_gostruct::transform_to_flow(data.contents.to_string()) {
        Ok(res) => Ok(json!({ "data": res })),
        Err(parse_error) => Err(BadRequest(Some(json!({
            "status": "error",
            "reason": format!("{:?}", parse_error)
        })))),
    }
}

#[post("/gostruct/to/typescript", format = "json", data = "<data>")]
fn transform_go_struct_to_typescript(
    data: Json<TransformRequest>,
) -> Result<JsonValue, BadRequest<JsonValue>> {
    match js_typify_gostruct::transform_to_typescript(data.contents.to_string()) {
        Ok(res) => Ok(json!({ "data": res })),
        Err(parse_error) => Err(BadRequest(Some(json!({
            "status": "error",
            "reason": format!("{:?}", parse_error)
        })))),
    }
}

#[options("/gostruct/to/flow")]
fn optionflowroute() -> &'static str {
    "Hello, world!"
}

#[options("/gostruct/to/typescript")]
fn optiontypescriptroute() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    json!({
        "status": "error",
        "reason": format!("Sorry, '{}' is not a valid path.", req.uri())
    })
}

fn main() {
    rocket::ignite()
        .mount(
            "/api/v1",
            routes![
                index,
                transform_go_struct_to_flow,
                transform_go_struct_to_typescript,
                optionflowroute,
                optiontypescriptroute
            ],
        )
        .register(catchers![not_found])
        .attach(CORS())
        .launch();
}
