//! OpenTelemetry provider initialisation — ported from `pkg/trace/agent.go`.
//!
//! Currently only the stdout exporter is wired up.  OTLP gRPC / HTTP support
//! can be added once the opentelemetry-otlp 0.27 builder API is confirmed.
//!
//! To enable: set `Trace.Batcher = "stdout"` in config (any non-empty batcher
//! with a non-empty endpoint also works; set `Trace.Disabled = true` to opt out).

use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_sdk::trace::TracerProvider;
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
/// Returns `None` when tracing is disabled (config `Trace.Disabled = true`)
/// or no batcher is configured.
pub fn init_otel(cfg: &TraceConfig) -> Option<OtelGuard> {
    if cfg.disabled {
        return None;
    }
    // Require explicit opt-in: batcher must be set.
    if cfg.batcher.is_empty() || cfg.batcher == "none" {
        return None;
    }

    let resource = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        cfg.name.clone(),
    )]);

    // ── Exporter selection ────────────────────────────────────────────────
    // Only stdout is wired for now; extend with OTLP branches as needed.
    let exporter = opentelemetry_stdout::SpanExporter::default();

    let provider = TracerProvider::builder()
        .with_resource(resource)
        .with_simple_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    tracing::info!(
        batcher = %cfg.batcher,
        name    = %cfg.name,
        "OpenTelemetry tracing initialised",
    );
    Some(OtelGuard { provider })
}
