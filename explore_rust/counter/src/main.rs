use actix_web::{App, HttpServer, Responder, web};
use prometheus::{TextEncoder, Encoder, gather};
use std::time::Instant;

mod metrics;
use metrics::Metrics;

async fn initiate_payment(metrics: web::Data<Metrics>) -> impl Responder {
    metrics.payment_initiated.inc();

    let start = Instant::now();

    // --- Simulate payment processing ---
    let ok = rand::random::<bool>();   // randomly success/fail
    std::thread::sleep(std::time::Duration::from_millis(300));
    // -----------------------------------

    if ok {
        metrics.payment_success.inc();
    } else {
        metrics.payment_failed.inc();
    }

    let elapsed = start.elapsed().as_secs_f64();
    metrics.payment_latency.observe(elapsed);

    format!("payment result = {}", ok)
}

async fn metrics_endpoint() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = gather();

    let mut buf = Vec::new();
    encoder.encode(&metric_families, &mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metrics = Metrics::new();
    let metrics_data = web::Data::new(metrics);

    println!("Server running → http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(metrics_data.clone())
            .route("/initiate-payment", web::get().to(initiate_payment))
            .route("/metrics", web::get().to(metrics_endpoint))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}