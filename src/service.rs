use crate::io::{compress_serde, get_state_location, Params};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub name: String,
    pub pid: i64,
    pub cpu: f32,
    pub memory: f32,
    pub active: String,
    pub enabled: String,
    pub params: Params,
}

impl Status {
    pub fn get_field_names() -> Vec<String> {
        vec![
            "name",
            "pid",
            "cpu",
            "memory",
            "active",
            "enabled",
            "path",
            "pyexec",
            "unit_file_path",
            "systemd_file_path",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }
}

pub fn create_service(
    service_name: &str,
    service_exec: &str,
    service_dir: PathBuf,
) -> Result<(String, String), Error> {
    let unit_file_content = format!(
        r#"
[Unit]
Description={service_name}

[Service]
Type=simple
Restart=always
ExecStart={service_exec}
StandardOutput=syslog+console
StandardError=syslog+console
Restart=on-failure

[Install]
WantedBy=multi-user.target
"#,
        service_name = service_name,
        service_exec = service_exec,
    );

    let unit_file_path = service_dir.join(format!("{}.service", service_name));
    let mut file = fs::File::create(&unit_file_path)?;
    file.write_all(unit_file_content.as_bytes())?;

    let systemd_dir = Path::new("/etc/systemd/system");
    let systemd_file_path = systemd_dir.join(format!("{}.service", service_name));

    if systemd_file_path.exists() {
        fs::remove_file(&systemd_file_path)?;
    }
    symlink(&unit_file_path, &systemd_file_path)?;

    // enable and start
    enable_and_start_service(service_name)?;

    Ok((
        unit_file_path.to_string_lossy().into_owned(),
        systemd_file_path.to_string_lossy().into_owned(),
    ))
}

fn enable_and_start_service(service_name: &str) -> Result<(), Error> {
    // Enable service
    let output = Command::new("systemctl")
        .arg("enable")
        .arg(service_name)
        .output()?;

    if !output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Failed to enable service `{}`: {}",
                service_name,
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // Start service
    let output = Command::new("systemctl")
        .arg("start")
        .arg(service_name)
        .output()?;

    if !output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Failed to start service: `{}`: {}",
                service_name,
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    Ok(())
}

pub fn get_service_status(service_name: &str, home_dir: &str) -> Result<Vec<Status>, Error> {
    let state_file = get_state_location(home_dir);
    let map = compress_serde::decompress_from_file(state_file)?;
    if map.is_empty() {
        return Ok(Vec::<Status>::new());
    }
    if service_name == "all" {
        let statuses: Result<Vec<Status>, Error> = map
            .into_iter()
            .map(|(k, v)| {
                let service_name = format!("{}.service", k);
                let pid = get_service_pid(service_name.as_str())?;
                let (cpu, mem) = get_resource_usage(pid)?;
                let (active, enabled) = get_active_enabled(service_name.clone())?;
                Ok(Status {
                    name: k,
                    pid,
                    cpu,
                    memory: mem,
                    active,
                    enabled,
                    params: v,
                })
            })
            .collect();
        return statuses;
    }
    let params = map.get(service_name);
    if params.is_none() {
        return Ok(Vec::<Status>::new());
    } else {
        let service_params = params.unwrap().to_owned();
        // check pid
        let pid = get_service_pid(format!("{}.service", service_name).as_str())?;
        let (cpu, mem) = get_resource_usage(pid)?;
        let (active, enabled) = get_active_enabled(format!("{}.service", service_name))?;
        Ok(vec![Status {
            name: service_name.parse().unwrap(),
            pid,
            cpu,
            memory: mem,
            active,
            enabled,
            params: service_params,
        }])
    }
}

fn get_service_pid(service_name: &str) -> Result<i64, Error> {
    let pid = Command::new("systemctl")
        .arg("show")
        .arg("--property")
        .arg("MainPID")
        .arg(service_name)
        .output()?;

    String::from_utf8_lossy(&pid.stdout)
        .trim()
        .split("=")
        .nth(1)
        .ok_or_else(|| Error::new(ErrorKind::Other, "Couldn't extract MainPID"))
        .and_then(|s| {
            s.parse::<i64>()
                .map_err(|_| Error::new(ErrorKind::Other, "Invalid MainPID value"))
        })
}

fn get_resource_usage(pid: i64) -> Result<(f32, f32), Error> {
    let ps_output = Command::new("ps")
        .arg("-o")
        .arg("%cpu,%mem")
        .arg("-p")
        .arg(pid.to_string())
        .stdout(Stdio::piped())
        .output()?;

    let output_str = String::from_utf8_lossy(&ps_output.stdout);
    let cpu_mem_percentages: Vec<&str> = output_str.trim().split_whitespace().collect();

    let cpu_percentage = cpu_mem_percentages
        .get(0)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);

    let mem_percentage = cpu_mem_percentages
        .get(1)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);

    Ok((cpu_percentage, mem_percentage))
}

fn get_active_enabled(service_name: String) -> Result<(String, String), Error> {
    let active = Command::new("systemctl")
        .arg("is-active")
        .arg(&service_name)
        .output()?;

    let enabled = Command::new("systemctl")
        .arg("is-enabled")
        .arg(&service_name)
        .output()?;

    Ok((
        String::from_utf8_lossy(&active.stdout).trim().to_string(),
        String::from_utf8_lossy(&enabled.stdout).trim().to_string(),
    ))
}
