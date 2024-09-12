use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Name(Name),
    Id(Id),
}

#[derive(Args)]
struct Name {
    name: String,
}

#[derive(Args)]
struct Id {
    id: i64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = rustemon::client::RustemonClient::default();

    let pokemon = match &cli.command {
        Commands::Name(name) => rustemon::pokemon::pokemon::get_by_name(&name.name, &client).await,
        Commands::Id(id) => rustemon::pokemon::pokemon::get_by_id(id.id, &client).await,
    }
    .unwrap();

    println!("{}", pokemon.name)
}
