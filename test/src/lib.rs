
#[cfg(test)]
mod tests {
    use proto;

    #[test]
    fn test_proto_imports() {
        let request = proto::proto::HelloRequest {
            id: String::from("test_id"),
            person: String::from("test_person"),
        };

        let response = proto::proto::HelloResponse {
            id: String::from("response_id"),
            greeting: String::from("Hello"),
        };
        assert!(!request.id.is_empty(), "Request ID should not be empty");
        assert!(!response.greeting.is_empty(), "Response greeting should not be empty");
    }

    
    use proto::proto::{
        hello_service_server::{
            HelloServiceServer,
            HelloService,
        },
        HelloRequest,
        HelloResponse,
    };

    #[test]
    fn test_proto_tonic_server_import() {
        let _server: HelloServiceServer<MyHelloService>;
        assert!(true, "Server stub imported successfully");
    }

    struct MyHelloService;

    #[tonic::async_trait]
    impl HelloService for MyHelloService {
        async fn hello(
            &self,
            _request: tonic::Request<HelloRequest>,
        ) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
            unimplemented!()
        }
    }
}
