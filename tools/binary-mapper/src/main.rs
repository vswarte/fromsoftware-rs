use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{collections::HashMap, fs, fs::File};

use clap::{Args, Parser, ValueEnum};
use fromsoftware_shared::{Class, find_rtti_classes};
use memmap::MmapOptions;
use pelite::{
    pattern,
    pe64::{Pe, PeFile},
};
use rayon::prelude::*;
use serde::Deserialize;

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Print,
    RustStruct,
    Rust,
}

/// Finds RVAs within a binary and emits them as code.
#[derive(Parser)]
enum BinaryMapper {
    Map(MapArgs),
    #[command(name = "er")]
    EldenRing(EldenRingArgs),
    #[command(name = "ds3")]
    DarkSoulsIII(DarkSoulsIIIArgs),
    #[command(name = "nr")]
    Nightreign(NightreignArgs),
}

/// Maps a single EXE to a single output and prints it to stdout.
#[derive(Args)]
struct MapArgs {
    #[arg(long, env("MAPPER_PROFILE"))]
    profile: PathBuf,

    #[arg(long, env("MAPPER_GAME_EXE"))]
    exe: Option<PathBuf>,

    #[arg(long, env("MAPPER_OUTPUT_FORMAT"))]
    output: OutputFormat,
}

/// Shortcut to map all files for Elden Ring.
#[derive(Args)]
struct EldenRingArgs {
    /// The worldwide EXE for patch 2.6.1.
    #[arg(long, env("MAPPER_ER_WW_EXE"))]
    ww_exe: PathBuf,

    /// The Japanese EXE for patch 2.6.1.1.
    #[arg(long, env("MAPPER_ER_JP_EXE"))]
    jp_exe: PathBuf,

    /// Root for the project folder.
    #[arg(long, env("MAPPER_ER_PROJECT_ROOT"))]
    project_root: Option<PathBuf>,
}

/// Shortcut to map all files for DarkSouls III.
#[derive(Args)]
struct DarkSoulsIIIArgs {
    /// The EXE for patch 1.15.2 (Japenese or worldwide, either workds).
    #[arg(long, env("MAPPER_DS3_EXE"))]
    exe: PathBuf,

    /// Root for the project folder.
    #[arg(long, env("MAPPER_DS3_PROJECT_ROOT"))]
    project_root: Option<PathBuf>,
}

/// Shortcut to map all files for Elden Ring.
#[derive(Args)]
struct NightreignArgs {
    /// Path to the worldwide EXE.
    #[arg(long, env("MAPPER_NR_WW_EXE"))]
    ww_exe: PathBuf,

    // /// Path to the Japanese EXE.
    // #[arg(long, env("MAPPER_NR_JP_EXE"))]
    // jp_exe: PathBuf,

    /// Root for the project folder.
    #[arg(long, env("MAPPER_ER_PROJECT_ROOT"))]
    project_root: Option<PathBuf>,
}

fn main() {
    match BinaryMapper::parse() {
        BinaryMapper::Map(args) => {
            let profile = read_profile(args.profile);
            if let OutputFormat::RustStruct = args.output {
                print!("{}", generate_rust_struct(&profile));
                return;
            }

            let results = map_results(
                &profile,
                &args.exe.unwrap_or_else(|| {
                    panic!(
                        "exe must be passed with --output {}",
                        args.output.to_possible_value().unwrap().get_name()
                    )
                }),
            );

            match args.output {
                OutputFormat::Print => println!("Results: {results:#x?}"),
                OutputFormat::Rust => println!("{}", generate_rust_instance(&results)),
                OutputFormat::RustStruct => { /* handled above */ }
            }
        }
        BinaryMapper::EldenRing(args) => {
            let er = args
                .project_root
                .inspect(|r| {
                    assert!(r.exists(), "Project root does not exist: {}", r.display());
                })
                .unwrap_or_else(|| game_crate_path("eldenring"));
            let profile = read_profile(er.join("mapper-profile.toml"));
            fs::write(er.join("src/rva/bundle.rs"), generate_rust_struct(&profile)).unwrap();
            fs::write(
                er.join("src/rva/rva_ww.rs"),
                generate_rust_instance(&map_results(&profile, &args.ww_exe)),
            )
            .unwrap();
            fs::write(
                er.join("src/rva/rva_jp.rs"),
                generate_rust_instance(&map_results(&profile, &args.jp_exe)),
            )
            .unwrap();
            cargo_fmt(&er);
        }
        BinaryMapper::DarkSoulsIII(args) => {
            let ds3 = args
                .project_root
                .inspect(|r| {
                    assert!(r.exists(), "Project root does not exist: {}", r.display());
                })
                .unwrap_or_else(|| game_crate_path("darksouls3"));
            let profile = read_profile(ds3.join("mapper-profile.toml"));
            fs::write(
                ds3.join("src/rva/bundle.rs"),
                generate_rust_struct(&profile),
            )
            .unwrap();
            fs::write(
                ds3.join("src/rva/rva_data.rs"),
                generate_rust_instance(&map_results(&profile, &args.exe)),
            )
            .unwrap();
            cargo_fmt(&ds3);
        }
        BinaryMapper::Nightreign(args) => {
            let nr = args
                .project_root
                .inspect(|r| {
                    assert!(r.exists(), "Project root does not exist: {}", r.display());
                })
                .unwrap_or_else(|| game_crate_path("nightreign"));
            let profile = read_profile(nr.join("mapper-profile.toml"));
            fs::write(nr.join("src/rva/bundle.rs"), generate_rust_struct(&profile)).unwrap();
            fs::write(
                nr.join("src/rva/rva_ww.rs"),
                generate_rust_instance(&map_results(&profile, &args.ww_exe)),
            )
            .unwrap();
            // fs::write(
            //     nr.join("src/rva/rva_jp.rs"),
            //     generate_rust_instance(&map_results(&profile, &args.jp_exe)),
            // )
            // .unwrap();
            cargo_fmt(&nr);
        }
    }
}

/// Reads a mapper profile from disk at `path`.
fn read_profile<P: AsRef<Path>>(path: P) -> MapperProfile {
    let contents = fs::read_to_string(path).expect("Could not read profile file");
    toml::from_str(&contents).expect("Could not parse profile TOML")
}

/// Returns the path to the game crate named `basename` in this repo.
///
/// Panics if this isn't being run from within the fromsoftware-rs repo.
fn game_crate_path<P: AsRef<Path>>(basename: P) -> PathBuf {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("../../../crates");
    path.push(basename);

    if !path.is_dir() {
        panic!(
            "{} doesn't exist, shortcut commands must be run within the fromsoftware-rs repo",
            path.display()
        );
    }

    path
}

/// Runs `cargo fmt` in `path`.
fn cargo_fmt<P: AsRef<Path>>(path: P) {
    Command::new("cargo")
        .arg("fmt")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path)
        .status()
        .unwrap();
}

/// Loads the results for `profile` from the binary at `exe`.
fn map_results(profile: &MapperProfile, exe: &Path) -> Vec<MapperEntryResult> {
    let exe_file = File::open(exe).expect("Could not open game binary");
    let exe_mmap =
        unsafe { MmapOptions::new().map(&exe_file) }.expect("Could not mmap game binary");
    let program =
        PeFile::from_bytes(&exe_mmap[0..]).expect("Could not create PE view for game binary");
    let rtti_map = find_rtti_classes(&program)
        .map(|class| (class.name.clone(), class))
        .collect::<HashMap<_, _>>();

    let mut results = profile
        .patterns
        .par_iter()
        .flat_map(|entry| entry.find(&program))
        .chain(
            profile
                .vmts
                .par_iter()
                .flat_map(|entry| entry.find(&program, &rtti_map)),
        )
        .collect::<Vec<_>>();
    results.sort_by(|r1, r2| r1.name.cmp(&r2.name));
    results
}

/// Generates a Rust struct with fields for each RVA lsited in the given
/// `profile`.
fn generate_rust_struct(profile: &MapperProfile) -> String {
    let mut output = String::from(
        "//! A generated RVA struct.\n\
        \n\
        // DO NOT EDIT THIS FILE DIRECTLY.\n\
        \n\
        /// A struct containing offsets (relative to the beginning of the executable) of\n\
        /// various addresses of structures and functions. They can be converted to a\n\
        /// usable address using the [Pe::rva_to_va](pelite::pe64::Pe::rva_to_va) trait function\n\
        /// of [Program](fromsoftware_shared::Program).\n\
        ///\n\
        /// These are populated from `mapper-profile.toml` in the root of this package\n\
        /// using `tools/binary-generator`.\n\
        pub struct RvaBundle {\n",
    );

    let mut fields = profile
        .patterns
        .iter()
        .flat_map(|entry| &entry.captures)
        .filter(|name| !name.is_empty())
        .chain(
            profile
                .vmts
                .iter()
                .flat_map(|entry| entry.captures.keys().chain(entry.vftable.iter())),
        )
        .collect::<Vec<_>>();
    fields.sort();
    for field in fields {
        writeln!(output, "pub {}: u32,", field).unwrap();
    }

    output.push('}');
    output
}

/// Generates a file that declares an instance of `RvaBundle` with the given
/// `results`.
fn generate_rust_instance(results: &[MapperEntryResult]) -> String {
    let mut output = String::from(
        "//! Generated RVA mappings for a single executable.\n\
                 \n\
                 // DO NOT EDIT THIS FILE DIRECTLY.\n\
                 \n\
                 use super::RvaBundle;\n\
                 \n\
                 /// The RVAs for this executable.\n\
                 ///\n\
                 /// These are populated from `mapper-profile.toml` in the root of this package\n\
                 /// using `tools/binary-generator`.\n\
                 pub const RVAS: RvaBundle = RvaBundle {\n",
    );
    for result in results {
        writeln!(output, "{}: {:#x},", result.name, result.rva).unwrap();
    }
    output.push_str("};");
    output
}

/// Profile describing what offsets to extract from a game binary.
#[derive(Debug, Deserialize)]
struct MapperProfile {
    #[serde(default)]
    pub patterns: Vec<MapperProfilePattern>,
    #[serde(default)]
    pub vmts: Vec<MapperProfileVmt>,
}

/// A Pelite pattern which matches one or more offsets.
#[derive(Debug, Deserialize)]
struct MapperProfilePattern {
    /// Pattern used for matching. Under the hood this uses pelite's parser.
    /// As such, the [same pattern syntax] is used.
    ///
    /// [same pattern syntax]: https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html
    pattern: String,

    /// Names for the captures. These names can be referenced from the
    /// generated definition file.
    captures: Vec<String>,
}

impl MapperProfilePattern {
    /// Consumes self and looks up the pattern in `program`.
    fn find<'a>(&self, program: &impl Pe<'a>) -> Vec<MapperEntryResult> {
        let Ok(scanner_pattern) = pattern::parse(&self.pattern) else {
            panic!("Could not parse provided pattern \"{}\"", &self.pattern)
        };

        let mut matches = vec![0u32; self.captures.len()];
        let captures = self
            .captures
            .iter()
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
                    name: e.clone(),
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
    #[serde(default)]
    captures: HashMap<String, u32>,

    // A name for the capture of the virtual method table itself.
    vftable: Option<String>,
}

impl MapperProfileVmt {
    /// Consumes self and looks up the VMT in `rtti_map`.
    fn find<'a, T: Pe<'a>>(
        &self,
        program: &T,
        rtti_map: &HashMap<String, Class<'a, T>>,
    ) -> Vec<MapperEntryResult> {
        let Some(class) = rtti_map.get(self.class.as_str()) else {
            panic!("No RTTI class named {}", self.class);
        };

        self.captures
            .iter()
            .map(|(name, index)| {
                // Safety: We're not actually dereferencing the VA.
                if let Some(va) = unsafe { class.vmt_fn(*index) }
                    && let Ok(rva) = program.va_to_rva(va)
                {
                    return MapperEntryResult {
                        name: name.clone(),
                        rva,
                    };
                }

                MapperEntryResult::not_found(name)
            })
            .chain(self.vftable.iter().map(|name| MapperEntryResult {
                name: name.clone(),
                rva: class.vftable,
            }))
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
    /// Returns a result indicating that the capture named `name` wasn't found.
    fn not_found(name: impl AsRef<str>) -> Self {
        MapperEntryResult {
            name: name.as_ref().to_string(),
            rva: 0x0,
        }
    }
}
