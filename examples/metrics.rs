extern crate bklyn;
extern crate hyper;

use bklyn::{Credentials, Heapster};
use hyper::Client;
use std::env;

fn main() {
    if let (Ok(baseurl), Ok(user), Ok(password)) = (
        env::var("HEAPSTER_BASEURL"),
        env::var("HEAPSTER_USER"),
        env::var("HEAPSTER_PASSWORD")
    ) {
        let client = Client::new();
        let heapster = Heapster::new(
            baseurl,
            &client,
            Credentials::Basic(
                user,
                password
            )
        );
        println!("metrics {:#?}", heapster.cluster().values("cpu-usage"));
    }
}
