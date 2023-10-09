use actix_web::{HttpServer, App};
use chrono::Local;
use fern::Dispatch;
use simple_kafka::{self, message_handler};
use tracing::info;


pub mod api;
pub mod setting;


#[actix_web::main]
pub async fn main() {
    init_log();
    let x = &setting::SETTING;
    let app = &x.app;
    let kafka_config = &x.kafka_config;
    let _init_task = tokio::spawn(async {
        let simple_kafka_config:simple_kafka::KafkaConfig = kafka_config.to_owned().into();
        simple_kafka::kafka_init::init_producers(&simple_kafka_config).await;
        simple_kafka::kafka_init::init_consumers(&simple_kafka_config,"test-topic", message_handler).await;
    });
    
    info!("server listening at http://{}:{}", app.host, app.port);

    HttpServer::new(move|| {
        App::new()
        .service(api::test_api::routes())
    })
    .bind((app.host.as_str(), app.port)).unwrap()
    .run()
    .await.unwrap();
}

fn init_log() {
    std::env::set_var("RUST_LOG", "sqlx::query=error");

    // let level = match setting::get_log_level().as_str() {
    //     "trace" => log::LevelFilter::Trace,
    //     "debug" => log::LevelFilter::Debug,
    //     "info" => log::LevelFilter::Info,
    //     "warn" => log::LevelFilter::Warn,
    //     _ => log::LevelFilter::Error,
    // };
    let level = log::LevelFilter::Info;
    let log_path = "app.log";
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path).unwrap())
        .level(level)
        .level_for("sqlx::query", log::LevelFilter::Error)
        .apply().expect("log init error");
}