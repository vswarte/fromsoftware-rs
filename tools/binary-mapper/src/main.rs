use std::{collections::HashMap, fs::File, path::PathBuf};

use clap::{Parser, ValueEnum};
use memmap::MmapOptions;
use pelite::{
    pattern,
    pe64::{Pe, PeFile},
};
use rayon::prelude::*;
use serde::Deserialize;
use shared::{find_rtti_classes, Class};

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Print,
    Rust,
}

/// Run a mapper profile against a binary to produce
#[derive(Parser)]
struct Args {
    #[arg(long, env("MAPPER_PROFILE"))]
    profile: PathBuf,

    #[arg(long, env("MAPPER_GAME_EXE"))]
    exe: PathBuf,

    #[arg(long, env("MAPPER_OUTPUT_FORMAT"))]
    output: OutputFormat,
}

fn main() {
    let args = Args::parse();

    let exe_file = File::open(&args.exe).expect("Could not open game binary");
    let exe_mmap =
        unsafe { MmapOptions::new().map(&exe_file) }.expect("Could not mmap game binary");
    let program =
        PeFile::from_bytes(&exe_mmap[0..]).expect("Could not create PE view for game binary");
    let rtti_map = find_rtti_classes(&program)
        .map(|class| (class.name.clone(), class))
        .collect::<HashMap<_, _>>();

    let contents = std::fs::read_to_string(args.profile).expect("Could not read profile file");
    let profile: MapperProfile = toml::from_str(&contents).expect("Could not parse profile TOML");

    let result = profile
        .patterns
        .into_par_iter()
        .flat_map(|entry| entry.find(&program))
        .chain(
            profile
                .vmts
                .into_par_iter()
                .flat_map(|entry| entry.find(&program, &rtti_map)),
        )
        .collect::<Vec<_>>();

    match args.output {
        OutputFormat::Print => println!("Results: {result:#x?}"),
        OutputFormat::Rust => {
            let lines = result
                .iter()
                .map(|r| format!("pub const RVA_{}: u32 = {:#x};", r.name, r.rva))
                .collect::<Vec<_>>();
            println!("{}", lines.join("\n"));
        }
    }
}

/// Profile describing what offsets to extract from a game binary.
#[derive(Debug, Deserialize)]
struct MapperProfile {
    pub patterns: Vec<MapperProfilePattern>,
    pub vmts: Vec<MapperProfileVmt>,
}

/// A Pelite pattern which matches one or more offsets.
#[derive(Debug, Deserialize)]
struct MapperProfilePattern {
    /// Pattern used for matching. Under the hood this uses pelite's parser.
    /// As such, the same pattern syntax is used. More:
    /// https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html
    pattern: String,

    /// Names for the captures. These names can be referenced from the
    /// generated definition file.
    captures: Vec<String>,
}

impl MapperProfilePattern {
    /// Consumes self and looks up the pattern in [program].
    fn find<'a>(self, program: &impl Pe<'a>) -> Vec<MapperEntryResult> {
        let Ok(scanner_pattern) = pattern::parse(&self.pattern) else {
            panic!("Could not parse provided pattern \"{}\"", &self.pattern)
        };

        let mut matches = vec![0u32; self.captures.len()];
        let captures = self
            .captures
            .into_iter()
            .enumerate()
            .filter(|(_, e)| !e.is_empty());

        if !program
            .scanner()
            .matches_code(&scanner_pattern)
            .next(&mut matches)
        {
            captures
                .map(|(_, e)| MapperEntryResult::not_found(e))
                .collect::<Vec<_>>()
        } else {
            captures
                .map(|(i, e)| MapperEntryResult {
                    name: e,
                    rva: matches[i],
                })
                .collect::<Vec<_>>()
        }
    }
}

/// An RTTI class that provides access to its virtual method table.
#[derive(Debug, Deserialize)]
struct MapperProfileVmt {
    /// The class name, according to the RTTI data in the executable.
    class: String,

    /// A map from names for the captures to indexes in the VMT whose values are
    /// be used as VMTs.
    captures: HashMap<String, u32>,
}

impl MapperProfileVmt {
    /// Consumes self and looks up the VMT in [rtti_map].
    fn find<'a, T: Pe<'a>>(
        self,
        program: &T,
        rtti_map: &HashMap<String, Class<'a, T>>,
    ) -> Vec<MapperEntryResult> {
        let Some(class) = rtti_map.get(self.class.as_str()) else {
            panic!("No RTTI class named {}", self.class);
        };

        self.captures
            .into_iter()
            .map(|(name, index)| {
                // Safety: We're not actually dereferencing the VA.
                if let Some(va) = unsafe { class.vmt_fn(index) } {
                    if let Ok(rva) = program.va_to_rva(va) {
                        return MapperEntryResult {
                            name: name,
                            rva,
                        };
                    }
                }

                MapperEntryResult::not_found(name)
            })
            .collect::<Vec<_>>()
    }
}

/// Result of one of the entry items.
#[derive(Debug, Deserialize)]
struct MapperEntryResult {
    pub name: String,
    pub rva: u32,
}

impl MapperEntryResult {
    /// Returns a result indicating that the capture named [name] wasn't found.
    fn not_found(name: impl AsRef<str>) -> Self {
        MapperEntryResult {
            name: name.as_ref().to_string(),
            rva: 0x0,
        }
    }
}
