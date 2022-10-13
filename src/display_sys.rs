use crate::display::{DisplayState, DisplayModifier};
use crate::error::MMTParseError;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::process;

use tempfile::TempDir;
use csv;

#[derive(Debug)]
pub struct MMTModifier {
    pub (crate) exe_path: PathBuf,
}

#[derive(Debug)]
struct ParsedRecord {
    enabled: bool,
    primary: bool,
    id: String,
}

impl DisplayModifier for MMTModifier {
    fn get_system_state(&self) -> Result<DisplayState, crate::Error> {
        get_system_state(&self.exe_path)
    }

    fn enable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error> {
        enable_monitors(&self.exe_path, device_ids)
    }

    fn disable_monitors(&self, device_ids: &[&str]) -> Result<(), crate::Error> {
        disable_monitors(&self.exe_path, device_ids)
    }

    fn set_primary_device(&self, id: &str) -> Result<(), crate::Error> {
        set_primary_device(&self.exe_path, id)
    }
}

impl MMTModifier {
    pub fn new(exe_path: std::path::PathBuf) -> Self {
        MMTModifier { exe_path: exe_path }
    }
}

fn scomma(exe_path: &Path, csv_outfile_path: &Path) -> Result<(), crate::Error> {
    mmt_run(exe_path, "/scomma", &[&csv_outfile_path.to_string_lossy()]).map(|_| ())
}

fn parse_csv_record(record: csv::StringRecord) -> Result<ParsedRecord, crate::Error> {
    let enabled: bool = match record.get(3) {
        Some("Yes") => true,
        Some("No") => false,
        _ => return Err(crate::Error::MMTParseError(MMTParseError {
            output: record,
            index: 3,
            description: "should contain either Yes or No",
        })),
    };
    let primary: bool = match record.get(5) {
        Some("Yes") => true,
        Some("No") => false,
        _ => return Err(crate::Error::MMTParseError(MMTParseError {
            output: record,
            index: 5,
            description: "should contain either Yes or No",
        })),
    };
    let id: String = match record.get(17) {
        Some(s) => s.to_string(),
        None => return Err(crate::Error::MMTParseError(MMTParseError {
            output: record,
            index: 17,
            description: "should contain device ID",
        })),
    };
    Ok(ParsedRecord {
        enabled: enabled,
        primary: primary,
        id: id,
    })
}

fn parse_csv_reader(reader: csv::Reader<File>) -> Result<DisplayState, crate::Error> {
    let mut display_state: DisplayState = DisplayState {
        primary_device_id: String::new(),
        enabled_device_ids: Vec::new(),
        disabled_device_ids: Vec::new(),
    };
    for record in reader.into_records() {
        let record: csv::StringRecord = record.map_err(|e| crate::Error::CsvError(e))?;
        let parsed_record: ParsedRecord = parse_csv_record(record)?;
        match (parsed_record.primary, parsed_record.enabled) {
            (false, false) => display_state.disabled_device_ids.push(parsed_record.id),
            (false, true) => display_state.enabled_device_ids.push(parsed_record.id),
            (true, false) => {
                display_state.primary_device_id = parsed_record.id.clone();
                display_state.disabled_device_ids.push(parsed_record.id)
            },
            (true, true) => {                
                display_state.primary_device_id = parsed_record.id.clone();
                display_state.enabled_device_ids.push(parsed_record.id)
            },
        }
    }
    Ok(display_state)
}

fn mmt_run(exe_path: &Path, arg: &str, extra_args: &[&str]) -> Result<process::Output, crate::Error> {
    process::Command::new(exe_path).arg(arg)
        .args(extra_args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .map_err(|e| crate::Error::CommandError(e))
}

fn get_system_state(exe_path: &Path) -> Result<DisplayState, crate::Error> {
    let temp_dir: TempDir = TempDir::new().map_err(|e| crate::Error::TempDirError(e))?;
    let csv_outfile_path: PathBuf = temp_dir.path().join("d.csv");
    scomma(exe_path, &csv_outfile_path)?;
    let reader: csv::Reader<File> = csv::Reader::from_path(csv_outfile_path).map_err(|e| crate::Error::CsvError(e))?;
    parse_csv_reader(reader)
}

fn enable_monitors(exe_path: &Path, device_ids: &[&str]) -> Result<(), crate::Error> {
    mmt_run(exe_path, "/enable", device_ids).map(|_| ())
}

fn disable_monitors(exe_path: &Path, device_ids: &[&str]) -> Result<(), crate::Error> {
    mmt_run(exe_path, "/disable", device_ids).map(|_| ())
}

fn set_primary_device(exe_path: &Path, id: &str) -> Result<(), crate::Error> {
    mmt_run(exe_path, "/SetPrimary", &[id]).map(|_| ())
}