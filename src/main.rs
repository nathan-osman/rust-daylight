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

extern crate bodyparser;
extern crate ctrlc;
extern crate iron;
extern crate mount;
extern crate router;
extern crate serde_json;
extern crate staticfile;
extern crate sunrise;

#[macro_use]
extern crate serde_derive;

use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

use mount::Mount;
use router::Router;
use staticfile::Static;

#[derive(Clone, Deserialize)]
struct Parameters {
    latitude: f64,
    longitude: f64,
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Serialize)]
struct Result {
    sunrise: i64,
    sunset: i64,
}

fn wait_for_signal() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let copy = pair.clone();

    ctrlc::set_handler(move || {
        let &(ref lock, ref cvar) = &*copy;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    })
    .expect("unable to set signal handler");

    let &(ref lock, ref cvar) = &*pair;

    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
}

fn api(req: &mut Request) -> IronResult<Response> {
    match req.get::<bodyparser::Struct<Parameters>>() {
        Ok(Some(parameters)) => {
            let content_type = "application/json".parse::<Mime>().unwrap();
            let (sunrise, sunset) = sunrise::sunrise_sunset(
                parameters.latitude,
                parameters.longitude,
                parameters.year,
                parameters.month,
                parameters.day,
            );
            let result = Result {
                sunrise: sunrise,
                sunset: sunset,
            };
            Ok(Response::with((
                content_type,
                status::Ok,
                serde_json::to_string(&result).unwrap(),
            )))
        }
        _ => Ok(Response::with(status::BadRequest)),
    }
}

fn main() {
    let mut router = Router::new();
    router.post("/", api, "api");

    let mut mount = Mount::new();
    mount
        .mount("/api", router)
        .mount("/", Static::new(Path::new("static")));

    let mut listening = Iron::new(mount).http("0.0.0.0:8000").unwrap();
    wait_for_signal();
    listening.close().unwrap();
}
