# SonarQube Manager

Windows desktop app built in Rust with [GPUI Kit](https://github.com/longbridge/gpui-kit). Select an existing SonarQube distribution, configure host and port, and manage its bundled Windows service.

The manager opens maximized, with the standard Windows title bar and taskbar available. Clicking **×** (or Alt+F4) hides the window in the system tray. Double-click the SonarQube tray icon or use **Show manager** to reopen it maximized. Right-click the tray icon for **Start / Stop / Restart / Exit**. Exit closes only the manager; the SonarQube service keeps running. Service controls and Exit are disabled while a service operation is in progress. If the tray icon cannot be created, the app reports the error and closing the window exits normally.

## Use

If the Windows service named **SonarQube** already exists, the manager detects its registered SonarQube folder and reads its current host/port on startup. Click **Use existing service** to reload these values and remember the connection. This works while the service is running and does not edit `sonar.properties`, reinstall, stop, or restart it. Use **Start / Stop / Restart** directly; **Install service** is disabled while a service exists. Changing host/port still requires stopping the service first. Supported existing services use SonarQube's bundled `bin\windows-x86-64\lib\SonarServiceWrapper.exe`.

Click the **ⓘ information icon** beside the app title to open the step-by-step setup popup. Close it with the × button, Escape, or by clicking outside the popup.

Use **Light mode / Dark mode** beside the language button to switch appearance. The default is dark. Your choice is saved immediately in `%PROGRAMDATA%\SonarQubeManager\appearance.json` and restored when the app restarts.

Use **EN → MM / MM → EN** beside the information icon to switch the interface and setup instructions between English and Myanmar. The selection is saved immediately in `%PROGRAMDATA%\SonarQubeManager\language.json` and restored when the app restarts. Windows service diagnostics and SonarQube log contents retain their original language.

1. Install the required Java version for your SonarQube release. Java must be available on PATH when installing the service.
2. Extract SonarQube into **`C:\sonarqube`** (the default path). The selected folder must directly contain `conf`, `bin`, and `lib`. If extraction creates a versioned subfolder, choose that subfolder with **Browse** or move its contents into `C:\sonarqube`.
3. Install and launch SonarQube Manager. Approve the Windows administrator prompt.
4. Select the SonarQube folder. Keep **Host IP** `127.0.0.1` for this computer, or use `0.0.0.0` for LAN access. Set **Port** (default `9000`).
5. Click **Save settings**, then **Install service**. The app installs SonarQube's bundled wrapper, enables delayed automatic startup, and starts it.
6. Wait for status **Running**. Status refreshes automatically every three seconds; **Refresh status** also checks immediately.
7. Click **Open SonarQube**. A running Windows service can still be initializing; if the page is unavailable, inspect **View log** and allow startup to finish.
8. Close the window to keep the manager in the tray, or select **Exit** from the tray menu to quit. The service keeps running and starts automatically after Windows restarts.

Button labels and icons identify each action. Start is green, Stop is amber, Remove service is red, and the main setup/browser actions use the primary theme color.

Windows service status and SonarQube web readiness are shown separately. **Open SonarQube** becomes available when `/api/system/status` reports ready (or requires database migration). **View log** shows `sonar.log`, `web.log`, and `es.log` together to diagnose startup failures. A running wrapper alone does not prove the web server is ready. Stopping an already-stopped service is harmless, and Restart also starts a stopped service.

The new default applies when there are no saved settings. Existing saved installation paths are preserved.

Start, stop, restart, and remove operate on the Windows service named `SonarQube`. Stop before saving configuration; remove the service before switching installation folders. The app supports one managed installation. Existing services should be inspected before using these controls.

Settings live in `%PROGRAMDATA%\SonarQubeManager\settings.json`. The first configuration edit backs up `conf\sonar.properties` as `sonar.properties.manager-backup`. Database and other properties are preserved. The service uses the absolute Java executable resolved during installation. Reinstall the service after moving Java.

Use `127.0.0.1` for local access or `0.0.0.0` for LAN hosting. Configure firewall rules separately if needed. The app does not bundle Java or SonarQube. Uninstalling the manager can optionally remove the service and keeps SonarQube files/data.

## Build

Requires Rust stable MSVC, Visual Studio C++ Build Tools with Windows SDK, and Inno Setup 6 for the installer.

```powershell
.\scripts\build.ps1
```

Executable: `target\release\sonarqube-manager.exe`. Installer: `dist\SonarQubeManager-Setup.exe`.

## Verification

```powershell
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --lib --no-default-features
```

Core tests cover property preservation, repeated saves, input validation, and browser URL formatting. Service installation/startup and native GUI interactions require an elevated Windows session and are not covered by these tests. No SonarQube service is installed automatically during the build.

The icon is sourced from the locally supplied SonarQube distribution. This is an independent management utility; SonarQube branding belongs to SonarSource.
