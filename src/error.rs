pub enum Error {
    Unimplemented,
    TempDirError(std::io::Error),
    CommandError(std::io::Error),
    CsvError(csv::Error),
    PowershellError(powershell_script::PsError),
    MalformedCsvError,
    MalformedPowershellError,
}