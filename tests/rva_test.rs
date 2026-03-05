use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::Result;
use glob::glob;
use regex_macro::regex;
use serde::Deserialize;

/// We just describe the subset of the mapper profile that we actually
/// validate.
#[derive(Debug, Deserialize)]
struct MapperProfile {
    #[serde(default)]
    pub patterns: Vec<MapperProfilePattern>,
    #[serde(default)]
    pub vmts: Vec<MapperProfileVmt>,
}

#[derive(Debug, Deserialize)]
struct MapperProfilePattern {
    captures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MapperProfileVmt {
    #[serde(default)]
    captures: HashMap<String, u32>,

    // A name for the capture of the virtual method table itself.
    vftable: Option<String>,
}

/// Converts a [Path] that's relative to the `test` directory into a [String]
/// that's relative to the root of the repo.
fn clean_path<'a>(path: impl AsRef<Path> + 'a) -> String {
    let mut components = path.as_ref().components().collect::<Vec<_>>();

    let mut buf = PathBuf::new();
    if matches!(&components[..], [Component::ParentDir, ..]) {
        components.remove(0);
    } else {
        buf.push("test");
    }

    for component in components {
        buf.push(component);
    }
    buf.to_string_lossy().into()
}

/// Loads all RVAs defined in the RVA data file at `path`.
fn load_rvas(path: impl AsRef<Path>) -> Result<Vec<(String, u64)>> {
    let path = path.as_ref();
    let rvas = regex!("(?im)^ +([^:]+): 0x([0-9a-f]+),\r?$")
        .captures_iter(&fs::read_to_string(path)?)
        .map(|c| {
            Ok((
                c.get(1).unwrap().as_str().to_string(),
                u64::from_str_radix(c.get(2).unwrap().as_str(), 16)?,
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    if rvas.is_empty() {
        panic!("{} doesn't seem to have any RVAs", clean_path(path))
    } else {
        Ok(rvas)
    }
}

// Verify that all RVAs have real values and aren't the default 0x0.
#[test]
fn rvas_non_zero() -> Result<()> {
    for path in glob("../crates/*/src/rva/rva_*.rs")? {
        let path = path?;
        for (name, rva) in load_rvas(&path)? {
            if rva == 0 {
                panic!("RVA {} in {} is 0x0", name, clean_path(path));
            }
        }
    }
    Ok(())
}

// Verify that all RVAs defined in mapper profiles have been generated.
#[test]
fn all_rvas_generated() -> Result<()> {
    for profile_path in glob("../crates/*/mapper-profile.toml")? {
        let profile_path = profile_path?;
        let profile = match toml::from_str::<MapperProfile>(&fs::read_to_string(&profile_path)?) {
            Ok(profile) => profile,
            Err(err) => panic!("Failed to read {}: {:?}", clean_path(profile_path), err),
        };
        let mapper_rva_names = profile
            .patterns
            .into_iter()
            .flat_map(|p| p.captures.into_iter().filter(|c| !c.is_empty()))
            .chain(
                profile
                    .vmts
                    .into_iter()
                    .flat_map(|v| v.captures.into_keys().chain(v.vftable.into_iter())),
            )
            .collect::<HashSet<_>>();

        let mut crate_dir = PathBuf::from(profile_path.clone());
        crate_dir.pop();
        for rva_path in glob(&format!(
            "{}/src/rva/rva_*.rs",
            crate_dir.to_str().expect("Invalid UTF-8 in path"),
        ))? {
            let rva_path = rva_path?;
            let generated_rva_names = load_rvas(&rva_path)?
                .into_iter()
                .map(|(name, _)| name)
                .collect::<HashSet<_>>();

            let not_generated = mapper_rva_names
                .difference(&generated_rva_names)
                .cloned()
                .collect::<Vec<_>>();
            if !not_generated.is_empty() {
                panic!(
                    "{} contains RVAs that haven't been generated for {}: {}",
                    clean_path(profile_path),
                    clean_path(rva_path),
                    not_generated.join(", ")
                );
            }

            let stale = generated_rva_names
                .difference(&mapper_rva_names)
                .cloned()
                .collect::<Vec<_>>();
            if !stale.is_empty() {
                panic!(
                    "{} contains RVAs that aren't specified in {}: {}",
                    clean_path(rva_path),
                    clean_path(profile_path),
                    stale.join(", ")
                );
            }
        }
    }
    Ok(())
}
