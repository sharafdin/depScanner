use serde_json::Value;
use serde_yaml::Value as YamlValue;
use regex::Regex;

pub fn parse_package_lock(content: &str) -> Vec<(String, String)> {
    let parsed: Value = serde_json::from_str(content).expect("Invalid JSON format");
    parsed["dependencies"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, details)| {
            let version = details["version"].as_str().unwrap().to_string();
            (name.clone(), version)
        })
        .collect()
}

pub fn parse_yarn_lock(content: &str) -> Vec<(String, String)> {
    let re = Regex::new(r#"^"([^@]+)@.*":\s+version\s+"([^"]+)""#).unwrap();
    re.captures_iter(content)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect()
}

pub fn parse_pnpm_lock(content: &str) -> Vec<(String, String)> {
    let parsed: YamlValue = serde_yaml::from_str(content).expect("Invalid YAML format");
    parsed["dependencies"]
        .as_mapping()
        .unwrap()
        .iter()
        .map(|(name, details)| {
            let name = name.as_str().unwrap().to_string();
            let version = details["version"].as_str().unwrap().to_string();
            (name, version)
        })
        .collect()
}

pub fn parse_bun_lock(content: &str) -> Vec<(String, String)> {
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_package_json(content: &str) -> Vec<(String, String)> {
    let parsed: Value = serde_json::from_str(content).expect("Invalid JSON format");
    parsed["dependencies"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, version)| (name.clone(), version.as_str().unwrap().to_string()))
        .collect()
}
