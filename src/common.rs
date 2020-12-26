#![macro_use]

use thiserror::Error;

macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

/// IpamError for the IPAM Service
#[derive(Error, Debug)]
pub enum IpamError {

    #[error(transparent)]
    InvalidEntry( #[from] ipnetwork::IpNetworkError ),

    #[error("Entry is either V6 or V4 and should be v.v.")]    
    InvalidProtocol,

}
