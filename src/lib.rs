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

/// Credentials used for authenticating with kubernetes cluster
pub enum Credentials {
    Basic(String, String),
}

/// A node is essentially a host within a cluster
pub struct Node<'a> {
    name: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> Node<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&format!("/nodes/{}/metrics", self.name))
    }

    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster
            .get::<Metrics>(&format!("/nodes/{}/metrics/{}", self.name, metric.into()))
            .map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>(&format!("/nodes/{}/stats", self.name))
    }

    pub fn pods(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>(&format!("/nodes/{}/pods", self.name))
    }

    pub fn freecontainers(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>(&format!("/nodes/{}/freecontainers", self.name))
    }

    pub fn freecontainer<C>(&self, container: C) -> FreeContainer
        where C: Into<String>
    {
        FreeContainer {
            node: self.name.clone(),
            container: container.into(),
            heapster: self.heapster,
        }
    }
}

/// Metrics associated with a container not bound to a specific pod
pub struct FreeContainer<'a> {
    node: String,
    container: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> FreeContainer<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&format!("/nodes/{}/freecontainers/{}/metrics",
                                                  self.node,
                                                  self.container))
    }

    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster
            .get::<Metrics>(&format!("/nodes/{}/freecontainers/{}/metrics/{}",
                                     self.node,
                                     self.container,
                                     metric.into()))
            .map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>(&format!("/nodes/{}/freecontainers/{}/stats",
                                            self.node,
                                            self.container))
    }
}

/// Metrics associated with a pod within a given namespace
pub struct NamespacePod<'a> {
    namespace: String,
    pod: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> NamespacePod<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&format!("/namespaces/{}/pods/{}/metrics",
                                                  self.namespace,
                                                  self.pod))
    }

    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster
            .get::<Metrics>(&format!("/namespaces/{}/pods/{}/metrics/{}",
                                     self.namespace,
                                     self.pod,
                                     metric.into()))
            .map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster
            .get::<Stats>(&format!("/namespaces/{}/pods/{}/stats", self.namespace, self.pod))
    }

    pub fn containers(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>(&format!("/namespaces/{}/pods/{}/containers",
                                                   self.namespace,
                                                   self.pod))
    }

    pub fn container<C>(&self, container: C) -> NamespacePodContainer
        where C: Into<String>
    {
        NamespacePodContainer {
            namespace: self.namespace.clone(),
            pod: self.pod.clone(),
            container: container.into(),
            heapster: self.heapster,
        }
    }
}

/// Metrics associated with a container, within a pod, within a namespace
pub struct NamespacePodContainer<'a> {
    namespace: String,
    pod: String,
    container: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> NamespacePodContainer<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&format!("/namespaces/{}/pods/{}/containers/{}/metrics",
                                                  self.namespace,
                                                  self.pod,
                                                  self.container))
    }

    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster
            .get::<Metrics>(&format!("/namespaces/{}/pods/{}/containers/{}/metrics/{}",
                                     self.namespace,
                                     self.pod,
                                     self.container,
                                     metric.into()))
            .map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>(&format!("/namespaces/{}/pods/{}/containers/{}/stats",
                                            self.namespace,
                                            self.pod,
                                            self.container))
    }
}

/// Metrics within a cluster namespace
pub struct Namespace<'a> {
    name: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> Namespace<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&format!("/namespaces/{}/metrics", self.name))
    }

    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster
            .get::<Metrics>(&format!("/namespace/{}/metrics/{}", self.name, metric.into()))
            .map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>(&format!("/namespaces/{}/stats", self.name))
    }

    pub fn pods(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>(&format!("/namespaces/{}/pods", self.name))
    }

    pub fn pod<N>(&self, name: N) -> NamespacePod
        where N: Into<String>
    {
        NamespacePod {
            namespace: self.name.clone(),
            pod: name.into(),
            heapster: self.heapster,
        }
    }
}

/// Metrics associated with a kubernetes cluster
pub struct Cluster<'a> {
    heapster: &'a Heapster<'a>,
}

impl<'a> Cluster<'a> {
    pub fn metrics(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>("/metrics")
    }

    // todo: support start/end
    pub fn values<M>(&self, metric: M) -> Result<Vec<Value>>
        where M: Into<String>
    {
        self.heapster.get::<Metrics>(&format!("/metrics/{}", metric.into())).map(|m| m.metrics)
    }

    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>("/stats")
    }
}

/// Central interface for communicating kubernetes heapster service
pub struct Heapster<'a> {
    baseurl: String,
    credentials: Credentials,
    client: &'a Client,
}

impl<'a> Heapster<'a> {
    pub fn new<B>(baseurl: B, client: &'a Client, credentials: Credentials) -> Heapster<'a>
        where B: Into<String>
    {
        Heapster {
            baseurl: baseurl.into(),
            client: client,
            credentials: credentials,
        }
    }

    pub fn cluster(&self) -> Cluster {
        Cluster { heapster: self }
    }

    pub fn nodes(&self) -> Result<Vec<Summary>> {
        self.get::<Vec<Summary>>("/nodes")
    }

    pub fn node<N>(&self, name: N) -> Node
        where N: Into<String>
    {
        Node {
            name: name.into(),
            heapster: self,
        }
    }

    pub fn namespaces(&self) -> Result<Vec<Summary>> {
        self.get::<Vec<Summary>>("/namespaces")
    }

    pub fn namespace<N>(&self, name: N) -> Namespace
        where N: Into<String>
    {
        Namespace {
            name: name.into(),
            heapster: self,
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
                self.client.request(method, &url).header(Authorization(Basic {
                    username: user.to_owned(),
                    password: Some(password.to_owned()),
                }))
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
            StatusCode::Forbidden => Err(Error::Fault { code: res.status }),
            _ => Ok(try!(serde_json::from_str::<D>(&body))),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
