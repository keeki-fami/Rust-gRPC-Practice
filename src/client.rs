use bbb::OptionalRequest;
use bbb::optional_practice_client::OptionalPracticeClient;

pub mod bbb {
	tonic::include_proto!("optional_practice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut client = OptionalPracticeClient::connect("http://[::1]:50051").await?;

	let request = tonic::Request::new(OptionalRequest {
		hello: "hello".to_string()
	});

	let response = client.judge_hello(request).await?;

	println!("response: {response:?}");

	Ok(())
}