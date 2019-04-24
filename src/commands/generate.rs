use crate::user::settings::{ProjectSettings, ProjectType};
use crate::{commands, install};
use binary_install::Cache;
use std::process::Command;

pub fn generate(name: &str, template: &str, cache: &Cache) -> Result<(), failure::Error> {
    let tool_name = "cargo-generate";
    let binary_path = install::install(tool_name, "ashleygwilliams", cache)?.binary(tool_name)?;

    let worker_init = format!(
        "{} generate --git {} --name {}",
        binary_path.to_string_lossy(),
        template,
        name
    );

    let project_type = project_type(template);

    commands::run(command(&worker_init, &project_type, name), &worker_init)?;
    ProjectSettings::generate(name.to_string(), project_type)?;
    Ok(())
}

fn command(cmd: &str, project_type: &ProjectType, name: &str) -> Command {
    println!(
        "🐑 Generating a new {} worker project with name '{}'...",
        project_type, name
    );

    if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(&["/C", cmd]);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c");
        c.arg(cmd);
        c
    }
}

fn project_type(template: &str) -> ProjectType {
    if template.contains("rust") {
        return ProjectType::Rust;
    }
    ProjectType::JavaScript
}
