use enums::{Env, Venture};

#[derive(Debug)]
pub struct EtcdEndpoint {
    pub env: Env,
    pub venture: Venture,
    pub node: String,
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

pub fn get_etcds() -> Vec<EtcdEndpoint> {
    let mut etcds = Vec::new();

    push(&mut etcds, Env::Live, Venture::ID, &LIVE_ID_NODES);
    push(&mut etcds, Env::Live, Venture::MY, &LIVE_MY_NODES);
    push(&mut etcds, Env::Live, Venture::PH, &LIVE_PH_NODES);
    push(&mut etcds, Env::Live, Venture::SG, &LIVE_SG_NODES);
    push(&mut etcds, Env::Live, Venture::TH, &LIVE_TH_NODES);
    push(&mut etcds, Env::Live, Venture::VN, &LIVE_VN_NODES);

    push(&mut etcds, Env::Staging, Venture::ID, &STAGING_ID_NODES);
    push(&mut etcds, Env::Staging, Venture::MY, &STAGING_MY_NODES);
    push(&mut etcds, Env::Staging, Venture::PH, &STAGING_PH_NODES);
    push(&mut etcds, Env::Staging, Venture::SG, &STAGING_SG_NODES);
    push(&mut etcds, Env::Staging, Venture::TH, &STAGING_TH_NODES);
    push(&mut etcds, Env::Staging, Venture::VN, &STAGING_VN_NODES);

    etcds
}

fn push(etcds: &mut Vec<EtcdEndpoint>, env: Env, venture: Venture, nodes: &[&'static str]) {
    for &node in nodes {
        etcds.push(EtcdEndpoint::new(env, venture, node));
    }
}

const LIVE_ID_NODES: [&str; 5] = [
    "http://idlzdliveaero1-pub.iddc:2379",
    "http://idlzdliveaero2-pub.iddc:2379",
    "http://idlzdliveaero3-pub.iddc:2379",
    "http://idlzdliveaero4-pub.iddc:2379",
    "http://idlzdliveaero5-pub.iddc:2379",
];

const LIVE_MY_NODES: [&str; 4] = [
    "http://sg1n-srv-01096.lzd.io:2379",
    "http://sg1n-srv-01137.lzd.io:2379",
    "http://sg1n-srv-01179.lzd.io:2379",
    "http://sg1n-srv-01221.lzd.io:2379",
];

const LIVE_PH_NODES: [&str; 3] = [
    "http://phlzdliveaero1-pub.hkdc:2379",
    "http://phlzdliveaero2-pub.hkdc:2379",
    "http://phlzdliveaero3-pub.hkdc:2379",
];

const LIVE_SG_NODES: [&str; 3] = [
    "http://sglzdliveaero1-pub.sgdc:2379",
    "http://sglzdliveaero2-pub.sgdc:2379",
    "http://sglzdliveaero3-pub.sgdc:2379",
];

const LIVE_TH_NODES: [&str; 3] = [
    "http://thlzdliveaero1-pub.sgdc:2379",
    "http://thlzdliveaero2-pub.sgdc:2379",
    "http://thlzdliveaero3-pub.sgdc:2379",
];

const LIVE_VN_NODES: [&str; 3] = [
    "http://vnlzdliveaero4-pub.sgdc:2379",
    "http://vnlzdliveaero5-pub.sgdc:2379",
    "http://vnlzdliveaero6-pub.sgdc:2379",
];

const STAGING_ID_NODES: [&str; 3] = [
    "http://idlzdstgaero3-pub.iddc:2379",
    "http://idlzdstgaero4-pub.iddc:2379",
    "http://idlzdstgaero5-pub.iddc:2379",
];

const STAGING_MY_NODES: [&str; 3] = [
    "http://mylzdstgaero1-pub.sgdc:2379",
    "http://mylzdstgaero2-pub.sgdc:2379",
    "http://mylzdstgaero3-pub.sgdc:2379",
];

const STAGING_PH_NODES: [&str; 3] = [
    "http://phlzdstgaero1-pub.sgdc:2379",
    "http://phlzdstgaero2-pub.sgdc:2379",
    "http://phlzdstgaero3-pub.sgdc:2379",
];

const STAGING_SG_NODES: [&str; 3] = [
    "http://sglzdstgaero1-pub.sgdc:2379",
    "http://sglzdstgaero2-pub.sgdc:2379",
    "http://sglzdstgaero3-pub.sgdc:2379",
];

const STAGING_TH_NODES: [&str; 3] = [
    "http://thlzdstgaero3-pub.sgdc:2379",
    "http://thlzdstgaero4-pub.sgdc:2379",
    "http://thlzdstgaero5-pub.sgdc:2379",
];

const STAGING_VN_NODES: [&str; 3] = [
    "http://vnlzdstgaero1-pub.sgdc:2379",
    "http://vnlzdstgaero2-pub.sgdc:2379",
    "http://vnlzdstgaero3-pub.sgdc:2379",
];
