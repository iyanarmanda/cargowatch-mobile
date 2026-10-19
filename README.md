# Tauri + SvelteKit + TypeScript

## Tech Stack

- Tauri
- SvelteKit
- Vite
- Rust
- TypeScript

## Usage

### Android USB Debugging Initialization

Install Rust cross platform target compilation:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

Android SDK Cmdline Tools Installation (*if needed*):

```sh
/home/<user>/Android/Sdk/cmdline-tools/latest/bin/sdkmanager --install "ndk;26.1.10909125" "cmdline-tools;latest" "build-tools;34.0.0" "platforms;android-34"
```

Add environment variables in `~/bashrc`:

```sh
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
```

Android initialization:

```sh
pnpm tauri android init
```

Add ADB reverse port forwarding (*if needed*):

```sh
# tauri
$ adb reverse tcp:1420 tcp:1420
$ adb reverse tcp:1421 tcp:1421

# API (if needed)
$ adb reverse tcp:3000 tcp:3000
```

Disabled firewall/ufw for selected tcp (*if needed*):

```sh
sudo ufw allow 1420/tcp
sudo ufw allow 1421/tcp
sudo ufw allow 3000/tcp
```

For enable firewall/ufw and close reverese port forwarding after finished:

```sh
# ADB reverse port forwarding
$ adb reverse --remove-all

# Firewall/UFW
$ sudo ufw delete allow 1420/tcp
$ sudo ufw delete allow 1421/tcp
$ sudo ufw delete allow 3000/tcp
```

### FCM Initialization

Get FCM Token from [https://console.firebase.google.com/](https://console.firebase.google.com/). Create a project for android and register the domain app. Then you will get `google-services.json`.

Encode `google-services.json` to Base64:

```sh
base64 -w 0 <google-services.json>
```

Copy the string and set GitHub environment variables with name `GOOGLE_SERVICES_JSON_BASE64`.

Then, CI/CD will do the rest.

### Development

```sh
pnpm tauri android dev
```

Inspect browser (open in chromium browser):

```
chrome://inspect/#devices
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## License

This project is MIT Licensed
