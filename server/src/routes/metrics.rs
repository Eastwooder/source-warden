use std::future::ready;

use axum::{extract::MatchedPath, middleware::Next, routing::get, Router};
use axum_core::{extract::Request, response::IntoResponse};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::time::Instant;

pub fn router() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

pub fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full(REQUEST_DURATION.to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    const UNKNOWN_PATH: &str = "/<unknown>";

    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str())
        .unwrap_or(UNKNOWN_PATH)
        .to_owned();
    let method = req.method().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!(SUM_REQUESTS, &labels).increment(1);
    metrics::histogram!(REQUEST_DURATION, &labels).record(latency);

    response
}

const REQUEST_DURATION: &str = "http_requests_duration_seconds";
const SUM_REQUESTS: &str = "http_requests_total";
