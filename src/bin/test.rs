use futures::future::join_all;
use mock::{mock_client::MockClient as MockClientTonic, DoItRequest};
use std::time::Duration;
use tokio::{process, time::delay_for};

mod mock {
    tonic::include_proto!("mock");
}

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn run_server() -> Result<process::Child, BoxError> {
    let child = process::Command::new("target/release/server")
        .kill_on_drop(true)
        .spawn()?;
    Ok(child)
}

async fn send_request() -> Result<(), BoxError> {
    let mut client = MockClientTonic::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(DoItRequest {});
    let _reply = client.do_it(request).await?.into_inner();
    Ok(())
}

fn get_num_clients() -> usize {
    let args = std::env::args().collect::<Vec<_>>();
    let num_clients = args.get(1).expect("usage: test <num-clients>");
    num_clients.parse().unwrap()
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let num_clients = get_num_clients();

    let _server = run_server()?;
    // Give the server time to start
    delay_for(Duration::from_millis(100)).await;

    let r =
        join_all((0..num_clients).map(|_| tokio::spawn(send_request()))).await;

    for r in r {
        r??;
    }

    Ok(())
}
