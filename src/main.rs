extern crate futures;
extern crate hyper;
extern crate reqwest;
extern crate tokio_core;

mod myetcd;
mod enums;
mod etcds;

use std::io::{copy, stdout, Read, Write};
use std::time::Duration;
use tokio_core::reactor::Core;
use enums::{Env, Venture};
use etcds::EtcdEndpoint;
use reqwest::header::ContentType;
use reqwest::Response;

fn main() {
    let mut core = Core::new().unwrap();

    // check all ventures
    if true {
        let env = Env::Staging;
        let ventures = [
            Venture::ID,
            Venture::MY,
            Venture::PH,
            Venture::SG,
            Venture::TH,
            Venture::VN,
        ];
        
        let service = "notificator";
        // let service = "product_service";
        // let service = "image_storage_api";

        for &venture in &ventures {
            let service = get_service(&mut core, env, venture, service);
            healthcheck(&service);
        }
    }

    // stop/reset consumers
    if false {
        let env = Env::Staging;
        let venture = Venture::ID;
        let reset = true; // true - reset, false - stop

        let service = get_service(&mut core, env, venture, "notificator");
        healthcheck(&service);

        let queues = [
            "scapi.Product.createProducts",
            "scapi.Product.updateProducts",
            "scapi.Product.updatePrices",
            "scapi.Product.updateStock",
            "scapi.Product.updateImageCollection",
        ];
        for queue in &queues {
            notificator_maintenance_queue_v1(&service, queue, reset);
        }
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
        .build()
        .unwrap();
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

fn get_service(core: &mut Core, env: Env, venture: Venture, name: &str) -> Service {
    let etcd: Vec<EtcdEndpoint> = etcds::get_etcds()
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
struct Service {
    env: Env,
    venture: Venture,
    name: String,
    nodes: Vec<String>,
}
