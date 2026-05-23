use actix_web::{guard, web, App, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use std::sync::{Arc, Mutex};

//---------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Book {
    id: i32,
    title: String,
    author: String,
}

// Make Book a GraphQL object
#[Object]
impl Book {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn author(&self) -> &str {
        &self.author
    }
}

//---------------------------------------------------------------------------------------------------------------------------------------------------

struct QueryRoot {
    storage: BookStorage,
}

#[Object]
impl QueryRoot {
    // Simple query to get a book by ID
    async fn book(&self, id: i32) -> Option<Book> {
        let books = self.storage.lock().unwrap();
        books.iter().find(|b| b.id == id).cloned()
    }

    // Query to get all books
    async fn books(&self) -> Vec<Book> {
        let books = self.storage.lock().unwrap();
        books.clone()
    }
}

//----------------------------------------------------------------------------

// GraphQL Mutation root
struct MutationRoot {
    storage: BookStorage,
}

#[Object]
impl MutationRoot {
    // Add a new book
    async fn add_book(&self, title: String, author: String) -> Book {
        let mut books = self.storage.lock().unwrap();
        let new_id = books.iter().map(|b| b.id).max().unwrap_or(0) + 1;
        
        let new_book = Book {
            id: new_id,
            title,
            author,
        };
        
        books.push(new_book.clone());
        new_book
    }

    // Update an existing book
    async fn update_book(&self, id: i32, title: Option<String>, author: Option<String>) -> Option<Book> {
        let mut books = self.storage.lock().unwrap();
        
        if let Some(book) = books.iter_mut().find(|b| b.id == id) {
            if let Some(t) = title {
                book.title = t;
            }
            if let Some(a) = author {
                book.author = a;
            }
            Some(book.clone())
        } else {
            None
        }
    }

    // Delete a book
    async fn delete_book(&self, id: i32) -> bool {
        let mut books = self.storage.lock().unwrap();
        let original_len = books.len();
        books.retain(|b| b.id != id);
        books.len() < original_len
    }
}

//---------------------------------------------------------------------------------------------------------------------------------------------------

// GraphQL schema type
type AppSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;

// GraphQL endpoint handler
async fn graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// GraphQL Playground handler
async fn graphql_playground() -> Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

//---------------------------------------------------------------------------------------------------------------------------------------------------

type BookStorage = Arc<Mutex<Vec<Book>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize in-memory storage with some initial books
    let storage: BookStorage = Arc::new(Mutex::new(vec![
        Book {
            id: 1,
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
        },
        Book {
            id: 2,
            title: "Programming Rust".to_string(),
            author: "Jim Blandy".to_string(),
        },
    ]));

    // Create the schema with queries and mutations
    let schema = Schema::build(
        QueryRoot {
            storage: storage.clone(),
        },
        MutationRoot {
            storage: storage.clone(),
        },
        async_graphql::EmptySubscription,
    )
    .finish();

    println!("🚀 GraphQL Playground: http://localhost:8080/playground");
    println!("📡 GraphQL endpoint: http://localhost:8080/graphql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/playground").guard(guard::Get()).to(graphql_playground))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}