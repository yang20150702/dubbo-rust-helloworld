use async_trait::async_trait;
use dubbo::codegen::{Request, Response};
use dubbo::Dubbo;
use dubbo_rust_helloworld::helloworld::greeter_server::{register_server, Greeter};
use dubbo_rust_helloworld::helloworld::{HelloReply, HelloRequest};

#[tokio::main]
async fn main() {
    register_server(GreeterImpl::new());

    Dubbo::new().start().await;
}

#[derive(Debug, Clone, Default)]
pub struct GreeterImpl {}

impl GreeterImpl {
    pub fn new() -> Self {
        GreeterImpl {}
    }
}

#[async_trait]
impl Greeter for GreeterImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, dubbo::status::Status> {
        println!("request: {:?}", request.into_inner());

        Ok(Response::new(HelloReply {
            message: "hello dubbo-rust!".to_string(),
        }))
    }
}
