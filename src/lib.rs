use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub const SERVICE: &str = "SonarQube";
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    Light,
    #[default]
    Dark,
}

pub fn load_appearance() -> Result<Appearance> {
    let path = settings_path().with_file_name("appearance.json");
    match fs::read(path) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Appearance::default()),
        Err(error) => Err(error.into()),
    }
}

pub fn save_appearance(appearance: Appearance) -> Result<()> {
    let path = settings_path().with_file_name("appearance.json");
    fs::create_dir_all(path.parent().context("Missing preferences directory")?)?;
    fs::write(path, serde_json::to_vec(&appearance)?)?;
    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub folder: PathBuf,
    pub host: String,
    pub port: u16,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            folder: PathBuf::from(r"C:\sonarqube"),
            host: "127.0.0.1".into(),
            port: 9000,
        }
    }
}
pub fn settings_path() -> PathBuf {
    PathBuf::from(std::env::var_os("PROGRAMDATA").unwrap_or_else(|| ".".into()))
        .join("SonarQubeManager/settings.json")
}
pub fn load() -> Result<Settings> {
    let path = settings_path();
    if !path.exists() {
        return Ok(Settings::default());
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
pub fn installed_folder() -> Result<PathBuf> {
    let mut cmd = command("sc.exe");
    cmd.args(["qc", SERVICE]);
    folder_from_service_config(&checked(cmd)?)
}
pub fn folder_from_service_config(config: &str) -> Result<PathBuf> {
    let suffix = r"\bin\windows-x86-64\lib\SonarServiceWrapper.exe";
    let line = config
        .lines()
        .find(|line| {
            line.replace('/', "\\")
                .to_ascii_lowercase()
                .contains(&suffix.to_ascii_lowercase())
        })
        .context("Existing service is not a supported SonarQube bundled Windows service")?;
    let value = line
        .split_once(':')
        .context("Missing service executable path")?
        .1
        .trim();
    let value = value.replace('/', "\\");
    let executable = if let Some(quoted) = value.strip_prefix('"') {
        quoted
            .split_once('"')
            .context("Unclosed service executable quote")?
            .0
    } else {
        let end = value
            .to_ascii_lowercase()
            .find(".exe")
            .context("Missing service executable")?
            + 4;
        &value[..end]
    };
    if !executable
        .to_ascii_lowercase()
        .ends_with(&suffix.to_ascii_lowercase())
    {
        bail!("Service executable is not the SonarQube bundled wrapper");
    }
    let folder = &executable[..executable.len() - suffix.len()];
    if folder.is_empty() {
        bail!("Cannot determine installed SonarQube folder");
    }
    Ok(PathBuf::from(folder))
}
pub fn settings_from_properties(folder: PathBuf, properties: &str) -> Result<Settings> {
    let mut settings = Settings {
        folder,
        host: "0.0.0.0".into(),
        port: 9000,
    };
    for line in properties.lines().map(str::trim) {
        if line.starts_with(['#', '!']) {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "sonar.web.host" => settings.host = value.trim().into(),
                "sonar.web.port" => {
                    settings.port = value
                        .trim()
                        .parse()
                        .context("Invalid sonar.web.port in installed configuration")?
                }
                _ => {}
            }
        }
    }
    Ok(settings)
}
pub fn existing_settings() -> Result<Settings> {
    let folder = installed_folder()?;
    let source = fs::read_to_string(folder.join("conf/sonar.properties"))?;
    settings_from_properties(folder, &source)
}
pub fn managed_settings() -> Result<Settings> {
    if service_status()? == "Not installed" {
        load()
    } else {
        existing_settings()
    }
}
pub fn attach_existing() -> Result<Settings> {
    let settings = existing_settings()?;
    settings.persist_preferences()?;
    Ok(settings)
}
pub fn load_language() -> Result<bool> {
    let path = settings_path().with_file_name("language.json");
    if !path.exists() {
        return Ok(false);
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
pub fn save_language(mm: bool) -> Result<()> {
    let path = settings_path().with_file_name("language.json");
    fs::create_dir_all(path.parent().context("Missing preferences directory")?)?;
    fs::write(path, serde_json::to_vec(&mm)?)?;
    Ok(())
}
pub fn command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    cmd.creation_flags(0x08000000);
    cmd
}
fn checked(mut cmd: Command) -> Result<String> {
    let output = cmd.output()?;
    let text = output_text(&output);
    if !output.status.success() {
        bail!("{}", text);
    }
    Ok(text)
}
fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}
impl Settings {
    fn persist_preferences(&self) -> Result<()> {
        let target = settings_path();
        fs::create_dir_all(target.parent().context("Missing settings directory")?)?;
        fs::write(target, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            bail!("Port must be between 1 and 65535");
        }
        self.host
            .parse::<std::net::IpAddr>()
            .context("Host must be an IP address, e.g. 127.0.0.1 or 0.0.0.0")?;
        if !self.folder.is_absolute() {
            bail!("Choose an absolute SonarQube folder path");
        }
        for file in [
            "conf/sonar.properties",
            "bin/windows-x86-64/lib/SonarServiceWrapper.exe",
            "bin/windows-x86-64/lib/SonarServiceWrapperTemplate.xml",
        ] {
            if !self.folder.join(file).is_file() {
                bail!("Missing {file} in selected folder");
            }
        }
        Ok(())
    }
    pub fn save(&self) -> Result<()> {
        self.validate()?;
        let status = service_status()?;
        if status != "Stopped" && status != "Not installed" {
            bail!("Stop the service before changing settings ({status})");
        }
        if status != "Not installed"
            && !installed_folder()?
                .to_string_lossy()
                .eq_ignore_ascii_case(&self.folder.to_string_lossy())
        {
            bail!("Remove the existing service before changing its folder");
        }
        let path = self.folder.join("conf/sonar.properties");
        let original = fs::read_to_string(&path)?;
        let updated = update_properties(&original, &self.host, self.port);
        let backup = path.with_extension("properties.manager-backup");
        if !backup.exists() {
            fs::write(backup, &original)?;
        }
        fs::write(path, updated)?;
        self.persist_preferences()
    }
    pub fn install(&self) -> Result<String> {
        if service_status()? != "Not installed" {
            bail!(
                "SonarQube service already exists. Use existing service, then Start / Stop / Restart."
            );
        }
        self.save()?;
        // Resolve Java to an absolute path so the service does not depend on a user's PATH.
        let mut where_java = command("where.exe");
        where_java.arg("java.exe");
        let java = checked(where_java)?;
        let java = java.lines().next().context("Java is not installed")?;
        let java = fs::canonicalize(java)?;
        let java = java
            .to_string_lossy()
            .trim_start_matches(r"\\?\")
            .to_string();
        let lib = self.folder.join("bin/windows-x86-64/lib");
        let xml = fs::read_to_string(lib.join("SonarServiceWrapperTemplate.xml"))?;
        let escaped = java
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        fs::write(
            lib.join("SonarServiceWrapper.xml"),
            xml.replace("JAVA_EXE", &escaped),
        )?;
        let mut install = command(lib.join("SonarServiceWrapper.exe"));
        install.current_dir(&lib).arg("install");
        checked(install)?;
        let mut auto = command("sc.exe");
        auto.args(["config", SERVICE, "start=", "delayed-auto"]);
        checked(auto)?;
        control("start")
    }
    pub fn url(&self) -> String {
        let host = match self.host.as_str() {
            "0.0.0.0" | "::" => "127.0.0.1",
            host => host,
        };
        let host = if host.contains(':') {
            format!("[{host}]")
        } else {
            host.into()
        };
        format!("http://{host}:{}", self.port)
    }
}
pub fn update_properties(source: &str, host: &str, port: u16) -> String {
    let mut lines: Vec<&str> = source
        .lines()
        .filter(|line| {
            if line.trim() == "# Managed by SonarQube Manager" {
                return false;
            }
            let key = line.trim().split('=').next().unwrap_or("").trim();
            key != "sonar.web.host" && key != "sonar.web.port"
        })
        .collect();
    lines.push("# Managed by SonarQube Manager");
    format!(
        "{}\r\nsonar.web.host={}\r\nsonar.web.port={}\r\n",
        lines.join("\r\n"),
        host,
        port
    )
}
pub fn service_status() -> Result<String> {
    let output = command("sc.exe").args(["query", SERVICE]).output()?;
    let text = output_text(&output);
    if text.contains("1060") {
        return Ok("Not installed".into());
    }
    if !output.status.success() {
        bail!("{text}");
    }
    for (needle, label) in [
        ("RUNNING", "Running"),
        ("STOPPED", "Stopped"),
        ("START_PENDING", "Starting"),
        ("STOP_PENDING", "Stopping"),
    ] {
        if text.contains(needle) {
            return Ok(label.into());
        }
    }
    Ok(text)
}
pub fn control(action: &str) -> Result<String> {
    if !["start", "stop", "delete"].contains(&action) {
        bail!("Unsupported service command");
    }
    // Identify the registered service independently of stale manager preferences.
    installed_folder()?;
    let state = service_status()?;
    if action == "stop" && state == "Stopped" {
        return Ok("Service is already stopped.".into());
    }
    if action == "start" && state == "Running" {
        return Ok(
            "Windows service is already running. Check web status for SonarQube readiness.".into(),
        );
    }
    if action == "delete" && state != "Stopped" {
        bail!("Stop the service before removing it");
    }
    let mut cmd = command("sc.exe");
    cmd.args([action, SERVICE]);
    let output = cmd.output()?;
    let text = output_text(&output);
    if !output.status.success() {
        if action == "stop" && text.contains("1062") {
            return Ok("Service is already stopped.".into());
        }
        if action == "start" && text.contains("1056") {
            return Ok(
                "Windows service is already running. Check web status for SonarQube readiness."
                    .into(),
            );
        }
        bail!("{text}");
    }
    Ok(match action {
        "start" => "Service start requested. SonarQube web server may still be initializing.",
        "stop" => "Service stop requested.",
        _ => "Service removed.",
    }
    .into())
}
pub fn restart() -> Result<String> {
    if service_status()? == "Stopped" {
        return control("start");
    }
    if service_status()? != "Stopping" {
        control("stop")?;
    }
    for _ in 0..60 {
        if service_status()? == "Stopped" {
            return control("start");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    bail!("Service is still stopping. Wait and press Start when stopped.")
}
pub fn read_log(folder: &Path) -> Result<String> {
    let mut sections = Vec::new();
    for name in ["sonar.log", "web.log", "es.log"] {
        let content = read_log_file(&folder.join("logs").join(name))
            .unwrap_or_else(|e| format!("Cannot read log: {e}"));
        sections.push(format!("=== {name} ===\n{content}"));
    }
    Ok(sections.join("\n\n"))
}
fn read_log_file(path: &Path) -> Result<String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut file = fs::File::open(path)?;
    let length = file.metadata()?.len();
    file.seek(SeekFrom::Start(length.saturating_sub(16000)))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into())
}
pub fn web_status(settings: &Settings) -> Result<String> {
    use std::io::{Read, Write};
    use std::net::{IpAddr, SocketAddr, TcpStream};
    use std::time::Duration;
    let host = match settings.host.as_str() {
        "0.0.0.0" => "127.0.0.1",
        "::" => "::1",
        host => host,
    };
    let address = SocketAddr::new(host.parse::<IpAddr>()?, settings.port);
    let timeout = Duration::from_secs(2);
    let mut stream = TcpStream::connect_timeout(&address, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    write!(
        stream,
        "GET /api/system/status HTTP/1.0\r\nHost: {address}\r\nConnection: close\r\n\r\n"
    )?;
    let mut response = String::new();
    stream.take(65536).read_to_string(&mut response)?;
    parse_web_status(&response)
}
pub fn parse_web_status(response: &str) -> Result<String> {
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .context("Invalid HTTP response")?;
    let code = headers
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .nth(1)
        .unwrap_or_default();
    if !matches!(code, "200" | "503") {
        bail!("SonarQube status endpoint is not ready");
    }
    let value: serde_json::Value = serde_json::from_str(body)?;
    Ok(match value["status"]
        .as_str()
        .context("Missing SonarQube status")?
    {
        "UP" => "Web: Ready",
        "STARTING" | "RESTARTING" => "Web: Starting",
        "DB_MIGRATION_NEEDED" | "DB_MIGRATION_RUNNING" => "Web: Database migration required",
        _ => "Web: Not ready",
    }
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn web_readiness_requires_sonarqube_up_not_just_an_http_listener() {
        assert_eq!(
            parse_web_status("HTTP/1.0 200 OK\r\n\r\n{\"status\":\"UP\"}").unwrap(),
            "Web: Ready"
        );
        assert_eq!(
            parse_web_status("HTTP/1.0 503 Unavailable\r\n\r\n{\"status\":\"STARTING\"}").unwrap(),
            "Web: Starting"
        );
        assert!(parse_web_status("HTTP/1.0 200 OK\r\n\r\n<html>unrelated server</html>").is_err());
        assert!(parse_web_status("HTTP/1.0 401 Unauthorized\r\n\r\n{}").is_err());
    }
    #[test]
    fn readiness_probe_queries_the_local_status_endpoint() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                .unwrap();
            let mut request = [0; 1024];
            let count = stream.read(&mut request).unwrap();
            assert!(
                String::from_utf8_lossy(&request[..count])
                    .starts_with("GET /api/system/status HTTP/1.0")
            );
            stream
                .write_all(b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\n{\"status\":\"UP\"}")
                .unwrap();
        });
        let settings = Settings {
            host: "0.0.0.0".into(),
            port,
            ..Settings::default()
        };
        assert_eq!(web_status(&settings).unwrap(), "Web: Ready");
        server.join().unwrap();
    }
    #[test]
    fn discovers_existing_service_folder_with_spaces_and_arguments() {
        let config = "BINARY_PATH_NAME : \"D:\\Sonar Tools\\server\\bin\\windows-x86-64\\lib\\SonarServiceWrapper.exe\" --service";
        assert_eq!(
            folder_from_service_config(config).unwrap(),
            PathBuf::from(r"D:\Sonar Tools\server")
        );
        assert_eq!(folder_from_service_config("BINARY_PATH_NAME : C:\\sonarqube\\bin\\windows-x86-64\\lib\\sonarservicewrapper.EXE").unwrap(), PathBuf::from(r"C:\sonarqube"));
    }
    #[test]
    fn rejects_other_service_executables_even_when_arguments_mention_sonar() {
        assert!(folder_from_service_config("BINARY_PATH_NAME : \"C:\\other.exe\" --path C:\\sonarqube\\bin\\windows-x86-64\\lib\\SonarServiceWrapper.exe").is_err());
        assert!(folder_from_service_config("BINARY_PATH_NAME : C:\\other.exe").is_err());
    }
    #[test]
    fn reads_existing_configuration_and_ignores_commented_values() {
        let source = "#sonar.web.port=1234\nsonar.web.host = 127.0.0.1\nsonar.web.port=9100\nsonar.jdbc.url=unchanged";
        let settings = settings_from_properties(PathBuf::from(r"D:\Sonar Tools"), source).unwrap();
        assert_eq!(settings.host, "127.0.0.1");
        assert_eq!(settings.port, 9100);
        assert!(settings_from_properties(PathBuf::new(), "sonar.web.port=invalid").is_err());
        assert_eq!(
            settings_from_properties(PathBuf::new(), "# defaults")
                .unwrap()
                .port,
            9000
        );
    }
    #[test]
    fn preserves_other_settings_and_removes_duplicate_keys() {
        let result = update_properties(
            "# sonar.web.port=9000\nsonar.jdbc.url=test\nsonar.web.port=1\n sonar.web.port =2\nsonar.web.host=old",
            "127.0.0.1",
            9100,
        );
        assert!(result.contains("sonar.jdbc.url=test"));
        assert!(!result.contains("port=1"));
        assert_eq!(result.matches("\r\nsonar.web.port=").count(), 1);
        assert!(result.ends_with("sonar.web.port=9100\r\n"));
    }
    #[test]
    fn formats_ipv6_and_wildcard_urls() {
        let mut s = Settings {
            host: "::1".into(),
            ..Settings::default()
        };
        assert_eq!(s.url(), "http://[::1]:9000");
        s.host = "0.0.0.0".into();
        assert_eq!(s.url(), "http://127.0.0.1:9000");
    }
    #[test]
    fn repeated_updates_are_stable() {
        let first = update_properties("sonar.jdbc.url=test\n", "127.0.0.1", 9000);
        assert_eq!(first, update_properties(&first, "127.0.0.1", 9000));
    }
    #[test]
    fn rejects_zero_port_and_invalid_host_before_file_access() {
        let mut settings = Settings {
            port: 0,
            ..Settings::default()
        };
        assert!(
            settings
                .validate()
                .unwrap_err()
                .to_string()
                .contains("Port")
        );
        settings.port = 9000;
        settings.host = "127.0.0.1\nsonar.jdbc.url=bad".into();
        assert!(settings.validate().is_err());
    }
}
