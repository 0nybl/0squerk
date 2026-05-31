use clap::{Parser, Subcommand};
use squerk::{config, decide, event};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "0squerk")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Decide whether a comment event should trigger a merge.
    Decide {
        #[arg(long)]
        event: PathBuf,
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Decide {
            event: event_path,
            config: config_path,
            out,
        } => {
            let ev = event::parse(&fs::read_to_string(&event_path).expect("read event.json"))
                .expect("parse event.json");
            // config file may be absent -> empty string -> default command
            let cfg = config::parse(&fs::read_to_string(&config_path).unwrap_or_default());
            let decision = decide::decide(&ev, &cfg);
            fs::write(&out, serde_json::to_string_pretty(&decision).unwrap())
                .expect("write decision.json");
        }
    }
}
