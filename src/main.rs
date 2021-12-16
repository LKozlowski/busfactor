mod busfactor;
mod contributor;

use anyhow::{Context, Result};
use octocrab::Octocrab;
use std::sync::Arc;
use structopt::StructOpt;

use busfactor::bus_factor_command;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake-case")]
struct Opt {
    #[structopt(long)]
    language: String,

    #[structopt(long)]
    project_count: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let github_access_token = std::env::var("BUSFACTOR_GITHUB_ACCESS_TOKEN")
        .with_context(|| "BUSFACTOR_GITHUB_ACCESS_TOKEN env variable is required")?;

    env_logger::init();
    let args = Opt::from_args();
    let octocrab = Arc::new(
        Octocrab::builder()
            .personal_token(github_access_token)
            .build()?,
    );

    bus_factor_command(octocrab.clone(), &args.language, args.project_count).await?;
    Ok(())
}
