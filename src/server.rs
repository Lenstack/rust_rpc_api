use tonic::{transport::Server, Request, Response, Status};

use users::user_service_server::{UserService, UserServiceServer};

use users::{GetUserRequest, GetUserResponse, User};

pub mod users {
    tonic::include_proto!("users");
}

#[derive(Default)]
pub struct MyUsers {}

#[tonic::async_trait]
impl UserService for MyUsers {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a request: {:?}", request);

        let user = User {
            id: 1,
            name: "John".into(),
        }
        .into();

        let response = GetUserResponse { user: Some(user) };

        Ok(Response::new(response))
    }
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let users = MyUsers::default();

    Server::builder()
        .add_service(UserServiceServer::new(users))
        .serve(addr)
        .await?;

    Ok(())
}

// Path: src\client.rs
