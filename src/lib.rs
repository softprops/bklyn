#[macro_use]
extern crate serializable_enum;
#[macro_use]
extern crate log;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

mod rep;
mod errors;

pub use rep::*;
pub use errors::*;
use serde::de::Deserialize;
use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::header::{Authorization, Basic, ContentLength};
use hyper::status::StatusCode;
use std::io::Read;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Credentials {
    Basic(String, String)
}

pub struct Cluster<'a> {
    heapster: &'a Heapster<'a>
}

impl<'a> Cluster<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>("/metrics")
    }

    // todo: support start/end
    pub fn values<N>(&self, name: N) -> Result<Vec<Value>> where N: Into<String> {
        self.heapster.get::<Metrics>(&format!("/metrics/{}", name.into())).map(|m| m.metrics)
    }
}

pub struct Heapster<'a> {
    baseurl: String,
    credentials: Credentials,
    client: &'a Client
}

impl<'a> Heapster<'a> {
    pub fn new<B>(baseurl: B, client: &'a Client, credentials: Credentials) -> Heapster<'a>
        where B: Into<String>
    {
        Heapster {
            baseurl: baseurl.into(),
            client: client,
            credentials: credentials
        }
    }

    pub fn cluster(&self) -> Cluster {
        Cluster {
            heapster: self
        }
    }

    fn get<D>(&self, uri: &str) -> Result<D>
        where D: Deserialize
    {
        self.request(Method::Get, uri, None)
    }

    fn authenticate(&self, method: Method, uri: &str) -> RequestBuilder {
        let url = format!("{}/api/v1/model{}", self.baseurl, uri);
        match self.credentials {
            Credentials::Basic(ref user, ref password) => {
                self.client.request(method, &url).header(Authorization(Basic { username: user.to_owned(), password: Some(password.to_owned())}))
            }
        }
    }

    fn request<D>(&self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<D>
        where D: Deserialize
    {
        let builder = self.authenticate(method, uri);
        let mut res = try!(match body {
            Some(ref bod) => builder.body(*bod).send(),
            _ => builder.send(),
        });
        let mut body = match res.headers.clone().get::<ContentLength>() {
            Some(&ContentLength(len)) => String::with_capacity(len as usize),
            _ => String::new(),
        };
        try!(res.read_to_string(&mut body));
        debug!("rev response {:#?} {:#?} {:#?}",
               res.status,
               res.headers,
               body);
        match res.status {
            StatusCode::Conflict |
            StatusCode::BadRequest |
            StatusCode::UnprocessableEntity |
            StatusCode::Unauthorized |
            StatusCode::NotFound |
            StatusCode::Forbidden => {
                Err(Error::Fault {
                    code: res.status
                })
            }
            _ => Ok(try!(serde_json::from_str::<D>(&body))),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
