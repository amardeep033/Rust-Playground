use actix_web::{
    App, HttpResponse, HttpServer, get, middleware, web,
};
use scraper::{Html, Selector};

#[get("/{name}")]
async fn index(name: web::Path<String>) -> HttpResponse {
    let url = "https://books.toscrape.com/";
    let html = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = Html::parse_document(&html);

    let book_sel = Selector::parse("article.product_pod").unwrap();
    let title_sel = Selector::parse("h3 a").unwrap();

    let ttle_fnd: Vec<String> = document
        .select(&book_sel)
        .filter_map(|book| book.select(&title_sel).next())
        .map(|a| a.value().attr("title").unwrap_or("").trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();


    HttpResponse::Ok()
        .body(format!("Hello {} -- {:?}",name,ttle_fnd))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}