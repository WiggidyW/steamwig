use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::io;//

use zip;

const MMT: &str = "multimonitortool.zip";
const MMT_X64: &str = "multimonitortool-x64.zip";
const NIRCMD: &str = "nircmd.zip";
const NIRCMD_X64: &str = "nircmd-x64.zip";

fn unzip(zip_path: &Path, output_path: &Path) {
    let file: fs::File = fs::File::open(zip_path).unwrap();
    let mut archive: zip::ZipArchive<fs::File> = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file: zip::read::ZipFile = archive.by_index(i).unwrap();
        let mut out_file: fs::File = fs::File::create(
            output_path.join(file.enclosed_name().unwrap())
        ).unwrap();
        io::copy(&mut file, &mut out_file).unwrap();
    }
}

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        panic!("steamwig only supports Windows os build targets")
    }

    let root_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let asset_path: PathBuf = root_path.join("assets");
    let output_path: PathBuf = root_path.join("target")
        .join(env::var("PROFILE").unwrap())
        .join("deps");

    match &env::var("CARGO_CFG_TARGET_ARCH").unwrap()[..] {
        "x86" => {
            unzip(&asset_path.join(MMT), &output_path);
            unzip(&asset_path.join(NIRCMD), &output_path);
        },
        "x86_64" => {
            unzip(&asset_path.join(MMT_X64), &output_path);
            unzip(&asset_path.join(NIRCMD_X64), &output_path);
        },
        _ => panic!("steamwig only support x86 and x86_64 architecture build targets"),
    }
}