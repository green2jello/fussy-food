use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fussyfood")]
#[command(about = "FussyFood CLI tool for food-related operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Performs a sample operation
    Sample {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sample { name } => {
            println!("Hello, {}! Welcome to FussyFood!", name);
        }
    }
}
