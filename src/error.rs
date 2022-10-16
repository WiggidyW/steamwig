#[derive(Debug)]
pub enum Error {
    PowershellError(powershell_script::PsError),
    CommandError(std::io::Error),
    TempDirError(std::io::Error),
    MMTParseError(MMTParseError),
    ADCParseError(ADCParseError),
    CsvError(csv::Error),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MMTParseError {
    pub (crate) output: csv::StringRecord,
    pub (crate) index: usize,
    pub (crate) description: &'static str,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ADCParseError {
    pub (crate) output: String,
    pub (crate) description: &'static str,
}