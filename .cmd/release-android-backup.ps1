param (
    [Parameter(Mandatory=$false, HelpMessage="Podaj ścieżkę do folderu głównego (Root). Domyślnie: bieżący folder.")]
    [string]$ProjectRoot = "."
)

$resolved = Resolve-Path $ProjectRoot -ErrorAction SilentlyContinue
if (-not $resolved) {
    Write-Host "❌ Błąd: Nie można odnaleźć folderu projektu: '$ProjectRoot'" -ForegroundColor Red
    return
}

$AbsoluteRoot = $resolved.Path
$SearchBaseDir = Join-Path $AbsoluteRoot "src-tauri/gen/android/app/build/outputs"
$ReleaseBaseDir = Join-Path $AbsoluteRoot "release"

Write-Host "`n🚀 Skanowanie w poszukiwaniu plików .apk i .aab w $SearchBaseDir..." -ForegroundColor Yellow

if (-not (Test-Path $SearchBaseDir)) {
    Write-Host "❌ Błąd: Folder budowania nie istnieje!" -ForegroundColor Red
    return
}

# Szukamy rekurencyjnie i filtrujemy tylko wersje release
$FilesToBackup = Get-ChildItem -Path $SearchBaseDir -Recurse -Include *.apk, *.aab | Where-Object { $_.FullName -like "*release*" }

if ($FilesToBackup.Count -gt 0) {
    if (-not (Test-Path $ReleaseBaseDir)) {
        New-Item -ItemType Directory -Path $ReleaseBaseDir -Force | Out-Null
    }

    $t = Get-Date
    $ts = "v.{0}.{1:000}.{2}" -f $t.Year, $t.DayOfYear, $t.ToString("HH.mm.ss.fff")
    $DestPath = Join-Path $ReleaseBaseDir $ts
    New-Item -ItemType Directory -Path $DestPath -Force | Out-Null

    foreach ($file in $FilesToBackup) {
        Copy-Item -Path $file.FullName -Destination $DestPath
        Write-Host "✅ Skopiowano: $($file.Name)" -ForegroundColor Green
    }

    Write-Host "`n📂 Backup gotowy w lokalizacji: $DestPath" -ForegroundColor Cyan
}
else {
    Write-Host "⚠️ Nie znaleziono żadnych plików .apk lub .aab z flagą release!" -ForegroundColor Yellow
}