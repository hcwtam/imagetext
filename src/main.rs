use imagetext::{cli, transform};

#[tokio::main]
async fn main() {
    
    let args = cli::run();
    transform::run(args).await;
}
