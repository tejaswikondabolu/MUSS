use std::process::Command;
use std::fs;
use std::io::Write;

pub struct CompilationResult {
    pub success: bool,
    pub errors: Vec<CompilerError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub code: Option<String>,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

pub fn analyze_code(code: &str) -> CompilationResult {
    let tmp_dir = std::env::temp_dir().join("museum_heresy");
    fs::create_dir_all(&tmp_dir).ok();

    let file_path = tmp_dir.join("analyze.rs");
    let mut file = fs::File::create(&file_path).expect("Failed to create temp file");
    file.write_all(code.as_bytes()).expect("Failed to write code");

    let output = Command::new("rustc")
        .arg(&file_path)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("Failed to execute rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        return CompilationResult {
            success: true,
            errors: vec![],
            warnings: extract_warnings(&stderr),
        };
    }

    let errors = parse_errors(&stderr);
    let warnings = extract_warnings(&stderr);

    CompilationResult {
        success: false,
        errors,
        warnings,
    }
}

fn parse_errors(stderr: &str) -> Vec<CompilerError> {
    let mut errors = vec![];
    let mut current_error = None;

    for line in stderr.lines() {
        if line.starts_with("error[") {
            if let Some(prev) = current_error.take() {
                errors.push(prev);
            }
            let code_end = line.find(']').unwrap_or(0);
            let code = Some(line[6..code_end].to_string());
            let msg = if code_end + 2 < line.len() {
                line[code_end + 2..].to_string()
            } else {
                String::new()
            };
            current_error = Some(CompilerError {
                code,
                message: msg.clone(),
                line: None,
                column: None,
            });
        } else if line.trim().starts_with("--> ") && current_error.is_some() {
            let loc_part = line.trim().trim_start_matches("--> ");
            if let Some(line_col) = loc_part.rsplit(':').next() {
                if line_col.contains(',') {
                    let parts: Vec<&str> = line_col.split(',').collect();
                    if let Some(l) = parts.first().and_then(|p| p.trim().parse().ok()) {
                        if let Some(e) = current_error.as_mut() {
                            e.line = Some(l);
                        }
                    }
                    if let Some(c) = parts.get(1).and_then(|p| p.trim().parse().ok()) {
                        if let Some(e) = current_error.as_mut() {
                            e.column = Some(c);
                        }
                    }
                } else if let Ok(l) = line_col.parse() {
                    if let Some(e) = current_error.as_mut() {
                        e.line = Some(l);
                    }
                }
            }
        } else if line.starts_with("   = ") && current_error.is_some() {
            if let Some(e) = current_error.as_mut() {
                e.message.push_str(&format!("\n  {}", line.trim_start_matches("   = ")));
            }
        } else if line.starts_with("  help: ") && current_error.is_some() {
            if let Some(e) = current_error.as_mut() {
                e.message.push_str(&format!("\n  Help: {}", line.trim_start_matches("  help: ")));
            }
        } else if line.starts_with("error: ") && !line.contains('[') {
            if let Some(prev) = current_error.take() {
                errors.push(prev);
            }
            current_error = Some(CompilerError {
                code: None,
                message: line.trim_start_matches("error: ").to_string(),
                line: None,
                column: None,
            });
        }
    }

    if let Some(err) = current_error.take() {
        errors.push(err);
    }

    errors
}

fn extract_warnings(stderr: &str) -> Vec<String> {
    stderr.lines()
        .filter(|l| l.contains("warning:"))
        .map(|l| l.to_string())
        .collect()
}
