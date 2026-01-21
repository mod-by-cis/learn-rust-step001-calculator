# `https://github.com/mod-by-cis/learn-rust-step001-calculator`

## komendy


- `cargo run --bin`
- `cargo check`
- `cargo fmt`
- `cargo clippy`
- `cargo tree`
- `cargo update`
- `deno task build-distGUI`
- `cargo run --bin lekcja003-kalkulator-cli`
- `cargo run --bin lekcja003-kalkulator-gui`
- `cargo build --release`
- `./.cmd/release-backup.ps1`
- `./.cmd/files-tree.ps1`
- `cargo tauri android init`
- `cargo tauri android dev`
- `cargo tauri android build --apk true --aab true --split-per-abi true`
- `cargo tauri android build --apk true --aab true`
- `Get-ChildItem -Path "src-tauri/gen/android/app/build/outputs" -Recurse -Include *.apk, *.aab | Select-Object FullName`
- `code .`
- `cargo clean`
- `$env:ANDROID_HOME`
- `$env:NDK_HOME`
- `$env:ANDROID_SDK_ROOT`
- `adb --version`
- `cargo tauri info`

---

`./deno.jsonc`

```jsonc
{
  "tasks": {   
    // --- APKA-KONSOLOWA ---
    "start-console": "cargo run -p lekcja003-kalkulator-cli --bin lekcja003-kalkulator-cli",
    "check-console": "cargo check -p lekcja003-kalkulator-cli",
    "build-console": "cargo build -p lekcja003-kalkulator-cli --release",
    "format-console": "cargo fmt -p lekcja003-kalkulator-cli",
    "analiza-console": "cargo clippy -p lekcja003-kalkulator-cli",
    "zaleznosci-console": "cargo tree -p lekcja003-kalkulator-cli",
    "aktualizacja-console": "cargo update -p lekcja003-kalkulator-cli",

    // --- INTERFEJS-GRAFICZNY ---
    "build-distGUI": "deno task -c src-deno/deno.jsonc build-gui",

    // --- APKA-GRAFICZNA (TAURI CLI) ---
    "tauri": "deno run -A npm:@tauri-apps/cli@2.9.1 ",
    "tauri-android-init" :"deno task tauri android init",
    "check-tauri": "cargo check -p lekcja003-kalkulator-gui",
    "format-tauri": "cargo fmt -p lekcja003-kalkulator-gui",
    "analiza-tauri": "cargo clippy -p lekcja003-kalkulator-gui",
    "zaleznosci-tauri": "cargo tree -p lekcja003-kalkulator-gui",
    "aktualizacja-tauri": "cargo update -p lekcja003-kalkulator-gui",

    // --- APKA-GRAFICZNA na WINDOWS ---
    "start-tauri-win": "deno task tauri dev",
    "build-tauri-win": "deno task tauri build",

    // --- APKA-GRAFICZNA na ANDROID ---
    "start-tauri-and": "deno task tauri android dev",
    "build-tauri-and": "deno task tauri android build",

    // --- HELPS ---
    "list-bin":  "cargo run --bin",
    "clean-all": "cargo clean"

  }
}
```
