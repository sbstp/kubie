use std::process::Command;
use std::str;

use anyhow::anyhow;

use crate::kubeconfig::KubeConfig;
use crate::tempfile::Tempfile;

pub fn get_namespaces<'a>(kubeconfig: impl Into<Option<&'a KubeConfig>>) -> anyhow::Result<Vec<String>> {
    let mut cmd = Command::new("kubectl");
    cmd.arg("get");
    cmd.arg("namespaces");

    let temp_config_file;

    if let Some(kubeconfig) = kubeconfig.into() {
        temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
        kubeconfig.write_to(&*temp_config_file)?;
        cmd.env("KUBECONFIG", temp_config_file.path());
    }

    let result = cmd.output()?;
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
