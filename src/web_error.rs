use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web::error::JsonPayloadError;
use log::debug;

fn handle_bad_request<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<Body>> {
    
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );

    let errorMsg: String = match res.response().error() {
        Some(e) => format!("{:?}", e),
        None =>  String::from("Unknown Error")
    };

    let new_res: ServiceResponse<Body> = res.map_body(|_head, _body| {
        ResponseBody::Other(Body::Message(Box::new(errorMsg)))
    });

    Ok(ErrorHandlerResponse::Response(new_res))
        
}
