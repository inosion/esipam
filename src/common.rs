#![macro_use]

use thiserror::Error;

macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

/// IpamGroupError for the Ipam Service
#[derive(Error, Debug)]
pub enum IpamError {

    #[error(transparent)]
    InvalidEntry( #[from] ipnetwork::IpNetworkError ),

    #[error("CidrEntry is the wrong Protocol. either V6 or V4 ")]    
    InvalidProtocol,

}
