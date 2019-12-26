use futures::future::join_all;
use mock::{mock_client::MockClient as MockClientTonic, DoItRequest};
use std::time::Duration;
use structopt::StructOpt;
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

async fn send_request_in_process() -> Result<(), BoxError> {
    let mut client = MockClientTonic::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(DoItRequest {});
    let _reply = client.do_it(request).await?.into_inner();
    Ok(())
}

async fn send_request_in_subprocess() -> Result<(), BoxError> {
    let status = process::Command::new("target/release/client")
        .kill_on_drop(true)
        .status()
        .await?;
    if status.success() {
        Ok(())
    } else {
        Err("client failed".into())
    }
}

async fn send_request(multiprocess: bool) -> Result<(), BoxError> {
    if multiprocess {
        send_request_in_subprocess().await
    } else {
        send_request_in_process().await
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "multiprocess")]
    multiprocess: bool,
    num_clients: usize,
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    env_logger::init();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    let _server = run_server()?;
    // Give the server time to start
    delay_for(Duration::from_millis(100)).await;

    let r = join_all(
        (0..opt.num_clients)
            .map(|_| tokio::spawn(send_request(opt.multiprocess))),
    )
    .await;

    for r in r {
        r??;
    }

    Ok(())
}
