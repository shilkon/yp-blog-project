mod transport;

#[cfg(feature = "grpc")]
mod blog_proto {
    tonic::include_proto!("blog"); 
}

pub use transport::{
    AuthResponse,
    User,
    Post,
    PostsResponse,
    Transport,
    BlogClientTransport,
    TransportError,
    HttpClient
};

#[cfg(feature = "grpc")]
pub use transport::GrpcClient;
