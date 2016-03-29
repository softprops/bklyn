//! Bklyn is a query interface for the kubernetes cluster metric service [heapster](https://github.com/kubernetes/heapster).

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
use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::header::{Authorization, Basic, ContentLength};
use hyper::status::StatusCode;
use serde::de::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use url::form_urlencoded;

/// query options for fetching metric values
#[derive(Default)]
pub struct MetricOptions {
    params: HashMap<&'static str, String>,
}

impl MetricOptions {
    pub fn builder() -> MetricOptionsBuilder {
        MetricOptionsBuilder { ..Default::default() }
    }

    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

#[derive(Default)]
pub struct MetricOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl MetricOptionsBuilder {
    /// start timestamp, in RFC3339 format
    pub fn start<S>(&mut self, start: S) -> &mut MetricOptionsBuilder
        where S: Into<String>
    {
        self.params.insert("start", start.into());
        self
    }

    /// end timestamp, in RFC3339 format
    pub fn end<E>(&mut self, end: E) -> &mut MetricOptionsBuilder
        where E: Into<String>
    {
        self.params.insert("end", end.into());
        self
    }

    pub fn build(&self) -> MetricOptions {
        MetricOptions { params: self.params.clone() }
    }
}

/// Result type for bklyn query operations
pub type Result<T> = std::result::Result<T, Error>;

/// Credentials used for authenticating with kubernetes cluster
pub enum Credentials {
    Basic(String, String),
}

///  metric interface
pub struct Metrics<'a> {
    uri: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> Metrics<'a> {
    /// query availble metric names
    pub fn names(&self) -> Result<Vec<String>> {
        self.heapster.get::<Vec<String>>(&self.uri)
    }

    /// query recorded metric values
    pub fn values<M>(&self, metric: M, options: &MetricOptions) -> Result<Vec<Value>>
        where M: Into<String>
    {
        let mut uri = vec![format!("{}/{}", self.uri, metric.into())];
        if let Some(query) = options.serialize() {
            uri.push(query)
        }
        self.heapster
            .get::<MetricCollection>(&uri.join("?"))
            .map(|m| m.metrics)
    }
}

/// A node is essentially a host within a cluster
pub struct Node<'a> {
    name: String,
    heapster: &'a Heapster<'a>,
}

impl<'a> Node<'a> {
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: format!("/nodes/{}/metrics", self.name),
            heapster: self.heapster,
        }
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
    /// list metric names defined for this node container
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: format!("/nodes/{}/freecontainers/{}/metrics",
                         self.node,
                         self.container),
            heapster: self.heapster,
        }
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
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: format!("/namespaces/{}/pods/{}/metrics", self.namespace, self.pod),
            heapster: self.heapster,
        }
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
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: format!("/namespaces/{}/pods/{}/containers/{}/metrics",
                         self.namespace,
                         self.pod,
                         self.container),
            heapster: self.heapster,
        }
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
    /// list metric names defined for this namespace
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: format!("/namespaces/{}/metrics", self.name),
            heapster: self.heapster,
        }
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
    /// list metric names defined for this cluster
    pub fn metrics(&self) -> Metrics {
        Metrics {
            uri: "/metrics".to_owned(),
            heapster: self.heapster,
        }
    }

    /// query aggregate stats for cluster
    pub fn stats(&self) -> Result<Stats> {
        self.heapster.get::<Stats>("/stats")
    }

    /// list cluster nodes
    pub fn nodes(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>("/nodes")
    }

    /// return a query interface for a cluster node
    pub fn node<N>(&self, name: N) -> Node
        where N: Into<String>
    {
        Node {
            name: name.into(),
            heapster: self.heapster,
        }
    }

    /// list cluster namespaces
    pub fn namespaces(&self) -> Result<Vec<Summary>> {
        self.heapster.get::<Vec<Summary>>("/namespaces")
    }

    /// return a query interface for a cluster namespace
    pub fn namespace<N>(&self, name: N) -> Namespace
        where N: Into<String>
    {
        Namespace {
            name: name.into(),
            heapster: self.heapster,
        }
    }
}

/// Central interface for communicating kubernetes heapster service
pub struct Heapster<'a> {
    baseurl: String,
    credentials: Credentials,
    client: &'a Client,
}

impl<'a> Heapster<'a> {
    /// create a new heapster instance
    pub fn new<B>(baseurl: B, client: &'a Client, credentials: Credentials) -> Heapster<'a>
        where B: Into<String>
    {
        Heapster {
            baseurl: baseurl.into(),
            client: client,
            credentials: credentials,
        }
    }

    /// return a query interface for entire cluster
    pub fn cluster(&self) -> Cluster {
        Cluster { heapster: self }
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

    fn get<D>(&self, uri: &str) -> Result<D>
        where D: Deserialize
    {
        let builder = self.authenticate(Method::Get, uri);
        let mut res = try!(builder.send());
        let mut body = match res.headers.clone().get::<ContentLength>() {
            Some(&ContentLength(len)) => String::with_capacity(len as usize),
            _ => String::new(),
        };
        try!(res.read_to_string(&mut body));
        debug!("recv response {:#?} {:#?} {:#?}",
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
