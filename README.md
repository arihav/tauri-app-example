
## About
The app is closing in a normal run after 10 seconds from launch, for running in dev mode change the main file.
App is running on port 1420

## Build
Build the project
```
cargo tauri build 
```

## Install Windows Service

Open Command Prompt (CMD) as Administrator and run:

```
sc create "TauriAppService" binPath= "C:\path\to\project\src-tauri\target\release\tauri-app.exe"
```

Start the service
```
net start "TauriAppService"
```

Stop the service
```
net stop  "TauriAppService"
```

Delete the service
```
sc delete "TauriAppService"
```

