use color_eyre::eyre::{ensure, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::Config;

pub fn build_dependencies(config: &Config) -> Result<Vec<(String, PathBuf)>> {
    let manifest_path = match &config.manifest_path {
        Some(path) => path,
        None => return Ok(vec![]),
    };
    let (program, args): (&Path, &[_]) = match &config.dependency_builder {
        Some((path, args)) => (path, args),
        None => (Path::new("cargo"), &[]),
    };
    let mut build = Command::new(program);
    build.args(args);
    build.arg("run");
    if let Some(target) = &config.target {
        build.arg(format!("--target={target}"));
    }
    let output = build
        .arg(format!("--manifest-path={}", manifest_path.display()))
        .arg("--message-format=json")
        .arg("-Zunstable-options")
        .output()?;
    ensure!(output.status.success(), "{output:#?}");
    let output = output.stdout;
    let output = String::from_utf8(output)?;
    Ok(output
        .lines()
        .filter_map(|line| {
            let message = serde_json::from_str::<cargo_metadata::Message>(line).ok()?;
            if let cargo_metadata::Message::CompilerArtifact(artifact) = message {
                let filename = artifact
                    .filenames
                    .into_iter()
                    .find(|filename| filename.extension() == Some("rmeta"))?;
                Some((
                    artifact.package_id.repr.split_once(' ').unwrap().0.to_string(),
                    filename.into_std_path_buf(),
                ))
            } else {
                None
            }
        })
        .collect())
}
