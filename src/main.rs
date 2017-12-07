extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod myetcd;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let mut services = get_staging_product_services();

    println!("filling nodes...");
    fill_nodes(&mut core, &mut services);

    println!("healthchecking...");
    let client = hyper::Client::new(&core.handle());
    for s in &services {
        for n in &s.nodes {
            let uri: hyper::Uri = n.parse().unwrap();
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
