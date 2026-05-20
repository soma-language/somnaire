pub mod stats;

use axum::Router;
use axum::http::Method;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, HeaderName, HeaderValue};
use axum::routing::get;
use governor::DefaultKeyedRateLimiter;
use governor::middleware::StateInformationMiddleware;
use sea_orm::DatabaseConnection;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_governor::governor::{GovernorConfig, GovernorConfigBuilder};
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

pub fn router(db: DatabaseConnection) -> Router {
    let cors = build_cors_layer();

    let login_governor = governor_cfg(4, 3);
    let signup_governor = governor_cfg(10, 2);
    let recovery_governor = governor_cfg(5, 2);
    let ws_governor = governor_cfg(20, 10);

    spawn_limiter_cleanup(
        vec![
            login_governor.limiter().clone(),
            signup_governor.limiter().clone(),
            recovery_governor.limiter().clone(),
            ws_governor.limiter().clone(),
        ],
        Duration::from_mins(1),
    );

    Router::new()
        .nest("/api", Router::new().route("/stats", get(stats::handler)))
        .layer(cors)
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .layer(security_headers())
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            ),
        )
        .with_state(db)
}

fn spawn_limiter_cleanup(
    limiters: Vec<Arc<DefaultKeyedRateLimiter<IpAddr, StateInformationMiddleware>>>,
    interval: Duration,
) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            for limiter in &limiters {
                let before: usize = limiter.len();
                limiter.retain_recent();
                let after: usize = limiter.len();
                if before != after {
                    tracing::debug!(
                        "Rate limiter cleanup: evicted {} stale entries ({} remaining)",
                        before - after,
                        after,
                    );
                }
            }
        }
    });
}

fn build_cors_layer() -> CorsLayer {
    let cors_origin = std::env::var("CORS_ORIGIN")
        .expect("CORS_ORIGIN environment variable is required for CORS configuration");
    let is_permissive = cors_origin.is_empty() || cors_origin == "*";

    let allow_origin = if is_permissive {
        tracing::warn!("Using permissive CORS");
        AllowOrigin::any()
    } else {
        let origins: Vec<HeaderValue> = cors_origin
            .split(',')
            .filter_map(|o| {
                let trimmed = o.trim();
                trimmed.parse::<HeaderValue>().ok()
            })
            .collect();
        AllowOrigin::list(origins)
    };

    let mut layer = CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .max_age(std::time::Duration::from_hours(1));

    if !is_permissive {
        layer = layer.allow_credentials(true);
    }

    layer
}

fn security_headers() -> SecurityHeadersService {
    ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static(
                "camera=(), microphone=(), geolocation=(), interest-cohort=()",
            ),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; frame-ancestors 'none'"),
        ))
}

type SecurityHeadersService = ServiceBuilder<
    tower::layer::util::Stack<
        SetResponseHeaderLayer<HeaderValue>,
        tower::layer::util::Stack<
            SetResponseHeaderLayer<HeaderValue>,
            tower::layer::util::Stack<
                SetResponseHeaderLayer<HeaderValue>,
                tower::layer::util::Stack<
                    SetResponseHeaderLayer<HeaderValue>,
                    tower::layer::util::Stack<
                        SetResponseHeaderLayer<HeaderValue>,
                        tower::layer::util::Stack<
                            SetResponseHeaderLayer<HeaderValue>,
                            tower::layer::util::Stack<
                                SetResponseHeaderLayer<HeaderValue>,
                                tower::layer::util::Identity,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

fn governor_cfg(
    per_second: u64,
    burst: u32,
) -> Arc<GovernorConfig<SmartIpKeyExtractor, StateInformationMiddleware>> {
    Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(per_second)
            .burst_size(burst)
            .use_headers()
            .finish()
            .expect("Failed to build rate limiter"),
    )
}
