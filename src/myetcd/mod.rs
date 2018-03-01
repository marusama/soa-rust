extern crate etcd;
extern crate futures;
extern crate tokio_core;

mod myetcd_error;

use std::vec;
use self::etcd::Client;
use self::etcd::kv;
use self::futures::Future;
use self::tokio_core::reactor::Core;

pub fn get_values_from_etcd(
    core: &mut Core,
    endpoint: &str,
    key: &str,
) -> Result<vec::Vec<String>, myetcd_error::MyEtcdError> {
    let client = Client::new(&core.handle(), &[endpoint], None)?;

    let mut result = vec![];

    {
        let req = kv::get(&client, key, kv::GetOptions::default()).and_then(|resp| {
            let data: etcd::kv::KeyValueInfo = resp.data;
            if let Some(nodes) = data.node.nodes {
                for node in nodes {
                    if let Some(val) = node.value {
                        result.push(val.to_string());
                    }
                }
            }
            Ok(())
        });
        core.run(req).unwrap();
    }

    Ok(result)
}
