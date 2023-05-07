use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

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
