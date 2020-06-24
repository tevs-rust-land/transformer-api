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
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use rocket::Request;

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

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    json!({
        "status": "error",
        "reason": format!("Sorry, '{}' is not a valid path.", req.uri())
    })
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::ignite()
        .mount(
            "/api/v1",
            routes![
                transform_go_struct_to_flow,
                transform_go_struct_to_typescript
            ],
        )
        .register(catchers![not_found])
        .attach(cors)
        .launch();

    Ok(())
}
