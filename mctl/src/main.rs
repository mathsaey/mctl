use clap::{Parser, Subcommand};
use std::cmp::min;

mod matrix;
mod process;

#[derive(Parser, Debug)]
#[command(version, about, trailing_var_arg = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = 20, verbatim_doc_comment)]
    /// Set default pixel brightness
    /// Values can range from 0 to 255
    brightness: u8,

    #[arg(short, long, default_value_t = 2, verbatim_doc_comment)]
    /// Timeout (seconds) after which
    /// the matrix is cleared
    timeout: u64,

    #[arg(short, long)]
    /// Don't print log messages, overrides verbose
    quiet: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    /// Increase verbosity, repeat to become more verbose
    verbose: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "p")]
    Percent { percent: u8 },

    Speaker {
        #[command(subcommand)]
        status: SpeakerStatus,
    },
}

#[derive(Subcommand, Debug)]
enum SpeakerStatus {
    On,
    Off,
}

fn setup_logger(args: &Cli) {
    const DEFAULT_VERBOSITY: u8 = 2;
    let filter = match if args.quiet { 0 } else { DEFAULT_VERBOSITY + args.verbose } {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(filter)
        .format_target(false)
        .format_level(false)
        .init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    setup_logger(&args);

    log::info!("mctl v{}", env!("CARGO_PKG_VERSION"));
    log::trace!("Command-line arguments: {:#?}", args);

    process::kill_other_mctl_processes();

    let mut matrix = matrix::open(args.brightness)?;
    log::info!("Connected to {:}", matrix);

    match args.command {
        Commands::Percent { percent: p } => {
            matrix.percent(min(p, 100))?;
        }
        Commands::Speaker { status } => {
            match status {
                SpeakerStatus::On => matrix::draw_speaker_on(&mut matrix),
                SpeakerStatus::Off => matrix::draw_speaker_mute(&mut matrix),
            }?;
        }
    }

    matrix::wait_and_reset(&mut matrix, args.timeout)?;
    Ok(())
}
