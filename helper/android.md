# Komendy Android i Mobile

## Przygotowanie Środowiska (Jednorazowo)
- `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`
  Dodaje wsparcie dla procesorów mobilnych do kompilatora Rust.

## Zadania Tauri Mobile (przez Deno)
- `deno task android-init`
  Inicjalizuje projekt mobilny. Generuje niezbędne pliki natywne Androida (uruchom tylko raz).
- `deno task android-dev`
  Kompiluje kod Rust i TS, a następnie przesyła aplikację na podłączony telefon lub emulator (tryb deweloperski).
- `deno task android-build`
  Generuje gotowy plik instalacyjny `.apk` lub pakiet `.aab` do Sklepu Play.

## Wymagania
- Zainstalowane Android Studio.
- Zmienna środowiskowa `ANDROID_HOME` ustawiona na ścieżkę SDK.
- Włączone "Debugowanie USB" w telefonie.

# Komendy Android: Instalacja i Konfiguracja

## 1. Instalacja Narzędzi (Windows - PowerShell)
Jeśli nie chcesz klikać w menu Android Studio, możesz użyć menedżera pakietów Windows (Winget):

- `winget install -e --id Google.AndroidStudio`
  Instaluje Android Studio (zawiera SDK Manager).
- `winget install -e --id Oracle.JDK.17`
  Instaluje wymaganą wersję Javy (JDK 17).

## 2. Przygotowanie Rusta pod Androida
Musisz zainstalować "targety" dla różnych architektur procesorów mobilnych:

- `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`

## 3. Automatyczna Diagnostyka Tauri
Tauri posiada wbudowane narzędzie, które powie Ci dokładnie, czego brakuje w Twoim systemie:

- `deno task tauri doctor`
  Sprawdza zainstalowane wersje SDK, NDK i zmienne środowiskowe.

## 4. Konfiguracja Zmiennych Środowiskowych
Jeśli system nie widzi komend, dodaj te ścieżki do swojego profilu PowerShell (plik `$PROFILE`):

```powershell
$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:PATH += ";$env:ANDROID_HOME\platform-tools"
$env:PATH += ";$env:ANDROID_HOME\cmdline-tools\latest\bin"
```