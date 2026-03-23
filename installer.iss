[Setup]
AppId={{8F3D9A7B-6C4E-4F2B-A1D8-5E7C9B3F4A2D}
AppName=SpeakEasy
AppVersion=0.1.0
AppPublisher=赵世俊
AppPublisherURL=https://github.com/qq296565302/tts-rust
AppSupportURL=https://github.com/qq296565302/tts-rust
AppCopyright=Copyright (C) #expr GetDateTimeString(Now, 'yyyy', '') 赵世俊
AppContact=赵世俊
AppComments=SpeakEasy - TTS语音生成工具
DefaultDirName={autopf}\SpeakEasy
DefaultGroupName=SpeakEasy
OutputDir=installer
OutputBaseFilename=SpeakEasy-Setup
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
SetupIconFile=D:\KaKaRoot\TTS语音生成工具\icon.ico
UninstallDisplayIcon={app}\speakeasy.exe
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64
VersionInfoVersion=0.1.0
VersionInfoCompany=赵世俊
VersionInfoDescription=SpeakEasy - TTS语音生成工具
VersionInfoCopyright=Copyright (C) #expr GetDateTimeString(Now, 'yyyy', '') 赵世俊
VersionInfoProductName=SpeakEasy
VersionInfoProductVersion=0.1.0

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: checkedonce

[Files]
Source: "D:\KaKaRoot\TTS语音生成工具\target\release\speakeasy.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "D:\KaKaRoot\TTS语音生成工具\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\SpeakEasy"; Filename: "{app}\speakeasy.exe"
Name: "{group}\{cm:UninstallProgram,SpeakEasy}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\SpeakEasy"; Filename: "{app}\speakeasy.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\speakeasy.exe"; Description: "{cm:LaunchProgram,SpeakEasy}"; Flags: nowait postinstall skipifsilent
