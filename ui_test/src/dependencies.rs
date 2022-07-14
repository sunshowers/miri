use color_eyre::eyre::{bail, Result};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::Config;

/// Compiles dependencies and returns the crate names and corresponding rmeta files.
pub fn build_dependencies(config: &Config) -> Result<Vec<(String, PathBuf)>> {
    let manifest_path = match &config.manifest_path {
        Some(path) => path,
        None => return Ok(vec![]),
    };
    let (program, args, envs): (&Path, &[_], &[_]) = match &config.dependency_builder {
        Some(db) => (&db.program, &db.args, &db.envs),
        None => (Path::new("cargo"), &[], &[]),
    };
    let mut build = Command::new(program);

    // Avoid poisoning the sysroot and causing unnecessary rebuilds.
    build.env_remove("RUSTFLAGS");

    build.envs(envs.iter().map(|(k, v)| (k, v)));
    build.args(args);
    build.arg("run");
    if let Some(target) = &config.target {
        build.arg(format!("--target={target}"));
    }
    build
        .arg(format!("--manifest-path={}", manifest_path.display()))
        .arg("--target-dir=target/test_dependencies")
        .arg("--message-format=json")
        .arg("-Zunstable-options");

    let output = build.output()?;

    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        bail!("failed to compile dependencies:\nstderr:\n{stderr}\n\nstdout:{stdout}");
    }

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
