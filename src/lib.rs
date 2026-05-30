use std::fmt::Write;
use zed_extension_api::{self as zed, Result, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, Worktree};

struct MiseExtension;

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "tools" => {
                let output = self.run_mise(&["ls", "--offline", "--json"], worktree)?;
                Ok(SlashCommandOutput {
                    text: self.format_tools(&output),
                    sections: vec![],
                })
            }
            "tasks" => {
                let output = self.run_mise(&["tasks", "ls", "--json"], worktree)?;
                Ok(SlashCommandOutput {
                    text: self.format_tasks(&output),
                    sections: vec![],
                })
            }
            "env" => {
                let output = self.run_mise(&["env", "--json"], worktree)?;
                Ok(SlashCommandOutput {
                    text: self.format_env(&output),
                    sections: vec![],
                })
            }
            "doctor" => {
                let output = self.run_mise(&["doctor"], worktree)?;
                Ok(SlashCommandOutput {
                    text: format!("```\n{}\n```", output),
                    sections: vec![],
                })
            }
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        Ok(vec![])
    }
}

impl MiseExtension {
    fn find_mise(&self, worktree: Option<&Worktree>) -> Result<String, String> {
        if let Some(worktree) = worktree {
            if let Some(path) = worktree.which("mise") {
                return Ok(path);
            }
        }
        Err("mise binary not found in PATH. Install it from https://mise.jdx.dev".to_string())
    }

    fn run_mise(
        &self,
        args: &[&str],
        worktree: Option<&Worktree>,
    ) -> Result<String, String> {
        let mise_path = self.find_mise(worktree)?;
        let output = zed::process::Command::new(mise_path)
            .args(args.iter().map(|s| s.to_string()))
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn format_tools(&self, json: &str) -> String {
        let tools: Vec<serde_json::Value> = serde_json::from_str(json).unwrap_or_default();
        let mut out = String::new();
        for tool in &tools {
            let name = tool["name"].as_str().unwrap_or("?");
            let version = tool["version"].as_str().unwrap_or("?");
            let request = tool["request_version"].as_str().unwrap_or("");
            let source = tool["source"]
                .as_object()
                .and_then(|s| s.get("path"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let _ = writeln!(out, "- **{name}** `{version}`",);
            if !request.is_empty() && request != version {
                let _ = writeln!(out, "  - requested: {request}");
            }
            if !source.is_empty() {
                let _ = writeln!(out, "  - from: {source}");
            }
        }
        if out.is_empty() {
            out = "*No tools configured*".to_string();
        }
        out
    }

    fn format_tasks(&self, json: &str) -> String {
        let tasks: Vec<serde_json::Value> = serde_json::from_str(json).unwrap_or_default();
        let mut out = String::new();
        for task in &tasks {
            let name = task["name"].as_str().unwrap_or("?");
            let description = task["description"].as_str().unwrap_or("");
            let source = task["source"]
                .as_object()
                .and_then(|s| s.get("path"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let _ = writeln!(out, "- **{name}**",);
            if !description.is_empty() {
                let _ = writeln!(out, "  - {description}");
            }
            if !source.is_empty() {
                let _ = writeln!(out, "  - from: {source}");
            }
        }
        if out.is_empty() {
            out = "*No tasks found*".to_string();
        }
        out
    }

    fn format_env(&self, json: &str) -> String {
        let env: serde_json::Value = serde_json::from_str(json).unwrap_or_default();
        let obj = match env.as_object() {
            Some(obj) => obj,
            None => return "*No environment variables*".to_string(),
        };
        let mut out = String::new();
        for (key, val) in obj {
            let val_str = val.as_str().unwrap_or("?");
            let _ = writeln!(out, "- `{key}` = `{val_str}`");
        }
        if out.is_empty() {
            out = "*No environment variables*".to_string();
        }
        out
    }
}

zed::register_extension!(MiseExtension);
