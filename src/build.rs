use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    // Setup target directory and file
    let generated_path = Path::new("./src/generated/dotenv.rs");
    std::fs::create_dir_all("./src/generated")?;
    let mut file = std::fs::File::create(generated_path)?;

    // Load environment variables from .env file
    let envs = parse_env_file("./.env")?;

    for (key, value) in envs {
        let key = key.to_uppercase();
        let line = format!(
            "#[allow(dead_code)] pub(crate) const {}: &str = \"{}\";\n",
            key, value,
        );
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn parse_env_file(file_path: &str) -> anyhow::Result<HashMap<String, String>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut env_map = HashMap::new();
    lines.for_each(|line| {
        if let Ok(line) = line {
            if !line.trim().is_empty() && !line.starts_with('#') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    env_map.insert(
                        parts[0].to_string(),
                        parts[1]
                            .to_string()
                            .replace('\"', ""),
                    );
                }
            }
        }
    });

    Ok(env_map)
}
