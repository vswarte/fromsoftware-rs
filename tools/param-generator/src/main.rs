use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use clap::{Parser, ValueEnum};
use quick_xml::de::from_str;

mod c;
mod param_def;
mod rust;
mod struct_def;

use c::*;
use param_def::*;
use rust::*;
use struct_def::*;

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    /// Rust struct definitions.
    Rust,

    /// C header definitions.
    C,
}

/// Command line arguments.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Folder containing XML files.
    #[arg(short, long)]
    input: String,

    /// Output file.
    #[arg(short, long)]
    output: String,

    /// The format to generate
    #[arg(short, long, default_value = "rust")]
    format: OutputFormat,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let definitions = load_definitions(&args.input)?;

    let output = match args.format {
        OutputFormat::Rust => generate_rust(definitions),
        OutputFormat::C => generate_c(definitions),
    };

    let output_path = Path::new(&args.output);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(output_path)?.write_all(output.as_bytes())?;
    Ok(())
}

/// Loads all the parameter definitions in the `input_path` directory and
/// returns them, sorted by name.
///
/// This also guarantees that either all structs have indexes defined, or none
/// do.
fn load_definitions(input_path: impl AsRef<Path>) -> io::Result<Vec<StructDef>> {
    let mut definitions = fs::read_dir(input_path.as_ref())?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("xml") {
                return None;
            }

            let content = fs::read_to_string(&path)
                .inspect_err(|e| eprintln!("Failed to read {}: {}", path.display(), e))
                .ok()?;
            let param_def = from_str::<ParamDef>(&content)
                .inspect_err(|e| eprintln!("Failed to parse {}: {}", path.display(), e))
                .ok()?;
            Some(StructDef::from(&param_def))
        })
        .collect::<Vec<_>>();

    // Parameters are always stored alphabetically in-game. We sort them by name
    // so that we can provide alphabetic indices to look them up by position.
    definitions.sort_by(|def1, def2| def1.name.cmp(&def2.name));

    if let Some(has) = definitions.iter().find(|def| def.index.is_some())
        && let Some(has_not) = definitions.iter().find(|def| def.index.is_none())
    {
        panic!(
            "{} has an index defined and {} does not. Either all params must have indices or none \
             may.",
            has.name, has_not.name
        );
    }

    Ok(definitions)
}
