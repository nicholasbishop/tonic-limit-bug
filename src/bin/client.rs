use mock::{mock_client::MockClient as MockClientTonic, DoItRequest};

mod mock {
    tonic::include_proto!("mock");
}

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let mut client = MockClientTonic::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(DoItRequest {});
    let _reply = client.do_it(request).await?.into_inner();
    Ok(())
}
