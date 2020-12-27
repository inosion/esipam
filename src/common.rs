#![macro_use]

use thiserror::Error;
use std::convert::From;
use cqrs_es::AggregateError;

macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

/// IpamError for the Ipam Service 
/// Using `thiserror` 
#[derive(Error, Debug)]
pub enum IpamError {

    #[error(transparent)]
    InvalidEntry( #[from ] ipnetwork::IpNetworkError ),

    #[error("CidrEntry is the wrong Protocol. either V6 or V4 ")]    
    InvalidProtocol,

}

impl std::convert::From<IpamError> for cqrs_es::AggregateError {
    fn from(err: IpamError) -> AggregateError {
        AggregateError::TechnicalError(err.to_string())
    }
}
