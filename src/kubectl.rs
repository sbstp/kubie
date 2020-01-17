use std::process::Command;
use std::str;

use anyhow::anyhow;

pub fn get_namespaces() -> anyhow::Result<Vec<String>> {
    let result = Command::new("kubectl").arg("get").arg("namespaces").output()?;
    if !result.status.success() {
        let stderr = str::from_utf8(&result.stderr).unwrap_or("could not decode stderr of kubectl as utf-8");
        return Err(anyhow!("Error calling kubectl: {}", stderr));
    }

    let text = str::from_utf8(&result.stdout)?;
    let mut namespaces = vec![];
    for line in text.lines().skip(1) {
        let idx = line.find(' ').unwrap_or(line.len());
        namespaces.push(line[..idx].to_string());
    }
    Ok(namespaces)
}
