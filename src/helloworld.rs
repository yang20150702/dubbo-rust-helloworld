/// The request message containing the user's name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message containing the greetings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod greeter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use dubbo::codegen::*;
    /// The greeting service definition.
    #[derive(Debug, Clone, Default)]
    pub struct GreeterClient {
        inner: TripleClient,
        uri: String,
    }
    impl GreeterClient {
        pub fn new() -> Self {
            Self {
                inner: TripleClient::new(),
                uri: "".to_string(),
            }
        }
        pub fn with_uri(mut self, uri: String) -> Self {
            self.uri = uri.clone();
            self.inner = self.inner.with_host(uri);
            self
        }
        /// Sends a greeting
        pub async fn say_hello(
            &mut self,
            request: Request<super::HelloRequest>,
        ) -> Result<Response<super::HelloReply>, dubbo::status::Status> {
            let codec = dubbo::codegen::ProstCodec::<
                super::HelloRequest,
                super::HelloReply,
            >::default();
            let path = http::uri::PathAndQuery::from_static(
                "/helloworld.Greeter/SayHello",
            );
            self.inner.unary(request, codec, path).await
        }
    }
}
/// Generated server implementations.
pub mod greeter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use dubbo::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GreeterServer.
    #[async_trait]
    pub trait Greeter: Send + Sync + 'static {
        /// Sends a greeting
        async fn say_hello(
            &self,
            request: Request<super::HelloRequest>,
        ) -> Result<Response<super::HelloReply>, dubbo::status::Status>;
    }
    /// The greeting service definition.
    #[derive(Debug)]
    pub struct GreeterServer<T: Greeter, I = TripleInvoker> {
        inner: _Inner<T>,
        invoker: Option<I>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Greeter, I> GreeterServer<T, I> {
        pub fn new(inner: T) -> Self {
            Self {
                inner: _Inner(Arc::new(inner)),
                invoker: None,
            }
        }
    }
    impl<T, I, B> Service<http::Request<B>> for GreeterServer<T, I>
    where
        T: Greeter,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
        I: Invoker + Send + 'static,
    {
        type Response = http::Response<BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/helloworld.Greeter/SayHello" => {
                    #[allow(non_camel_case_types)]
                    struct SayHelloServer<T: Greeter> {
                        inner: _Inner<T>,
                    }
                    impl<T: Greeter> UnarySvc<super::HelloRequest>
                    for SayHelloServer<T> {
                        type Response = super::HelloReply;
                        type Future = BoxFuture<
                            Response<Self::Response>,
                            dubbo::status::Status,
                        >;
                        fn call(
                            &mut self,
                            request: Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.inner.0.clone();
                            let fut = async move { inner.say_hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let fut = async move {
                        let mut server = TripleServer::new(
                            dubbo::codegen::ProstCodec::<
                                super::HelloReply,
                                super::HelloRequest,
                            >::default(),
                        );
                        let res = server.unary(SayHelloServer { inner }, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Greeter, I: Invoker + Send + 'static> Clone for GreeterServer<T, I> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner, invoker: None }
        }
    }
    impl<T: Greeter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    pub fn register_server<T: Greeter>(server: T) {
        let s = GreeterServer::<_, TripleInvoker>::new(server);
        dubbo::protocol::triple::TRIPLE_SERVICES
            .write()
            .unwrap()
            .insert(
                "helloworld.Greeter".to_string(),
                dubbo::utils::boxed_clone::BoxCloneService::new(s),
            );
    }
}
