# bklyn

[![Build Status](https://travis-ci.org/softprops/bklyn.svg?branch=master)](https://travis-ci.org/softprops/bklyn) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

> collect artisanal kubernetes cluster metrics from rust

Bklyn is a [rust](https://www.rust-lang.org/) interface querying [heapster](https://github.com/kubernetes/heapster).

## api docs

Find them [here](https://softprops.github.io/bklyn)

## usage

```rust
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
```

Doug Tangren (softprops) 2016
