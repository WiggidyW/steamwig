mod error;
use error::Error;

mod internal;
use internal::system_modifier::SystemModifier;
use internal::task::Task;

use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Steam {
    Off,
    Normal,
    BigPicture,
}

impl std::fmt::Display for Steam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Steam::Off => "off".fmt(f),
            Steam::Normal => "normal".fmt(f),
            Steam::BigPicture => "big picture".fmt(f),
        }
    }
}

impl std::str::FromStr for Steam {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Steam::Off),
            "normal" => Ok(Steam::Normal),
            "big picture" | "big_picture" => Ok(Steam::BigPicture),
            _ => Err(format!("Unknown steam state: {s}")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// [path]
    #[arg(long)]
    steam_path: PathBuf,

    /// [path]
    #[arg(long)]
    mmt_path: Option<PathBuf>,

    /// [path]
    #[arg(long)]
    adc_path: Option<PathBuf>,

    /// [integer]
    #[arg(long)]
    max_attempts: Option<usize>,

    /// [integer]
    #[arg(long)]
    sleep_interval: Option<u64>,

    /// [ID code]
    #[arg(long)]
    primary_display: Option<String>,

    /// [ID code]
    #[arg(long)]
    enable_display: Option<Vec<String>>,

    /// [ID code]
    #[arg(long)]
    disable_display: Option<Vec<String>>,

    /// [ID code]
    #[arg(long)]
    primary_audio: Option<String>,

    /// [integer, possible values: 0 to 100]
    #[arg(long)]
    volume: Option<u8>,

    #[arg(long)]
    muted: Option<bool>,

    /// [possible values: off, normal, big_picture]
    #[arg(long)]
    steam: Option<Steam>,

    /// Print all Display and Audio IDs
    #[arg(short, long)]
    readout: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let mut system_modifier: SystemModifier = SystemModifier::new(args.steam_path.clone());

    if let Some(p) = args.mmt_path { system_modifier.with_mmt_path(p) }
    if let Some(p) = args.adc_path { system_modifier.with_adc_path(p) }

    if args.readout {
        let display_readout: String = match system_modifier.display_id_readout() {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let audio_readout: String = match system_modifier.audio_id_readout() {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("Display Readout");
        println!("");
        println!("{}", display_readout);
        println!("");
        println!("Audio Readout");
        println!("");
        println!("{}", audio_readout);
        return ()
    }

    if let Some(u) = args.max_attempts { system_modifier.with_max_attempts(u) }
    if let Some(u) = args.sleep_interval { system_modifier.with_sleep_interval(Duration::from_secs(u)) }

    let mut task: Task = Task::new();

    if let Some(id) = args.primary_display { task.set_primary_display(id) }
    if let Some(ids) = args.enable_display { task.enable_displays(ids) }
    if let Some(ids) = args.disable_display { task.disable_displays(ids) }
    if let Some(id) = args.primary_audio { task.set_primary_audio(id) }
    if let Some(v) = args.volume { task.set_volume(v) }
    if let Some(m) = args.muted { task.set_muted(m) }
    if let Some(s) = args.steam {
        match s {
            Steam::Off => task.set_steam_not_running(),
            Steam::Normal => task.set_steam_running_normal(),
            Steam::BigPicture => task.set_steam_running_big_picture(),
        }
    }

    match system_modifier.run(&task) {
        Ok(true) => println!("Success"),
        Ok(false) => println!("Failure"),
        Err(e) => panic!("{:?}", e),
    }
}