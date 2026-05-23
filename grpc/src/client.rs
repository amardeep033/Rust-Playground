use tonic::{Request, transport::Channel};
use prost_types::FieldMask;

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_client::GreeterClient;
use hello::{UpdateUserRequest, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = UpdateUserRequest {
        user: Some(User {
            id: 1,
            name: "Amardeep".into(),
            email: "".into(),
        }),
        update_mask: Some(FieldMask {
            paths: vec!["name".into()],
        }),
    };

    let mut grpc_request = Request::new(request);

    // Add auth token
    grpc_request
        .metadata_mut()
        .insert("authorization", "Bearer secret123".parse()?);

    let response = client.update_user(grpc_request).await?;

    println!("Updated User: {:?}", response.into_inner());

    Ok(())
}
