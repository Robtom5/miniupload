use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod conf;
mod serv;

pub(crate) const APP_NAME: &str = "MiniUpload";

#[derive(Parser)]
struct Cli {
    // /// The base url to upload the file to
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Upload a file to the file server")]
    Upload(UploadArgs),
    #[command(about = "Download a file from the file server")]
    Download(DownloadArgs),
    #[command(about = "Set configuration options for this tool")]
    Config(ConfigArgs),
}

#[derive(Args)]
struct UploadArgs {
    #[arg(help = "The file to upload to the file server")]
    file: std::path::PathBuf,
}

#[derive(Args)]
struct DownloadArgs {
    #[arg(help = "The name of the file to download")]
    file_name: String,

    #[arg(help = "The location to save the file to")]
    dest: String,
}

#[derive(Args)]
struct ConfigArgs {
    #[arg(short, long, help = "Set the folder to use for upload")]
    folder: Option<String>,

    #[arg(short, long, help = "Set the address to upload to")]
    address: Option<String>,

    // Print the current config values
    #[arg(short, long, help = "Prints the currently configured values")]
    print: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Commands::Upload(up_args) => {
            serv::upload_file(up_args.file.to_path_buf())?;
        }
        Commands::Download(down_args) => {
            serv::download(&down_args.file_name, &down_args.dest)?;
        }
        Commands::Config(conf_args) => {
            let mut cfg = conf::ToolConfig::from_app(APP_NAME)?;

            match &conf_args.folder {
                Some(folder) => cfg.update_folder(folder.to_string()),
                None => {}
            }

            match &conf_args.address {
                Some(target) => cfg.update_target(target.to_string()),
                None => {}
            }

            if conf_args.print {
                println!("Current Target: {}", &cfg.get_target()?);
                println!("Current Folder: {}", &cfg.get_folder()?);
            }

            cfg.save_app(APP_NAME)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
