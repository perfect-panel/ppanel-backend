//! OpenTelemetry provider initialisation — ported from `pkg/trace/agent.go`.
//!
//! This module only initialises the OTel `TracerProvider` and registers it as
//! the global OTel provider.  It does NOT touch the `tracing` subscriber — the
//! bridge layer (`tracing-opentelemetry`) is added in `main.rs` when building
//! the subscriber stack.
//!
//! Supported batchers (mirrors Go `kindJaeger` / `kindOtlpGrpc` / etc.):
//! - `"jaeger"`   → OTLP gRPC (Jaeger v2 speaks native OTLP)
//! - `"otlpgrpc"` → OTLP gRPC
//! - `"otlphttp"` → OTLP HTTP
//! - `"stdout"`   → pretty-print to stdout
//! - disabled / empty endpoint → no-op

use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, TracerProvider};
use opentelemetry_sdk::Resource;

use crate::config::TraceConfig;

/// Held for the process lifetime; flushes pending spans on drop.
pub struct OtelGuard {
    provider: TracerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(e) = self.provider.shutdown() {
            eprintln!("[otel] shutdown error: {e}");
        }
        global::shutdown_tracer_provider();
    }
}

/// Build and install the OTel `TracerProvider`.
///
/// Returns `None` when tracing is disabled or no endpoint is configured.
/// The caller must keep the returned `OtelGuard` alive for the process lifetime.
pub fn init_otel(cfg: &TraceConfig) -> Option<OtelGuard> {
    if cfg.disabled {
        return None;
    }
    if cfg.endpoint.is_empty() && cfg.batcher.as_str() != "stdout" {
        return None;
    }

    let resource = Resource::builder()
        .with_service_name(cfg.name.clone())
        .build();

    let sampler = if (cfg.sampler - 1.0_f64).abs() < f64::EPSILON {
        Sampler::AlwaysOn
    } else {
        Sampler::TraceIdRatioBased(cfg.sampler)
    };

    let provider = match build_provider(cfg, resource, sampler) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("[otel] failed to initialise TracerProvider: {e}");
            return None;
        }
    };

    global::set_tracer_provider(provider.clone());
    tracing::info!(
        batcher = %cfg.batcher,
        endpoint = %cfg.endpoint,
        "OpenTelemetry tracing initialised",
    );
    Some(OtelGuard { provider })
}

// ─── private helpers ─────────────────────────────────────────────────────────

fn build_provider(
    cfg: &TraceConfig,
    resource: Resource,
    sampler: Sampler,
) -> anyhow::Result<TracerProvider> {
    use opentelemetry_sdk::trace::BatchExporter;

    let exporter: BatchExporter = match cfg.batcher.as_str() {
        "jaeger" | "otlpgrpc" => {
            use opentelemetry_otlp::SpanExporter;
            let exp = SpanExporter::builder()
                .with_tonic()
                .with_endpoint(&cfg.endpoint)
                .build()
                .map_err(|e| anyhow::anyhow!("otlpgrpc: {e}"))?;
            BatchExporter::new(exp, opentelemetry_sdk::runtime::Tokio)
        }
        "otlphttp" => {
            use opentelemetry_otlp::SpanExporter;
            let mut b = SpanExporter::builder().with_http().with_endpoint(&cfg.endpoint);
            if !cfg.otlp_headers.is_empty() {
                b = b.with_headers(cfg.otlp_headers.clone());
            }
            let exp = b.build().map_err(|e| anyhow::anyhow!("otlphttp: {e}"))?;
            BatchExporter::new(exp, opentelemetry_sdk::runtime::Tokio)
        }
        // stdout / default
        _ => {
            let exp = opentelemetry_stdout::SpanExporter::default();
            BatchExporter::new(exp, opentelemetry_sdk::runtime::Tokio)
        }
    };

    let provider = TracerProvider::builder()
        .with_resource(resource)
        .with_sampler(sampler)
        .with_id_generator(RandomIdGenerator::default())
        .with_span_processor(exporter)
        .build();

    Ok(provider)
}
