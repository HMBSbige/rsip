#![warn(clippy::unwrap_used)]

use actix_web::HttpRequest;
use actix_web::http::StatusCode;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::env;
use std::net::{IpAddr, Ipv6Addr};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn get_client_ip(req: HttpRequest) -> (String, StatusCode) {
    let get_xff = |header_name: &str| -> Option<IpAddr> {
        let ip = req.headers().get(header_name)?.to_str().ok()?;
        let ip = ip.split(',').next()?;
        let ip = ip.trim();

        ip.parse::<IpAddr>().ok()
    };

    if let Some(ip) = get_xff("X-Forwarded-For") {
        return (ip.to_string(), StatusCode::OK);
    }

    // unreachable!()
    if let Some(addr) = req.peer_addr() {
        return (
            addr.ip().to_canonical().to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }

    (String::default(), StatusCode::INTERNAL_SERVER_ERROR)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("RSIP_PORT_NUMBER").unwrap_or_default();
    let port = port.parse::<u16>().unwrap_or(80);

    HttpServer::new(|| App::new().service(health).service(get_client_ip))
        .bind((Ipv6Addr::UNSPECIFIED, port))?
        .run()
        .await
}

#[cfg(test)]
mod tests;
