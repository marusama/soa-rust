extern crate reqwest;

use std::io::Read;
use tokio_core::reactor::Core;
use enums::{Env, Venture};
use etcds::{get_etcds, EtcdEndpoint};
use myetcd::get_values_from_etcd;

#[derive(Debug)]
pub struct Service {
    env: Env,
    venture: Venture,
    name: String,
    pub nodes: Vec<String>,
}

pub fn get_service(core: &mut Core, env: Env, venture: Venture, name: &str) -> Service {
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
    let nodes = get_values_from_etcd(core, etcd_node, &key);
    Service {
        env: env,
        venture: venture,
        name: name.to_owned(),
        nodes: nodes.unwrap(),
    }
}

impl Service {
    pub fn healthcheck(self: &Service) {
        println!("healthchecking...");
        for n in &self.nodes {
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
}
