param (
    # Flaga opcjonalna, domyślnie "."
    [Parameter(Mandatory=$false, HelpMessage="Podaj ścieżkę do folderu głównego (Root). Domyślnie: bieżący folder.")]
    [string]$ProjectRoot = ".",
    [int]$MaxDepth = 5
)

# Konfiguracja filtrów i priorytetów
$ExcludeList = @("target", ".git", "node_modules", "gen", "helper", "temp")
$PriorityNames = @("Cargo.lock", "deno.lock", "Cargo.toml", "deno.jsonc", "tauri.conf.json")

# 1. Bezpieczna rezolucja ścieżki
$resolved = Resolve-Path $ProjectRoot -ErrorAction SilentlyContinue
if (-not $resolved) {
    Write-Host "❌ Błąd: Nie można odnaleźć folderu: '$ProjectRoot'" -ForegroundColor Red
    return
}
$AbsoluteRoot = $resolved.Path

function Write-FileTree {
    param (
        [string]$CurrentPath,
        [string]$Indent = "",
        [int]$CurrentDepth = 0
    )

    if ($CurrentDepth -gt $MaxDepth) { return }

    # --- SORTOWANIE (Priorytety 0-4, Pliki 10, Foldery 20) ---
    $Items = Get-ChildItem -Path $CurrentPath | 
             Where-Object { $ExcludeList -notcontains $_.Name } |
             Sort-Object @{Expression={
                 if ($_.PSIsContainer) { 20 }
                 else {
                     $idx = [array]::IndexOf($PriorityNames, $_.Name)
                     ($idx -ne -1) ? $idx : 10
                 }
             }}, Name

    $Count = $Items.Count
    $LastWasPriority = $false

    for ($i = 0; $i -lt $Count; $i++) {
        $Item = $Items[$i]
        $IsLast = ($i -eq ($Count - 1))
        $IsPriority = $PriorityNames.Contains($Item.Name)

        # --- LOGIKA SEPARATORÓW ---
        if ($i -eq 0 -and $IsPriority) { Write-Host ($Indent + "│") -ForegroundColor Gray }
        if ($LastWasPriority -and -not $IsPriority) { 
            Write-Host ($Indent + "│") -ForegroundColor Gray
            $LastWasPriority = $false 
        }

        # Rysowanie gałęzi
        $Branch = $IsLast ? "└── " : "├── "
        Write-Host ($Indent + $Branch) -NoNewline -ForegroundColor Gray
        
        if ($Item.PSIsContainer) {
            # --- FOLDER ---
            Write-Host "📂 " -NoNewline -ForegroundColor Yellow
            Write-Host ($Item.Name + "/") -ForegroundColor Cyan
            $NextIndent = $Indent + ($IsLast ? "    " : "│   ")
            Write-FileTree -CurrentPath $Item.FullName -Indent $NextIndent -CurrentDepth ($CurrentDepth + 1)
        } else {
            # --- PLIK (IKONY I KOLORY) ---
            $Icon = "📄 "; $Color = "White"

            if ($Item.FullName -match "[\\/](capabilities|permissions)[\\/]") {
                $Icon = "⚙️   "; $Color = "Red"
            } else {
                switch -Regex ($Item.Name) {
                    "^(Cargo\.lock|deno\.lock)$" { $Icon = "🔒  "; $Color = "Yellow" }
                    "^(Cargo\.toml|deno\.jsonc?|config\.toml|settings\.json|tauri\.conf\.json|\.gitignore)$" { $Icon = "⚙️   "; $Color = "Red" }
                    "\.ps1$" { $Icon = "▶️  "; $Color = "Green" }
                    "\.(png|svg|jpg|jpeg|ico|icns)$" { $Icon = "🖼️  "; $Color = "Gray" }
                    "\.md$" { $Icon = "📖 "; $Color = "White" }
                    "\.(rs|ts)$" { $Icon = "📜  "; $Color = "Magenta" }
                    "\.exe$" { $Icon = "📦 "; $Color = "Green" }
                }
            }
            Write-Host $Icon -NoNewline
            Write-Host $Item.Name -ForegroundColor $Color
            if ($IsPriority) { $LastWasPriority = $true }
        }
    }
}

# Start wyświetlania
Write-Host "`nStruktura projektu: $AbsoluteRoot" -ForegroundColor Yellow
Write-Host "------------------------------------------------" -ForegroundColor Yellow
Write-FileTree -CurrentPath $AbsoluteRoot
Write-Host "------------------------------------------------`n" -ForegroundColor Yellow