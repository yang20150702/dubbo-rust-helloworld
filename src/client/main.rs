use dubbo::codegen::Request;
use dubbo_rust_helloworld::helloworld::greeter_client::GreeterClient;
use dubbo_rust_helloworld::helloworld::HelloRequest;

#[tokio::main]
async fn main() {
    let mut cli = GreeterClient::new().with_uri("http://127.0.0.1:8888".to_string());
    let resp = cli
        .say_hello(Request::new(HelloRequest {
            name: "hello, I'm client".to_string(),
        }))
        .await
        .unwrap();

    let (_, msg) = resp.into_parts();
    println!("response: {:?}", msg);
}
