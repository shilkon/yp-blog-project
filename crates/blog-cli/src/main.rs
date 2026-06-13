use std::fs;

use anyhow::Context;
use blog_client::{
    BlogClientTransport, GrpcClient, HttpClient, Transport
};
use clap::{Parser, Subcommand};
use uuid::Uuid;

static SERVER_DEFAULT_ADDRESS: &'static str = "http://127.0.0.1";

#[derive(Parser)]
#[command(name = "blog", about = "CLI-утилита для взаимодействия с Blog-сервером", version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    server: Option<String>,
    #[arg(short, long)]
    grpc: bool,
    #[arg(short, long, default_value = "./.blog_token")]
    token_path: String
}

#[derive(Subcommand)]
enum Commands {
    /// Register user with `username`, `email` and `password`
    Register {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    /// Login with `username` and `password`
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    /// Get post by `id`
    Get {
        #[arg(short, long)]
        id: Uuid,
    },
    /// Get posts with `limit` and `offset`
    List {
        #[arg(short, long, default_value_t = 10)]
        limit: i64,
        #[arg(short, long, default_value_t = 0)]
        offset: i64,
    },
    /// Create post with `title` and `offset`
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    /// Create post with `id`, `title` and `offset`
    Update {
        #[arg(short, long)]
        id: Uuid,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    /// Delete post with `id`
    Delete {
        #[arg(short, long)]
        id: Uuid,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let address = args.server.unwrap_or_else(|| {
        if args.grpc {
            format!("{SERVER_DEFAULT_ADDRESS}:50051")
        } else {
            format!("{SERVER_DEFAULT_ADDRESS}:8080/api")
        }
    });

    let mut client = if args.grpc {
        Transport::Grpc(GrpcClient::connect(address).await?)
    } else {
        Transport::Http(HttpClient::new(address)?)
    };

    match args.command {
        Commands::Register { username, email, password } => {
            let resp = client.register(username, email, password).await?;
            fs::write(&args.token_path, resp.token)?;
            println!("{:?}", resp.user);
        },
        Commands::Login { username, password } => {
            let resp = client.login(username, password).await?;
            fs::write(&args.token_path, resp.token)?;
            println!("{:?}", resp.user);
        },
        Commands::Get { id } => {
            let post = client.get_post(id).await?;
            println!("{:?}", post);
        },
        Commands::List { limit, offset } => {
            let resp = client.list_posts(limit, offset).await?;
            println!("{:?}", resp);
        },
        Commands::Create { title, content } => {
            let token = fs::read_to_string(&args.token_path)
                .context(format!("Missed token: {}", args.token_path))?;
            client.set_token(token);
            let post = client.create_post(title, content).await?;
            println!("{:?}", post);
        },
        Commands::Update { id, title, content } => {
            let token = fs::read_to_string(&args.token_path)
                .context(format!("Missed token: {}", args.token_path))?;
            client.set_token(token);
            let post = client.update_post(id, title, content).await?;
            println!("{:?}", post);
        },
        Commands::Delete { id } => {
            let token = fs::read_to_string(&args.token_path)
                .context(format!("Missed token: {}", args.token_path))?;
            client.set_token(token);
            client.delete_post(id).await?;
        },
    }

    Ok(())
}
