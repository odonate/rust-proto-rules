pub mod proto;

#[cfg(test)]
mod tests {
    use crate::proto::test_proto;

    #[test]
    fn test_proto_imports() {
        let request = test_proto::HelloRequest {
            id: String::from("test_id"),
            person: String::from("test_person"),
        };

        let response = test_proto::HelloResponse {
            id: String::from("response_id"),
            greeting: String::from("Hello"),
        };
        assert!(!request.id.is_empty(), "Request ID should not be empty");
        assert!(!response.greeting.is_empty(), "Response greeting should not be empty");
    }

    
    use crate::proto::test_proto_tonic::{
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
