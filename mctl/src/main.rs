use clap::{Parser, Subcommand};

mod matrix;
mod process;

#[derive(Parser,Debug)]
#[command(version, about, disable_version_flag = true, trailing_var_arg = true)]
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
    /// Don't print log messages
    quiet: bool,

    #[arg(long, verbatim_doc_comment)]
    /// Don't write a lockfile.
    no_lock: bool,

    #[arg(short = 'v', long, action = clap::builder::ArgAction::Version)]
    /// Print version
    version: (),
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "p")]
    Percent{
        percent: u8
    },

    Speaker{
        #[command(subcommand)]
        status: SpeakerStatus
    }
}

#[derive(Subcommand, Debug)]
enum SpeakerStatus { On, Off }

fn setup_logger(args: &Cli) {
    if !args.quiet {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Warn)
            .format_target(false)
            .init();
    }
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
        Commands::Percent{percent: p} => {
            matrix.percent(p)?;
        },
        Commands::Speaker{status} => {
            match status {
                SpeakerStatus::On => matrix::draw_speaker_on(&mut matrix),
                SpeakerStatus::Off => matrix::draw_speaker_mute(&mut matrix)
            }?;
        }
    }

    matrix::wait_and_reset(&mut matrix, args.timeout)?;
    Ok(())
}
