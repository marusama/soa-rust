use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum Env {
    Staging,
    Live
}

#[derive(Debug, PartialEq)]
pub enum Venture {
    ID,
    MY,
    PH,
    TH,
    SG,
    VN,
}

impl Display for Env {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Venture {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
