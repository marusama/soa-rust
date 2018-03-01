extern crate futures;
extern crate hyper;
extern crate reqwest;
extern crate tokio_core;

mod enums;
mod etcds;
mod service;
mod myetcd;

use std::io::Read;
use std::time::Duration;
use tokio_core::reactor::Core;
use enums::{Env, Venture};
use service::Service;
use reqwest::header::ContentType;
use reqwest::Response;

fn main() {
    let mut core = Core::new().unwrap();

    // check all ventures
    healthcheck_all_ventures(&mut core, Env::Live, "product_service");
    healthcheck_all_ventures(&mut core, Env::Live, "image_storage_api");
    healthcheck_all_ventures(&mut core, Env::Live, "notificator");
    healthcheck_all_ventures(&mut core, Env::Live, "sellercenter_api");
    healthcheck_all_ventures(&mut core, Env::Live, "seller_api");

    // stop/reset consumers
    // notificator_stop_reset_consumers(&mut core, Env::Staging, Venture::ID, true);
}

fn healthcheck_all_ventures(core: &mut Core, env: Env, service: &str) {
    let ventures = [
        Venture::ID,
        Venture::MY,
        Venture::PH,
        Venture::SG,
        Venture::TH,
        Venture::VN,
    ];

    for &venture in &ventures {
        let service = service::get_service(core, env, venture, service);
        service.healthcheck();
    }
}

fn notificator_stop_reset_consumers(core: &mut Core, env: Env, venture: Venture, reset: bool) {
    let service = service::get_service(core, env, venture, "notificator");
    service.healthcheck();

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
