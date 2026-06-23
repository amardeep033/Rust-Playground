use tonic::{transport::Server, Request, Response, Status};
use prost_types::FieldMask;
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{UpdateUserRequest, User};

//1 include proto-generated code
pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<User>, Status> {

        let req = request.into_inner();

        let user = req.user.ok_or(Status::invalid_argument("Missing user"))?;
        let mask: FieldMask = req.update_mask.ok_or(
            Status::invalid_argument("Missing update_mask")
        )?;

        // Simulated DB record
        let mut db_user = User {
            id: 1,
            name: "OldName".into(),
            email: "old@email.com".into(),
        };

        for path in mask.paths {
            match path.as_str() {
                "name" => db_user.name = user.name.clone(),
                "email" => db_user.email = user.email.clone(),
                _ => {}
            }
        }

        Ok(Response::new(db_user))
    }
}

// Simple auth interceptor
fn auth_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token = req
        .metadata()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if token != "Bearer secret123" {
        return Err(Status::unauthenticated("Invalid token"));
    }

    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "127.0.0.1:50051".parse()?;

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(GreeterServer::with_interceptor(
            MyGreeter::default(),
            auth_interceptor,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
