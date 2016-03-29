extern crate bklyn;
extern crate hyper;

use bklyn::{Credentials, Heapster};
use hyper::Client;
use std::env;
use std::time::Duration;

fn main() {
    if let (Ok(baseurl), Ok(user), Ok(password)) = (
        env::var("HEAPSTER_BASEURL"),
        env::var("HEAPSTER_USER"),
        env::var("HEAPSTER_PASSWORD")
    ) {
        let mut client = Client::new();
        client.set_read_timeout(Some(Duration::from_secs(2)));
        let heapster = Heapster::new(
            baseurl,
            &client,
            Credentials::Basic(
                user,
                password
            )
        );
        if let Ok(names) = heapster.cluster().metrics().names() {
            for metric in names {
                println!(
                    "{:#?} metrics {:#?}",
                    metric,
                    heapster.cluster().metrics().values(metric.clone(), &Default::default())
                );
            }
        }

    }
}
