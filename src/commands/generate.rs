use crate::user::settings::ProjectSettings;
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

    let template_type = template_type(template);

    commands::run(command(&worker_init, template_type, name), &worker_init)?;
    ProjectSettings::generate(name.to_string())?;
    Ok(())
}

fn command(cmd: &str, template_type: &str, name: &str) -> Command {
    println!(
        "🐑 Generating a new {} worker project with name '{}'...",
        template_type, name
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

fn template_type(template: &str) -> &str {
    if template.contains("rust") {
        return "rust";
    }
    "js"
}
