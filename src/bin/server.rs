use mock::{
    mock_server::{Mock, MockServer},
    DoItReply, DoItRequest,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod mock {
    tonic::include_proto!("mock");
}

pub struct MockService {}

#[tonic::async_trait]
impl Mock for MockService {
    async fn do_it(
        &self,
        _request: Request<DoItRequest>,
    ) -> Result<Response<DoItReply>, Status> {
        let size = 2 * 1024 * 1024; // 2 MiB
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(0);
        }

        let reply = DoItReply { data };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mock = MockService {};

    Server::builder()
        .add_service(MockServer::new(mock))
        .serve(addr)
        .await?;

    Ok(())
}
