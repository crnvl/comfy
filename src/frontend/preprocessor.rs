use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const SYSTEM_LIB_PATH: &str = "/usr/include/comfylang";

pub fn preprocess_file(entry_path: &Path, user_include_dirs: &[PathBuf]) -> Result<String, String> {
    let mut seen = HashSet::new();
    expand_includes(entry_path, &mut seen, user_include_dirs)
}

fn expand_includes(
    file_path: &Path,
    seen: &mut HashSet<PathBuf>,
    user_include_dirs: &[PathBuf],
) -> Result<String, String> {
    let canonical_path = fs::canonicalize(file_path).map_err(|e| format!("File not found: {file_path:?} - {e}"))?;

    if !seen.insert(canonical_path.clone()) {
        return Err(format!("Circular include detected: {}", canonical_path.display()));
    }

    let source = fs::read_to_string(&canonical_path)
        .map_err(|e| format!("Failed to read {}: {}", canonical_path.display(), e))?;

    let mut output = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(user_path) = parse_user_include(trimmed) {
            let resolved = resolve_user_include(&user_path, file_path.parent(), user_include_dirs)?;
            let contents = expand_includes(&resolved, seen, user_include_dirs)?;
            output.push_str(&contents);
        } else if let Some(include_path) = parse_system_include(trimmed) {
            let sys_path = Path::new(SYSTEM_LIB_PATH).join(include_path);
            let contents = expand_includes(&sys_path, seen, user_include_dirs)?;
            output.push_str(&contents); 
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    Ok(output)
}

fn parse_system_include(line: &str) -> Option<String> {
    if line.starts_with("#include<") && line.ends_with('>') {
        Some(line.trim_start_matches("#include<").trim_end_matches('>').trim().to_string())
    } else {
        None
    }
}

fn parse_user_include(line: &str) -> Option<String> {
    if line.starts_with("#include<\"") && line.ends_with("\">") {
        Some(
            line.trim_start_matches("#include<\"")
                .trim_end_matches("\">")
                .trim()
                .to_string(),
        )
    } else {
        None
    }
}

fn resolve_user_include(
    name: &str,
    current_dir: Option<&Path>,
    user_paths: &[PathBuf],
) -> Result<PathBuf, String> {
    if let Some(dir) = current_dir {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    for path in user_paths {
        let candidate = path.join(name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(format!("User include not found: {}", name))
}
