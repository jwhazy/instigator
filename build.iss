; Thank you to these StackOverflow posts
; https://stackoverflow.com/questions/3304463/how-do-i-modify-the-path-environment-variable-when-running-an-inno-setup-install
; https://stackoverflow.com/questions/24574035/how-to-install-microsoft-vc-redistributables-silently-in-inno-setup

[Setup]
AppName=Instigator
AppVersion=3.0.0
AppPublisher=jacksta
WizardStyle=modern
DefaultDirName={userpf}\instigator
DefaultGroupName=Instigator
UninstallDisplayIcon={app}\instigator.exe
OutputBaseFilename=Instigator_install
AppVerName=Instigator
Compression=lzma2
SolidCompression=yes
OutputDir=.\build
SetupIconFile=.\assets\installer.ico
ChangesEnvironment=true
DisableProgramGroupPage=yes
AllowNoIcons=yes
PrivilegesRequired=none

[Files]
Source: "target\release\instigator.exe"; DestDir: "{app}"
Source: "VC_redist.x64.exe"; DestDir: {tmp}; Flags: dontcopy

[Run]
Filename: "{tmp}\VC_redist.x64.exe"; StatusMsg: "Installing VC redistributables."; \
  Parameters: "/quiet"; Check: InstallRedist ; Flags: waituntilterminated

[Code]
const EnvironmentKey = 'Environment';

procedure EnvAddPath(Path: string);
var
    Paths: string;
begin
    { Retrieve current path (use empty string if entry not exists) }
    if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths)
    then Paths := '';

    { Skip if string already found in path }
    if Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';') > 0 then exit;

    { App string to the end of the path variable }
    Paths := Paths + ';'+ Path +';'

    { Overwrite (or create if missing) path environment variable }
    if RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths)
    then Log(Format('The [%s] added to PATH: [%s]', [Path, Paths]))
    else Log(Format('Error while adding the [%s] to PATH: [%s]', [Path, Paths]));
end;

procedure EnvRemovePath(Path: string);
var
    Paths: string;
    P: Integer;
begin
    { Skip if registry entry not exists }
    if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths) then
        exit;

    { Skip if string not found in path }
    P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
    if P = 0 then exit;

    { Update path variable }
    Delete(Paths, P - 1, Length(Path) + 1);

    { Overwrite path environment variable }
    if RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths)
    then Log(Format('The [%s] removed from PATH: [%s]', [Path, Paths]))
    else Log(Format('Error while removing the [%s] from PATH: [%s]', [Path, Paths]));
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
    if CurStep = ssPostInstall 
     then EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
    if CurUninstallStep = usPostUninstall
    then EnvRemovePath(ExpandConstant('{app}'));
end;

function InstallRedist: Boolean;
var 
  Version: String;
begin
  if RegQueryStringValue(HKEY_LOCAL_MACHINE,
       'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64', 'Version', Version) then
  begin
    // Is the installed version at least 14.14 ? 
    Log('VC Redist Version check : found ' + Version);
    Result := (CompareStr(Version, 'v14.14.26429.03')<0);
  end
  else 
  begin
    // Not even an old version installed
    Result := True;
  end;
  if (Result) then
  begin
    ExtractTemporaryFile('VC_redist.x64.exe');
  end;
end;
