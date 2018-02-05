extern crate futures;
extern crate hyper;
extern crate reqwest;
extern crate tokio_core;

mod myetcd;
mod enums;

use std::io::{self, copy, stdout, Read, Write};
use std::time::Duration;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use enums::{Env, Venture};
use reqwest::header::{ContentType, Headers, UserAgent};
use reqwest::Response;

fn main() {
    let mut core = Core::new().unwrap();

    let service = get_service(&mut core, Env::Live, Venture::SG, "notificator");

    healthcheck(&service);

    let queues = vec![
        "scapi.Product.createProducts",
        "scapi.Product.updateProducts",
        "scapi.Product.updatePrices",
        "scapi.Product.updateStock",
        "scapi.Product.updateImageCollection",
    ];
    for queue in queues {
        // notificator_maintenance_queue_v1(&service, queue, false); // stop consumers
        notificator_maintenance_queue_v1(&service, queue, true); // reset consumers
    }
}

fn healthcheck(service: &Service) {
    println!("healthchecking...");
    for n in &service.nodes {
        let url = n.to_owned() + "/health_check";
        println!("GET {:?}", url);
        let mut response = reqwest::get(&url).unwrap();
        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("Failed to read response");
        println!("{}", buf);
    }
}

fn notificator_maintenance_queue_v1(notificator: &Service, queue: &str, reset: bool) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build().unwrap();
    for n in &notificator.nodes {
        let url = n.to_owned() + "/rpc";

        let body = if reset {
            r#"{
                "jsonrpc": "2.0",
                "method": "/maintenance/queue/v1",
                "params": {
                    "config": [
                        {
                            "queue": ""#.to_owned() + queue
                + r#""
                        }
                    ],
                    "reset_config": true
                },
                "id": 1
            }"#
        } else {
            r#"{
                "jsonrpc": "2.0",
                "method": "/maintenance/queue/v1",
                "params": {
                    "config": [
                        {
                            "queue": ""#.to_owned() + queue
                + r#"",
                            "number-worker": 0,
                            "prefetch-count": 0
                        }
                    ]
                },
                "id": 1
            }"#
        };

        println!("POST {:?}: {}", url, body);

        let mut response: Response = client
            .post(&url)
            .body(body)
            .header(ContentType::json())
            .send()
            .unwrap();
        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("Failed to read response");
        println!("{}", buf);
    }
}

fn get_etcds() -> Vec<EtcdEndpoint> {
    let mut etcds = Vec::new();

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::ID,
        "http://idlzdliveaero1-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::ID,
        "http://idlzdliveaero2-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::ID,
        "http://idlzdliveaero3-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::ID,
        "http://idlzdliveaero4-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::ID,
        "http://idlzdliveaero5-pub:2379",
    ));

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::MY,
        "http://sg1n-srv-01096.lzd.io:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::MY,
        "http://sg1n-srv-01137.lzd.io:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::MY,
        "http://sg1n-srv-01179.lzd.io:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::MY,
        "http://sg1n-srv-01221.lzd.io:2379",
    ));

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::PH,
        "http://phlzdliveaero1-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::PH,
        "http://phlzdliveaero2-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::PH,
        "http://phlzdliveaero3-pub:2379",
    ));

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::SG,
        "http://sglzdliveaero1-pub.sgdc:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::SG,
        "http://sglzdliveaero2-pub.sgdc:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::SG,
        "http://sglzdliveaero3-pub.sgdc:2379",
    ));

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::TH,
        "http://thlzdliveaero1-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::TH,
        "http://thlzdliveaero2-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::TH,
        "http://thlzdliveaero3-pub:2379",
    ));

    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::VN,
        "http://vnlzdliveaero4-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::VN,
        "http://vnlzdliveaero5-pub:2379",
    ));
    etcds.push(EtcdEndpoint::new(
        Env::Live,
        Venture::VN,
        "http://vnlzdliveaero6-pub:2379",
    ));

    etcds
}

fn get_service(core: &mut Core, env: Env, venture: Venture, name: &str) -> Service {
    let etcd: Vec<EtcdEndpoint> = get_etcds()
        .into_iter()
        .filter(|x| x.env == env && x.venture == venture)
        .collect();
    let key = format!(
        "/lazada_api/{}/{}/{}/nodes",
        venture.to_string().to_lowercase(),
        env.to_string().to_lowercase(),
        name
    );
    let etcd_node = &etcd[0].node;
    let nodes = myetcd::get_values_from_etcd(core, etcd_node, &key);
    Service {
        env: env,
        venture: venture,
        name: name.to_owned(),
        nodes: nodes.unwrap(),
    }
}

#[derive(Debug)]
struct EtcdEndpoint {
    env: Env,
    venture: Venture,
    node: String,
}

impl EtcdEndpoint {
    fn new(env: Env, venture: Venture, node: &str) -> EtcdEndpoint {
        EtcdEndpoint {
            env: env,
            venture: venture,
            node: node.to_owned(),
        }
    }
}

#[derive(Debug)]
struct Service {
    env: Env,
    venture: Venture,
    name: String,
    nodes: Vec<String>,
}
