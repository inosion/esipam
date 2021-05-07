use thiserror::Error;
// use std::convert::From;
use cqrs_es::AggregateError;

use actix_web::{error::ResponseError, HttpResponse};
use actix_web::{HttpRequest};
use actix_web::error::JsonPayloadError;


#[macro_export]
macro_rules! type_of {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                let (type_,tmp) = $crate::error::type_of2(tmp);
                eprintln!("[{}:{}] {}: {}",
                    file!(), line!(), stringify!($val), type_);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::type_of!($val)),+,)
    };
}


pub fn type_of2<T>(v: T) -> (&'static str, T) {
    (std::any::type_name::<T>(), v)
}

/// IpamError for the Ipam Service 
/// Using `thiserror` 
#[derive(Error, Debug)]
pub enum IpamError {

    #[error(transparent)]
    InvalidEntry( #[from ] ipnetwork::IpNetworkError ),

    #[error("CidrEntry is the wrong Protocol. either V6 or V4 ")]    
    InvalidProtocol,

    #[error("The request was badness::\n{0}")]
    BadRequest(String),

    #[error("BadRequestPayload format - {0}")]
    BadRequestPayload(String),

    #[error("The payload was too large")]
    PayloadTooLarge,

    #[error("badness on the inside")]
    InternalServerError

}


impl std::convert::From<IpamError> for cqrs_es::AggregateError {
    fn from(err: IpamError) -> AggregateError {
        AggregateError::TechnicalError(err.to_string())
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for IpamError {
    fn error_response(&self) -> HttpResponse {
        match self {
            IpamError::InternalServerError => {
                HttpResponse::InternalServerError().json(format!("{}",self))
            },
            IpamError::BadRequest(_) => HttpResponse::BadRequest().json(format!("{}",self)),
            IpamError::BadRequestPayload(_) => HttpResponse::BadRequest().json(format!("{}",self)),
            IpamError::InvalidProtocol => HttpResponse::BadRequest().json(format!("{}",self)),
            // IpamError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            // IpamError::NotFound => HttpResponse::NotFound().json("Not Found"),
            // IpamError::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            // IpamError::Conflict(ref message) => HttpResponse::Conflict().json(message),
            IpamError::PayloadTooLarge => {
                HttpResponse::PayloadTooLarge().json("Payload Too Large")
            },
            _ => HttpResponse::BadRequest().json("error occured, IpamError not mapped due to laziness"),
        }
    }
}

pub fn json_error_handler(error: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    type_of!(&error);
    match error {
        JsonPayloadError::Overflow => IpamError::PayloadTooLarge.into(),
        JsonPayloadError::Deserialize(error) => IpamError::BadRequestPayload(error.to_string()).into(),
        _ => IpamError::BadRequest(error.to_string()).into(),
    }
}
