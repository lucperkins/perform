use async_trait::async_trait;
use perform::{Error, DB};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short = "p", long, default_value = "8080")]
    port: i16,

    #[structopt(
        long,
        default_value = "postgres://localhost/perform",
        env = "POSTGRES_URL"
    )]
    postgres_url: String,
}

struct Server {
    port: i16,
    db: DB,
}

impl Server {
    async fn new(opts: Opts) -> Result<Self, Error> {
        let db = DB::new(&opts.postgres_url).await?;

        println!("Connected to Postgres at {}", opts.postgres_url);

        Ok(Self {
            port: opts.port,
            db,
        })
    }

    async fn run(&self) -> Result<(), Error> {
        println!("Starting up on port {}", self.port);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts = Opts::from_args();
    let server = Server::new(opts).await?;

    let _ = server.run().await?;

    Ok(())
}
