use clap::{Parser, Subcommand};

mod signer;
mod verifier;

static DEFAULT_MESSAGE: &str = "workshop_la_casona";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Sign a message
    Sign {
        message: Option<String>,
    },
    // Verify signature
    // takes in a signature(encoded as a hex string) and the
    // signer's public-key-hash(same encoding as signature)
    Verify {
        signature: String,
        pkh: String,
        message: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sign { message } => {
            println!("message: {:?}", message);
            let res = signer::sign_message(message);
            // usar hex::encode para convertir la firma a un string y mostrarlo en pantalla.
            unimplemented!()
        }
        Commands::Verify {
            signature,
            pkh,
            message,
        } => {
            println!(
                "message: {:?} - signature: {} - pkh: {}",
                message, signature, pkh
            );
            let res = verifier::verify(message, pkh, signature);
            unimplemented!()
        }
    }
}
