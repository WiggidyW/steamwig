#[derive(Debug)]
pub enum Error {
    MMTParseError(crate::display_sys::MMTParseError),
    ADCParseError(crate::audio_sys::ADCParseError),
    PowershellError(powershell_script::PsError),
    CommandError(std::io::Error),
    TempDirError(std::io::Error),
    CsvError(csv::Error),
}