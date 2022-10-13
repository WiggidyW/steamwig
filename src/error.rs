#[derive(Debug)]
pub enum Error {
    MMTParseError(MMTParseError),
    ADCParseError(ADCParseError),
    PowershellError(powershell_script::PsError),
    CommandError(std::io::Error),
    TempDirError(std::io::Error),
    CsvError(csv::Error),
}

#[derive(Debug)]
pub struct MMTParseError {
    pub (crate) output: csv::StringRecord,
    pub (crate) index: usize,
    pub (crate) description: &'static str,
}

#[derive(Debug)]
pub struct ADCParseError {
    pub (crate) output: Vec<u8>,
    pub (crate) description: &'static str,
}