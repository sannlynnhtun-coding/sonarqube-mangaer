[Setup]
AppId={{74A39ED6-2783-47F4-A601-DBBECD91C52D}
AppName=SonarQube Manager
AppVersion=0.1.0
DefaultDirName={autopf}\SonarQube Manager
DefaultGroupName=SonarQube Manager
OutputDir=..\dist
OutputBaseFilename=SonarQubeManager-Setup
SetupIconFile=..\assets\sonarqube.ico
UninstallDisplayIcon={app}\sonarqube-manager.exe
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
Compression=lzma2
SolidCompression=yes

[Files]
Source: "..\target\release\sonarqube-manager.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\SonarQube Manager"; Filename: "{app}\sonarqube-manager.exe"
Name: "{autodesktop}\SonarQube Manager"; Filename: "{app}\sonarqube-manager.exe"; Tasks: desktopicon

[Tasks]
Name: desktopicon; Description: "Create a desktop shortcut"

[Run]
Filename: "{app}\sonarqube-manager.exe"; Description: "Open SonarQube Manager"; Flags: postinstall nowait skipifsilent

[UninstallRun]
Filename: "{sys}\sc.exe"; Parameters: "stop SonarQube"; Flags: runhidden; RunOnceId: StopSonarQube; Check: RemoveService
Filename: "{sys}\sc.exe"; Parameters: "delete SonarQube"; Flags: runhidden; RunOnceId: DeleteSonarQube; Check: RemoveService

[Code]
var RemoveServiceChoice: Boolean;
function InitializeUninstall(): Boolean;
begin
  RemoveServiceChoice := False;
  if not UninstallSilent then
    RemoveServiceChoice := MsgBox('Also stop and remove the SonarQube Windows service? SonarQube files and data will be kept.', mbConfirmation, MB_YESNO) = IDYES;
  Result := True;
end;
function RemoveService(): Boolean;
begin
  Result := RemoveServiceChoice;
end;
