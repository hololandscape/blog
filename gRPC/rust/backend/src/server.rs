//! Server for the demo backend.

use tonic::{transport::Server, Request, Response, Status};

pub mod demo {
    include!("../generated/demo.rs");
}

// Import the trait of Demo and struct of struct DemoService.
use crate::demo::demo_server::{Demo, DemoServer};
use crate::demo::{HelloRequest, HelloResponse};

#[derive(Default, Debug)]
pub struct MyDemo {}

#[tonic::async_trait]
impl Demo for MyDemo {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HellloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = demo::HelloResponse {
            // We must use .into_inner() as the fields of gRPC requests and responses are private
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let demo = MyDemo::default();

    let svc = DemoServer::new(demo);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
