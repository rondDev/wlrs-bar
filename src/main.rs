pub mod app;
pub mod draw;
pub mod utils;
pub mod widget;

pub mod clock;
pub mod updated_last;
pub mod workspaces;

use clap::Parser;

/// A Hyprland Status Bar for me :)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The timestamp of the last update
    #[arg(short, long)]
    updated_last: Option<i64>,
    #[arg(long, default_value_t = 28)]
    height: u32,

    #[arg(long, default_value_t = 0)]
    width: u32,
}

pub fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("BAR_WLRS_LOG")
        .init();

    let args = Args::parse();

    let (mut app, mut event_queue) = app::App::new(args);

    app.run_queue(&mut event_queue);
}
