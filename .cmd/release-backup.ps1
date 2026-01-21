param (
    # Flaga jest teraz opcjonalna (Mandatory=$false) i domyślnie ustawiona na "."
    [Parameter(Mandatory=$false, HelpMessage="Podaj ścieżkę do folderu głównego (Root). Domyślnie: bieżący folder.")]
    [string]$ProjectRoot = "."
)

# 1. Bezpieczna konwersja ścieżki na absolutną
# Resolve-Path poradzi sobie z kropką "." zamieniając ją na pełną ścieżkę do Twojego projektu
$resolved = Resolve-Path $ProjectRoot -ErrorAction SilentlyContinue

if (-not $resolved) {
    Write-Host "❌ Błąd: Nie można odnaleźć folderu projektu: '$ProjectRoot'" -ForegroundColor Red
    return
}

$AbsoluteRoot = $resolved.Path
$TargetDir = Join-Path $AbsoluteRoot "target/release"
$ReleaseBaseDir = Join-Path $AbsoluteRoot "release"

Write-Host "`n🚀 Rozpoczynam backup plików binarnych..." -ForegroundColor Yellow

# 2. Utworzenie głównego folderu /release jeśli nie istnieje
if (-not (Test-Path $ReleaseBaseDir)) {
    New-Item -ItemType Directory -Path $ReleaseBaseDir -Force | Out-Null
    Write-Host "📂 Utworzono główny folder: $ReleaseBaseDir" -ForegroundColor Gray
}

# 3. Generowanie timestampu w formacie v.YYYY.DDD.HH.MM.SS.SSS
$t = Get-Date
$ts = "v.{0}.{1:000}.{2}" -f $t.Year, $t.DayOfYear, $t.ToString("HH.mm.ss.fff")

# 4. Definiowanie pełnej ścieżki docelowej
$DestPath = Join-Path $ReleaseBaseDir $ts

# 5. Sprawdzenie czy folder /target/release istnieje i zawiera pliki .exe
if (Test-Path $TargetDir) {
    $ExeFiles = Get-ChildItem -Path $TargetDir -Filter "*.exe"

    if ($ExeFiles.Count -gt 0) {
        # Tworzenie podfolderu z timestampem
        New-Item -ItemType Directory -Path $DestPath -Force | Out-Null
        
        # Kopiowanie plików
        Copy-Item -Path "$TargetDir/*.exe" -Destination $DestPath
        
        Write-Host "✅ Backup gotowy!" -ForegroundColor Green
        Write-Host "📂 Lokalizacja: $DestPath" -ForegroundColor Cyan
        Write-Host "📄 Skopiowane pliki: $($ExeFiles.Name -join ', ')" -ForegroundColor White
    }
    else {
        Write-Host "⚠️ W folderze $TargetDir nie znaleziono żadnych plików .exe!" -ForegroundColor Yellow
    }
}
else {
    Write-Host "❌ Błąd: Ścieżka $TargetDir nie istnieje. Czy na pewno zbudowałeś projekt w wersji --release?" -ForegroundColor Red
}

Write-Host ""