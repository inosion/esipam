macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}
pub(crate) use s;

/// IpamError for the IPAM Service
#[derive(Error, Debug)]
pub enum IpamError {
    #[error(transparent)
    InvalidEntry( #[from] ipnetwork::IpNetworkError )
}
