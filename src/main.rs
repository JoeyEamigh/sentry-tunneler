#![feature(let_chains)]

use axum::{extract::State, response::IntoResponse, routing::post};
use config::TunnelConfig;
use hyper::{header, StatusCode};
use std::sync::Arc;
use tracing::info;

mod config;
mod sentry;

#[tokio::main]
async fn main() {
  init_logger();
  let config = TunnelConfig::new();
  let state = Arc::new(config.clone());

  let mut cors = tower_http::cors::CorsLayer::new()
    .allow_headers([hyper::header::CONTENT_TYPE])
    .allow_methods([hyper::Method::POST]);

  if let Some(allowed_origins) = config.cors_allowed_origins {
    let mut origins = Vec::new();

    for origin in allowed_origins {
      if let Ok(origin) = origin.parse::<hyper::header::HeaderValue>() {
        origins.push(origin);
      }
    }

    cors = cors.allow_origin(origins);
  } else {
    cors = cors.allow_origin(hyper::header::HeaderValue::from_static("*"));
  }

  let app = axum::Router::new()
    .route(&config.tunnel_path, post(tunnel))
    .layer(cors)
    .with_state(state);

  info!("listening on port {}", config.listen_port);

  let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.listen_port));
  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn tunnel(State(config): State<Arc<TunnelConfig>>, body: String) -> Result<StatusCode, impl IntoResponse> {
  let sentry = sentry::SentryMessage::try_from(body)
    .map_err(|_| StatusCode::BAD_REQUEST)
    .unwrap();

  info!(
    "received request bound for {} project {}",
    sentry.dsn.host(),
    sentry.dsn.project_id().value()
  );

  if !config.is_allowed_host(sentry.dsn.host()) || !config.is_allowed_project(sentry.dsn.project_id().value()) {
    return Err((
      StatusCode::FORBIDDEN,
      [(header::CONTENT_TYPE, "application/json")],
      "".to_string(),
    ));
  }

  let res = sentry.forward().await;

  if let Err(res) = res {
    Err((
      res.status(),
      [(header::CONTENT_TYPE, "application/json")],
      res.text().await.expect("failed to get response body"),
    ))
  } else {
    Ok(StatusCode::NO_CONTENT)
  }
}

fn init_logger() {
  use tracing::metadata::LevelFilter;
  use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
  };

  #[cfg(debug_assertions)]
  let filter_directives = if let Ok(filter) = std::env::var("RUST_LOG") {
    filter
  } else {
    "sentry_tunneler=trace".to_string()
  };

  #[cfg(debug_assertions)]
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::TRACE.into())
    .parse_lossy(filter_directives);

  #[cfg(not(debug_assertions))]
  let filter_directives = if let Ok(filter) = std::env::var("RUST_LOG") {
    filter
  } else {
    "sentry_tunneler=info".to_string()
  };

  #[cfg(not(debug_assertions))]
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .parse_lossy(filter_directives);

  tracing_subscriber::registry()
    .with(fmt::layer().with_filter(filter))
    .init();
}
