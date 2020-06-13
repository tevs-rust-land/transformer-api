#![feature(proc_macro_hygiene, decl_macro)]

use js_typify_gostruct;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct TransformRequest {
    contents: String,
}

#[post("/go/to/flow", format = "json", data = "<data>")]
fn transform_go_struct_to_flow(data: Json<TransformRequest>) -> JsonValue {
    match js_typify_gostruct::transform(data.contents.to_string()) {
        Ok(res) => json!({ "data": res }),
        Err(parse_error) => json!({
            "status": "error",
            "reason": format!("{:?}", parse_error)
        }),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/api/v1", routes![index, transform_go_struct_to_flow])
        .launch();
}
