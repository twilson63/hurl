use anyhow::Result;
use clap::{Parser, Subcommand};
use hurl_lib::VERSION;
use std::path::PathBuf;

mod cli;

use cli::commands;

#[derive(Parser)]
#[command(name = "hurl")]
#[command(version = VERSION)]
#[command(about = "A modern HTTP client written in Rust")]
#[command(author = "HURL Contributors")]
#[command(long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, short, long, help = "Enable verbose output")]
    verbose: bool,

    #[arg(global = true, short, long, help = "Suppress all output except errors")]
    quiet: bool,

    #[arg(global = true, long, help = "Path to configuration file")]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Send a GET request")]
    Get {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Send a POST request")]
    Post {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'd', long, help = "Request body data")]
        data: Option<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Send a PUT request")]
    Put {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'd', long, help = "Request body data")]
        data: Option<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Send a DELETE request")]
    Delete {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Send a PATCH request")]
    Patch {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'd', long, help = "Request body data")]
        data: Option<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },

    #[command(about = "Send a HEAD request")]
    Head {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,
    },

    #[command(about = "Send an OPTIONS request")]
    Options {
        #[arg(value_name = "URL", help = "Target URL")]
        url: String,

        #[arg(short = 'H', long, help = "Add request header")]
        header: Vec<String>,

        #[arg(short = 'u', long, help = "Set Basic authentication (user:password)")]
        auth: Option<String>,

        #[arg(long, help = "Set request timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'o', long, help = "Output response to file")]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = cli::config::Config::new(cli.verbose, cli.quiet, cli.config);

    if config.verbose {
        eprintln!("[VERBOSE] Starting HURL v{}", VERSION);
    }

    let result = match cli.command {
        Commands::Get {
            url,
            header,
            auth,
            timeout,
            output,
        } => commands::handle_get(&url, header, auth, timeout, output, &config).await,
        Commands::Post {
            url,
            header,
            data,
            auth,
            timeout,
            output,
        } => commands::handle_post(&url, header, data, auth, timeout, output, &config).await,
        Commands::Put {
            url,
            header,
            data,
            auth,
            timeout,
            output,
        } => commands::handle_put(&url, header, data, auth, timeout, output, &config).await,
        Commands::Delete {
            url,
            header,
            auth,
            timeout,
            output,
        } => commands::handle_delete(&url, header, auth, timeout, output, &config).await,
        Commands::Patch {
            url,
            header,
            data,
            auth,
            timeout,
            output,
        } => commands::handle_patch(&url, header, data, auth, timeout, output, &config).await,
        Commands::Head {
            url,
            header,
            auth,
            timeout,
        } => commands::handle_head(&url, header, auth, timeout, &config).await,
        Commands::Options {
            url,
            header,
            auth,
            timeout,
            output,
        } => commands::handle_options(&url, header, auth, timeout, output, &config).await,
    };

    match result {
        Ok(_) => {
            if config.verbose {
                eprintln!("[VERBOSE] Request completed successfully");
            }
            Ok(())
        }
        Err(e) => {
            if !config.quiet {
                eprintln!("Error: {}", e);
            }
            Err(e)
        }
    }
}
