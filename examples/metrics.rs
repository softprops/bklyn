extern crate bklyn;
extern crate hyper;

use bklyn::{Credentials, Heapster};
use hyper::Client;

fn main() {
    let client = Client::new();
    let heapster = Heapster::new(
        env!("HEAPSTER_BASEURL"),
        &client,
        Credentials::Basic(
            env!("HEAPSTER_USER").to_owned(),
            env!("HEAPSTER_PASSWORD").to_owned()
                )
            );
    println!("metrics {:#?}", heapster.cluster().metrics().unwrap());
}
