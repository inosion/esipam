#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// esipam is a REST, Event-Sourced Web Application that helps you keep track of your IP address allocations.
/// - At Scale
/// - Across multiple heterogenous architectures
/// 
/// 

use std::collections::HashMap;
use std::io::Read;

use cqrs_es::{AggregateError, Command};
use iron::{Headers, Iron, IronResult, Request, Response, status};
use postgres::{Connection, TlsMode};
use postgres_es::{GenericQueryRepository, PostgresCqrs};
use router::Router;
use serde::de::DeserializeOwned;

use crate::ipam_model::Ipam;
use crate::commands::{CreateNewIpam};
use crate::events::IpamEvent;
use crate::queries::{IpamQuery, SimpleLoggingQueryProcessor};

mod common;
mod commands;
mod ipam_model;
mod application;
mod events;
mod queries;
mod test;

fn main() {
    let mut router = Router::new();
    router.get("/ipam/:query_id", ipam_query, "ipam_query");
    router.post("/ipam/:command_type/:aggregate_id", ipam_command, "ipam_command");
    Iron::new(router).http("localhost:9090").unwrap();
}

pub fn ipam_command(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let command_type = params.find("command_type").unwrap_or("");
    let aggregate_id = params.find("aggregate_id").unwrap_or("");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();

    let result = match command_type {
        // todo convert the string to a class (use a macro)
        "createIpam" => process_command::<CreateNewIpam>(aggregate_id, payload),
        _ => return Ok(Response::with(status::NotFound))
    };
    match result {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.clone(),
            };
            let mut response = Response::with((status::BadRequest, err_payload));
            response.headers = std_headers();
            Ok(response)
        }
    }
}

fn process_command<T>(aggregate_id: &str, payload: String) -> Result<(), AggregateError>
    where T: Command<Ipam, IpamEvent> + DeserializeOwned
{
    let payload = match serde_json::from_str::<T>(payload.as_str()) {
        Ok(payload) => { payload }
        Err(err) => {
            return Err(AggregateError::TechnicalError(err.to_string()));
        }
    };
    let cqrs = cqrs_framework();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    cqrs.execute_with_metadata(aggregate_id, payload, metadata)
}

pub fn ipam_query(req: &mut Request) -> IronResult<Response> {
    let query_id = req.extensions.get::<Router>().unwrap().find("query_id").unwrap_or("").to_string();

    println!("foo - {}", query_id);
    let query_repo = EsIpamQuery::new("ipam_query", db_connection());
    match query_repo.load(query_id) {
        None => {
            Ok(Response::with(status::NotFound))
        }
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            let mut response = Response::with((status::Ok, body));
            response.headers = std_headers();
            Ok(response)
        }
    }
}

fn std_headers() -> Headers {
    let mut headers = Headers::new();
    let content_type = iron::headers::ContentType::json();
    headers.set(content_type);
    headers
}

type EsIpamQuery = GenericQueryRepository::<IpamQuery, Ipam, IpamEvent>;

fn cqrs_framework() -> PostgresCqrs<Ipam, IpamEvent> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut ipam_query_processor = EsIpamQuery::new("ipam_query", db_connection());
    ipam_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(ipam_query_processor)])
}

fn db_connection() -> Connection {
    Connection::connect("postgresql://esipam_user:secret_saucey@localhost:5432/esipam", TlsMode::None).unwrap()
}
