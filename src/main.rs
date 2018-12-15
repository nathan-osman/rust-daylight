// The MIT License (MIT)
//
// Copyright (c) 2018 Nathan Osman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate sunrise;

use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

#[derive(Deserialize)]
struct Parameters {
    latitude: f64,
    longitude: f64,
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Serialize)]
struct Response {
    sunrise: i64,
    sunset: i64,
}

#[post("/api", format = "json", data = "<parameters>")]
fn api(parameters: Json<Parameters>) -> Json<Response> {
    let (sunrise, sunset) = sunrise::sunrise_sunset(
        parameters.latitude,
        parameters.longitude,
        parameters.year,
        parameters.month,
        parameters.day,
    );
    Json(Response {
        sunrise: sunrise,
        sunset: sunset,
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![api])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
