use std::fs;

pub fn read_file(path_str: &str) -> eyre::Result<String> {
    let content = fs::read_to_string(path_str)?;

    Ok(content)
}

