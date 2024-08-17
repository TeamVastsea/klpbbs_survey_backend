use axum::extract::DefaultBodyLimit;
use axum::http::HeaderValue;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use lazy_static::lazy_static;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, NotSet};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::log::{warn, LevelFilter};
use tracing::{debug, info};
use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};
use uuid::Uuid;
use migration::{Migrator, MigratorTrait};

use crate::config::core::CoreConfig;
use crate::config::get_config;
use crate::config::oauth::OAuthConfig;
use crate::model::generated::prelude::Question;
use crate::model::generated::question;
use crate::model::question::{Condition, ConditionInner, ConditionType, QuestionType};
use crate::model::ValueWithTitle;
use crate::service::questions::save_question;

mod config;
mod controller;
mod model;
mod service;

lazy_static! {
    static ref CORE_CONFIG: CoreConfig = get_config("core");
    static ref OAUTH_CONFIG: OAuthConfig = get_config("oauth");
    static ref DATABASE: DatabaseConnection = {
        let mut opt = ConnectOptions::new(&CORE_CONFIG.db_uri);
        opt.sqlx_logging(true);
        opt.sqlx_logging_level(LevelFilter::Info);
        futures::executor::block_on(Database::connect(opt)).unwrap_or_else(|e| {
            panic!("Failed to connect to database '{}': {}", CORE_CONFIG.db_uri, e)
        })
    };
}

#[tokio::main]
async fn main() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&CORE_CONFIG.trace_level));
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .filename_prefix("logs/")
        .build("")
        .unwrap();
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    let formatting_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.f(%:z)".to_string()));
    let file_layer = fmt::layer()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.f(%:z)".to_string()))
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(file_layer)
        .init();

    Migrator::up(&*DATABASE, None).await.unwrap();

    // let question = crate::model::question::Question {
    //     id: Uuid::new_v4(),
    //     content: ValueWithTitle {
    //         title: "title".to_string(),
    //         content: "content".to_string()
    //     },
    //     r#type: QuestionType::SingleChoice,
    //     values: Some(vec![ValueWithTitle {
    //         title: "title".to_string(),
    //         content: "content".to_string()
    //     }]),
    //     condition: Some(vec![Condition {
    //         r#type: ConditionType::And,
    //         conditions: vec![ConditionInner {
    //             id: Uuid::new_v4(),
    //             value: "value".to_string()
    //         }]
    //     }]),
    //     required: false
    // };
    // 
    // println!("{}", serde_json::to_string(&question).unwrap());

    // save_question(question, None).await;

    let origins = CORE_CONFIG.origins.clone().iter().map(|x| x.parse().unwrap()).collect::<Vec<HeaderValue>>();
    let app = Router::new()
        .nest("/api", controller::all_routers())
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier()
        ))
        .layer(DefaultBodyLimit::max(
            CORE_CONFIG.max_body_size * 1024 * 1024,
        ))
        .layer(CorsLayer::very_permissive().allow_origin(origins).allow_credentials(CORE_CONFIG.allow_credentials))
        .layer(CatchPanicLayer::new());

    let addr = CORE_CONFIG.server_addr.parse().unwrap();
    info!("Listening: {addr}");

    if CORE_CONFIG.tls {
        debug!("HTTPS enabled.");
        let tls_config =
            RustlsConfig::from_pem_file(&CORE_CONFIG.ssl_cert, &CORE_CONFIG.ssl_key)
                .await
                .unwrap();
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        warn!("HTTPS disabled.");
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
