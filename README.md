
## About
The app is closing in a normal run after 10 seconds from launch, for running in dev mode change the main file.

## Build
Build the project
```
cargo tauri build 
```

## Install Windows Service

Open PowerShell as Administrator and run:

```
New-Service -Name "TauriAppService " -BinaryPathName "C:\path\to\project\src-tauri\target\release\tauri-app.exe"
Start-Service "YourServiceName"
```

