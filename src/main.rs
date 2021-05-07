#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// esipam is a REST, Event-Sourced Web Application that helps you keep track of your IP address allocations.
/// - At Scale
/// - Across multiple heterogenous architectures
/// 

use std::collections::HashMap;
// use std::io::Read;

use cqrs_es::{AggregateError, Command};
use postgres::{Connection, TlsMode};
use postgres_es::{GenericQueryRepository, PostgresCqrs};
use serde::de::DeserializeOwned;
use actix_web::{get, post, web};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web::JsonConfig};
use actix_web::error::JsonPayloadError;
use log::debug;
use uuid::Uuid;

use crate::ipam_model::Ipam;
use crate::commands::{CreateNewIpam, AddCidrEntry};
use crate::error::IpamError;
use crate::events::IpamEvent;
use crate::queries::{IpamSummaryView, SimpleLoggingQueryProcessor};

mod common;
mod error;
mod commands;
mod ipam_model;
mod application;
mod events;
mod queries;

#[get("/api/health")]
async fn health() -> &'static str {
    "{ \"health\": \"ok\" }"
}

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("
    <h1>esipam</h1>
    <ul>
    <li>
      /api/ipam
    </li>
    <li>
      /api/health
    </li>
    <ul>
    ")
}

#[post("/api/ipam")]
async fn create_ipam(json: web::Json<CreateNewIpam>) -> impl Responder {
    let create: CreateNewIpam = json.clone();
    let ipam_id = &create.uuid; 
    match process_command::<CreateNewIpam>(&ipam_id, json.into_inner().clone()) {
        Ok(p)    => HttpResponse::Ok().json(&create),
        Err(err) => HttpResponse::InternalServerError().body(format!("fail {:?}", err))
    }
}

#[post("/api/ipam/{ipam_id}/cidrs")]
async fn add_cidr(web::Path(ipam_id): web::Path<Uuid>, json: web::Json<AddCidrEntry>) -> impl Responder {

    let add: AddCidrEntry = json.clone();

    match process_command::<AddCidrEntry>(&ipam_id, json.into_inner().clone()) {
        Ok(p)    => HttpResponse::Ok().json(&add),
        Err(err) => HttpResponse::InternalServerError().body(format!("fail {:?}", err))
    }


}


// router.post("/ipam",             ipam_command, "ipam_create");
// router.get("/ipam",              ipam_query,   "ipam_summary");
// router.delete("/ipam/:ipam_id",  ipam_command, "ipam_delete");
// router.put("/ipam/:ipam_id",     ipam_command, "ipam_amend");
// // router.get("/ipam/v/:query_id",             ipam_query, "ipam_view_query");

// // cidrs 
// router.post("/ipam/:ipam_id/cidrs",            ipam_command, "cidr_create");
// router.get("/ipam/:ipam_id/cidrs",             ipam_query, "ipam_cidrs_all");
// // router.get("/ipam/:ipam_id/cidrs/:cidr_id",    ipam_query, "ipam_cidr_by_id");
// router.delete("/ipam/:ipam_id/cidrs/:cidr_id", ipam_command, "cidr_delete");
// router.put("/ipam/:ipam_id/cidrs/:cidr_id",    ipam_command, "cidr_amend");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .data(
                web::JsonConfig::default()
                .error_handler(crate::error::json_error_handler)
                .limit(262_144),
             )
            .wrap(logger)
            .service(create_ipam)
            .service(add_cidr)
            .service(health)
            .service(index)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}


// pub fn ipam_command(req: &mut Request) -> IronResult<Response> {
//     let params = req.extensions.get::<Router>().unwrap();
//     let command_type = params.find("command_type").unwrap_or("");
//     let ipam_id = params.find("ipam_id").unwrap_or("");
//     let mut payload = String::new();
//     req.body.read_to_string(&mut payload).unwrap();

//     let result = match command_type {
//         // todo convert the string to a class (use a macro)
//         "createIpam"   => process_command::<CreateNewIpam>(ipam_id, payload),
//         "addCidrEntry" => process_command::<AddCidrEntry>(ipam_id, payload),
//         _ => return Ok(Response::with(status::NotFound))
//     };
    
//     match result {
//         Ok(p) => {
//             let ok_payload = serde_json::to_string(&p).unwrap();
//             Ok(Response::with(( status::Ok, ok_payload)))
//         },
//         Err(err) => {
//             let err_payload = match &err {
//                 AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
//                 AggregateError::TechnicalError(e) => e.clone(),
//             };

//             println!("!! error {:?}", err_payload);

//             let mut response = Response::with((status::BadRequest, err_payload));
//             response.headers = std_headers();
//             Ok(response)
//         }
//     }
// }

fn process_command<T>(ipam_id: &Uuid, payload: T) -> Result<(), AggregateError>
    where T: Command<Ipam, IpamEvent> + DeserializeOwned
{

    let cqrs = cqrs_framework();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    // TODO insert the authenticated endpoint
    // metadata.insert("identity".to_string(), ... );
    // metadata.insert("originator".to_string(), ... );

    cqrs.execute_with_metadata(&ipam_id.to_string(), payload, metadata)
}

type IpamSummaryViewProcessor = GenericQueryRepository::<IpamSummaryView, Ipam, IpamEvent>;


// pub fn ipam_query(req: &mut Request) -> IronResult<Response> {
//     let query_id = req.extensions.get::<Router>().unwrap().find("query_id").unwrap_or("").to_string();

//     println!(":: ipam_query called - {}", query_id);
    
//     let query_repo = IpamSummaryViewProcessor::new("ipam_query", db_connection());

//     match query_repo.load(query_id) {
//         None => {
//             Ok(Response::with(status::NotFound))
//         }
//         Some(query) => {
//             let body = serde_json::to_string(&query).unwrap();
//             let mut response = Response::with((status::Ok, body));
//             response.headers = std_headers();
//             Ok(response)
//         }
//     }
// }

fn cqrs_framework() -> PostgresCqrs<Ipam, IpamEvent> {

    // two query processors
    
    let simple_logger         = SimpleLoggingQueryProcessor {};
    let mut ipam_summary_view = IpamSummaryViewProcessor::new("ipam_query", db_connection());
    ipam_summary_view.with_error_handler(Box::new(|e| println!("<ipam_summary_view_failed> {}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_logger), Box::new(ipam_summary_view)])
}

fn db_connection() -> Connection {
    Connection::connect("postgresql://esipam_user:secret_saucey@localhost:5432/esipam", TlsMode::None).unwrap()
}
