use anyhow::Result;
use clap::{Parser, Subcommand};

const OWNER: &str = env!("HARNESS_REPO_OWNER");
const REPO: &str = env!("HARNESS_REPO_NAME");
const BIN: &str = "harness";

#[derive(Parser)]
#[command(name = "harness", version, about = "Local-first voice agent harness for code projects")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check for a newer release, optionally install it
    Update {
        /// Only check, do not install
        #[arg(long)]
        check: bool,
    },
}

fn target() -> &'static str {
    self_update::get_target()
}

fn updater() -> Result<Box<dyn self_update::update::ReleaseUpdate>> {
    Ok(self_update::backends::github::Update::configure()
        .repo_owner(OWNER)
        .repo_name(REPO)
        .bin_name(BIN)
        .target(target())
        .current_version(env!("CARGO_PKG_VERSION"))
        .show_download_progress(true)
        .no_confirm(true)
        .build()?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Cmd::Update { check }) => {
            let u = updater()?;
            let latest = u.get_latest_release()?;
            let cur = env!("CARGO_PKG_VERSION");
            if !self_update::version::bump_is_greater(cur, &latest.version)? {
                println!("up to date ({cur})");
                return Ok(());
            }
            println!("update available: {cur} -> {}", latest.version);
            if !check {
                let st = u.update()?;
                println!("updated to {}", st.version());
            }
        }
        None => println!("harness {} (voice UI and dashboard not built yet)", env!("CARGO_PKG_VERSION")),
    }
    Ok(())
}
