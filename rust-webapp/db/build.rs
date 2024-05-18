use cornucopia::{CodegenSettings, Error};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let migrations_path = "migrations";

    // Fetch the output directory from Cargo environment variables
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let destination = Path::new(&out_dir).join("queries.rs");

    let settings = CodegenSettings {
        is_async: true,
        derive_ser: true,
    };

    // Ensure the directory for the destination file exists
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).expect("Failed to create destination directory");
    }

    // Find all files and convert paths to `String`
    let schema_files: Vec<String> = find_schema_files(&PathBuf::from(migrations_path));
    println!("cargo:rerun-if-changed={}", queries_path);
    println!("cargo:rerun-if-changed={}", migrations_path);
    schema_files
        .iter()
        .for_each(|file| println!("cargo:rerun-if-changed={}", file));

    if let Ok(live_db_url) = env::var("PG_TMP_URL") {
        let cfg = postgres::config::Config::from_str(&live_db_url)
            .expect(&format!("Error parsing Live DB URL '{}'", live_db_url));
        let mut client = cfg
            .connect(postgres::tls::NoTls)
            .expect("Error connecting to the DB");

        cornucopia::load_schema(&mut client, schema_files).expect("Error applying DB schema");
        cornucopia::generate_live(&mut client, &queries_path, destination.to_str(), settings)?;
    } else {
        cornucopia::generate_managed(
            queries_path,
            schema_files,
            destination.to_str(),
            true,
            settings,
        )?;
    }

    Ok(())
}

fn find_schema_files(dir: &PathBuf) -> Vec<String> {
    let mut schema_files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            if entry.path().is_file() {
                schema_files.push(entry.path().to_string_lossy().to_string());
            }
        }
    }
    schema_files
}
