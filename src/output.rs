use colored::*;

pub fn display_results(results: &[(String, String, Option<String>)]) {
    println!("{}", "Dependency Vulnerability Report".bold());
    println!("{:<30} {:<15} {:<50}", "Package", "Version", "Issues");
    println!("{}", "-".repeat(80));

    for (name, version, vulnerability) in results {
        if let Some(vuln) = vulnerability {
            println!(
                "{:<30} {:<15} {:<50}",
                name.red(),
                version,
                vuln.yellow()
            );
        } else {
            println!(
                "{:<30} {:<15} {:<50}",
                name.green(),
                version,
                "No issues found".green()
            );
        }
    }
}

pub fn display_summary(total: usize, vulnerable: usize) {
    println!("\n{}", "Scan Summary".bold());
    println!("{}", "-".repeat(30));
    println!("Total dependencies scanned: {}", total);
    println!("Vulnerable dependencies: {}", vulnerable.to_string().red());
    println!("Safe dependencies: {}", (total - vulnerable).to_string().green());
}
