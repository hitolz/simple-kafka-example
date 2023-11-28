use std::time::Duration;

use actix_web::{get, HttpResponse, Scope, web};
use simple_kafka::kafka_producer;

use crate::api::success;

pub fn routes() -> Scope {
    web::scope("/test")
    .service(test_send_kafka)
}

#[get("/send")]
pub async fn test_send_kafka() -> HttpResponse{
    let topic = "my_topic";
    // let _= kafka_producer::send(topic,"key","测试下kafka消息1111".as_bytes()).await;
    // let _= kafka_producer::send_result(topic,"key","测试下kafka消息1111".as_bytes()).await;
    let _= kafka_producer::send_timeout(topic,"key","测试下kafka消息1111".as_bytes(),Duration::from_secs(3)).await;
    success(Some("Successfully"))
}
