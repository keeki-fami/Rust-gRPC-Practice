mod  aaa {
    tonic::include_proto!("optional_practice");
}

 use aaa::{
    optional_practice_server::{OptionalPracticeServer, OptionalPractice},
    OptionalRequest, OptionalResponse, ServerComment
 };

 use tonic::{transport::Server, Request, Response, Status};

// use crate::aaa::TestResponse1;

pub struct MyOptionPractice {}

#[tonic::async_trait]
impl OptionalPractice for MyOptionPractice {
    async fn judge_hello(
        &self,
        request: Request<OptionalRequest>,
    ) -> Result<Response<OptionalResponse>, Status> {
        let server_comment =  ServerComment {
            comment: "声が小さい".to_string()
        };
        println!("client");
        let response = OptionalResponse {
            point: Some(10 as i32),
            comment: Some(server_comment)
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let origin = "127.0.0.1:50051".parse()?;
    let service = MyOptionPractice {};

    println!("server is listening on {}", origin);

    Server::builder()
        .add_service(OptionalPracticeServer::new(service))
        .serve(origin)
        .await?;

    Ok(())
}

