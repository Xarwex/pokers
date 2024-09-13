use clap::{Args, Parser, Subcommand};
use rascii_art::RenderOptions;
use tempdir::TempDir;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search the pokemon by name
    Name(Name),
    /// Search the pokemon by id
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

    let mut print_name = false;

    let Ok(pokemon) = (match &cli.command {
        Commands::Name(name) => rustemon::pokemon::pokemon::get_by_name(&name.name, &client).await,
        Commands::Id(id) => {
            print_name = true;
            rustemon::pokemon::pokemon::get_by_id(id.id, &client).await
        }
    }) else {
        println!("Pokemon not found :(");
        return;
    };

    if let Some(url) = pokemon.sprites.front_default {
        //get the sprite
        let sprite = reqwest::get(url).await.unwrap().bytes().await.unwrap();
        let tmp_dir = TempDir::new("pokers").unwrap();
        let file_path = tmp_dir.path().join("sprite.png");
        let mut file = File::create_new(file_path.clone()).await.unwrap();
        file.write_all(sprite.as_ref()).await.unwrap();
        let mut buf = String::new();
        rascii_art::render_to(
            file_path.to_str().unwrap(),
            &mut buf,
            &RenderOptions::new().width(100).colored(true),
        )
        .unwrap();
        println!("{}", buf);
    } else {
        println!("No sprite found for {}", pokemon.name)
    }

    if print_name {
        println!("{}!", pokemon.name.to_uppercase())
    }
}
