extern crate etcd;

use std::io;

#[derive(Debug)]
pub enum MyEtcdError {
    Etcd(etcd::Error),
    Io(io::Error)
}

impl From<etcd::Error> for MyEtcdError {
    fn from(err: etcd::Error) -> MyEtcdError {
        MyEtcdError::Etcd(err)
    }
}

impl From<io::Error> for MyEtcdError {
    fn from(err: io::Error) -> MyEtcdError {
        MyEtcdError::Io(err)
    }
}
