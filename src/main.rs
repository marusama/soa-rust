extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod myetcd;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Request, Method};
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    //let mut services = get_staging_product_services();
    let mut services = get_my_live_notificator_services();

    println!("filling nodes...");
    fill_nodes(&mut core, &mut services);

    println!("healthchecking...");
    let client = hyper::Client::new(&core.handle());
    for s in &services {
        for n in &s.nodes {
            let url = n.to_owned() + "/health_check";
            let uri: hyper::Uri = url.parse().unwrap();
            println!("uri: {:?}", uri);
            let work = client.get(uri).and_then(|resp| {
                println!("Response: {}", resp.status());

                resp.body().for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(From::from)
                })
            });
            core.run(work).unwrap();
        }
    }

    //
    for s in &services {
        for n in &s.nodes {
            let url = n.to_owned() + "/rpc";
            let uri: hyper::Uri = url.parse().unwrap();
            let body = r#"{"jsonrpc": "2.0", "id": "debug","method": "/maintenance/queue/v1", "params": {"reset_config": true}}"#;
            println!("uri: {:?}, body: {}", uri, body);

            let mut req = Request::new(Method::Post, uri);
            req.set_body(body);
            req.headers_mut().set_raw("Content-Length", format!("{}", body.len()));
            req.headers_mut().set_raw("Content-Type", "application/json");

            let work = client.request(req).and_then(|resp| {
                println!("Response: {}", resp.status());

                resp.body().for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(From::from)
                })
            });

            core.run(work).unwrap();
        }
    }
}

fn get_staging_product_services() -> std::vec::Vec<Service> {
    vec![
        Service {
            name: "product_service".to_owned(),
            venture: "my".to_owned(),
            env: "staging".to_owned(),
            etcd_endpoint: "http://mylzdstgaero3-pub.sgdc:2379".to_owned(),
            nodes: std::vec::Vec::new()
        }
    ]
}

fn get_live_product_services() -> std::vec::Vec<Service> {
    vec![
        Service {
            name: "product_service".to_owned(),
            venture: "id".to_owned(),
            env: "live".to_owned(),
            etcd_endpoint: "http://mylzdstgaero3-pub.sgdc:2379".to_owned(),
            nodes: std::vec::Vec::new()
        }
    ]
}

fn get_my_live_notificator_services() -> std::vec::Vec<Service> {
    vec![
        Service {
            name: "notificator".to_owned(),
            venture: "my".to_owned(),
            env: "live".to_owned(),
            etcd_endpoint: "http://sg1n-srv-01096.lzd.io:2379".to_owned(),
            nodes: std::vec::Vec::new()
        }
    ]
}

fn fill_nodes(core: &mut Core, services: &mut Vec<Service>) {
    for s in services {
        let key = s.get_key();
        let nodes = myetcd::get_values_from_etcd(core, &s.etcd_endpoint, &key);
        s.nodes = nodes.unwrap();
        println!("{:?}", s.nodes)
    }
}

#[derive(Debug)]
struct Service {
    name: String,
    env: String,
    venture: String,
    etcd_endpoint: String,
    nodes: std::vec::Vec<String>
}

impl Service {
    fn get_key(&self) -> String {
        format!("/lazada_api/{}/{}/{}/nodes", self.venture, self.env, self.name)
    }
}
