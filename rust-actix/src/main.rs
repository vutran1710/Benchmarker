use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SumResponse {
    total: u32,
}

#[derive(Deserialize)]
pub struct Sum {
    x: u32,
    y: u32,
}

async fn sum_calculator(web::Query(param): web::Query<Sum>) -> Result<HttpResponse, Error> {
    let total = param.x + param.y;
    let resp = SumResponse { total };
    Ok(HttpResponse::Ok().json(resp))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/rust/plus").route(web::get().to(sum_calculator)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
