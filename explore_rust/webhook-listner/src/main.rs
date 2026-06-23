use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer};

#[post("/webhook")]
async fn webhook(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    println!("--- Webhook received ---");

    println!("Headers:");
    for (k, v) in req.headers() {
        println!("{}: {:?}", k, v);
    }

    println!("\nBody:");
    match std::str::from_utf8(&body) {
        Ok(s) => println!("{}", s),
        Err(_) => println!("{:?}", body),
    }

    println!("------------------------");

    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Webhook receiver listening on http://127.0.0.1:9000/webhook");

    HttpServer::new(|| {
        App::new()
            .service(webhook)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
