use actix_web::{get, HttpResponse, Scope, web};
use simple_kafka::kafka_producer;

use crate::api::success;

pub fn routes() -> Scope {
    web::scope("/test")
    .service(test_send_kafka)
}

#[get("/send")]
pub async fn test_send_kafka() -> HttpResponse{
    let topic = "test-topic";
    let _= kafka_producer::send(topic,"key","测试下kafka消息1111".as_bytes()).await;
    success(Some("Successfully"))
}
