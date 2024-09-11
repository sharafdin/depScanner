use std::fs;
use std::env;

mod parsers;
mod vulnerability_checker;
mod output;

#[tokio::main]
async fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <lockfile-or-package.json>", args[0]);
        return;
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Failed to read the file");

    // Determine file type and parse dependencies
    let dependencies = if file_path.ends_with("lock.json")
        || file_path.ends_with("yaml")
        || file_path.ends_with("lockb")
    {
        match file_path.split('.').last().unwrap() {
            "json" if file_path.contains("package-lock") => parsers::parse_package_lock(&content),
            "lock" if file_path.contains("yarn") => parsers::parse_yarn_lock(&content),
            "yaml" | "yml" if file_path.contains("pnpm-lock") => parsers::parse_pnpm_lock(&content),
            "lockb" => parsers::parse_bun_lock(&content),
            _ => {
                eprintln!("Unsupported lock file format");
                return;
            }
        }
    } else if file_path.ends_with("package.json") {
        parsers::parse_package_json(&content)
    } else {
        eprintln!("Unsupported file type");
        return;
    };

    println!("Scanning dependencies...");
    let mut results = Vec::new();

    for (name, version) in dependencies {
        if let Some(vulnerability) = vulnerability_checker::check_vulnerabilities(&name, &version).await {
            results.push((name, version, Some(vulnerability)));
        } else {
            results.push((name, version, None));
        }
    }

    // Display results and summary
    output::display_results(&results);
    output::display_summary(results.len(), results.iter().filter(|r| r.2.is_some()).count());
}
