#![feature(proc_macro_hygiene, decl_macro)]

use js_typify_gostruct;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::response::status::BadRequest;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct TransformRequest {
    contents: String,
}

#[post("/gostruct/to/flow", format = "json", data = "<data>")]
fn transform_go_struct_to_flow(
    data: Json<TransformRequest>,
) -> Result<JsonValue, BadRequest<JsonValue>> {
    match js_typify_gostruct::transform(data.contents.to_string()) {
        Ok(res) => Ok(json!({ "data": res })),
        Err(parse_error) => Err(BadRequest(Some(json!({
            "status": "error",
            "reason": format!("{:?}", parse_error)
        })))),
    }
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
        .mount("/api/v1", routes![index, transform_go_struct_to_flow])
        .register(catchers![not_found])
        .launch();
}
