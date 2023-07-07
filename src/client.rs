use users::user_service_client::UserServiceClient;
use users::GetUserRequest;

pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetUserRequest { id: 42 });

    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
