<#
SWBPIPE sitting pack: run one MBF of the pack through the documented batch pattern with the guest script.

Runs Invoke-CaepipeRoundTrip.ps1 (beside this script) on the MBF given, with no GUI fallback unless
-AllowGuiFallback is given, and saves the script's report, which carries the SHA-256 of the MBF it solved, as a new
file <stem>.guest.json beside the MBF. The results the program writes appear beside the MBF as well.

It refuses to run, and changes nothing, unless:
- the MBF lies in a case folder of a copy of the pack (two folders up holds pack_index.json), and no folder from
  the case folder up holds harness_config.json: the pack is never run inside the exchange folder;
- none of the files this run would write exists yet (<stem>.csv, .txt, _results.csv, .res, .mod, .guest.json):
  the guest script removes earlier prints before it runs and the program replaces its own outputs, so a second run
  needs a fresh copy of the pack.
The report is created as a new file; an existing one is never replaced.

    powershell -NoProfile -ExecutionPolicy Bypass -File Run-SittingCase.ps1 -Exe "<program>" -Mbf "<case folder>\<stem>.mbf"

This driver has not been executed by SWBPIPE's tests: smoke-test it on Windows with a substituted guest script
before the guest-script route is used.
#>
param(
    [Parameter(Mandatory = $true)][string]$Exe,
    [Parameter(Mandatory = $true)][string]$Mbf,
    [switch]$AllowGuiFallback
)
$ErrorActionPreference = "Stop"
function Refuse([string]$Why) {
    [Console]::Error.WriteLine("Run-SittingCase: refused: " + $Why)
    exit 64
}

$mbfItem = Get-Item -LiteralPath $Mbf
$stem = [IO.Path]::GetFileNameWithoutExtension($mbfItem.Name)
$caseDir = $mbfItem.Directory

# 1. A copy of the pack, outside the exchange folder.
for ($dir = $caseDir; $null -ne $dir; $dir = $dir.Parent) {
    if (Test-Path -LiteralPath (Join-Path $dir.FullName "harness_config.json")) {
        Refuse ("the case folder lies inside the exchange folder (" + $dir.FullName + " holds harness_config.json); run from a fresh copy of the pack")
    }
}
$packRoot = $caseDir.Parent.Parent
if ($null -eq $packRoot -or -not (Test-Path -LiteralPath (Join-Path $packRoot.FullName "pack_index.json"))) {
    Refuse "the MBF is not in a case folder of a copy of the pack (no pack_index.json two folders up)"
}

# 2. Nothing this run would write may exist yet.
$report = Join-Path $caseDir.FullName ($stem + ".guest.json")
foreach ($suffix in @(".csv", ".txt", "_results.csv", ".res", ".mod", ".guest.json")) {
    $path = Join-Path $caseDir.FullName ($stem + $suffix)
    if (Test-Path -LiteralPath $path) {
        Refuse ((Split-Path $path -Leaf) + " already exists; this run would remove or replace it. Use a fresh copy of the pack for another run.")
    }
}

# 3. Run, then create the report as a new file.
$script = Join-Path $PSScriptRoot "Invoke-CaepipeRoundTrip.ps1"
$arguments = @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $script, "-Exe", $Exe, "-Mbf", $mbfItem.FullName)
if (-not $AllowGuiFallback) { $arguments += "-NoGuiFallback" }
$output = & powershell @arguments 2>&1 | ForEach-Object { "$_" }
$code = $LASTEXITCODE
$bytes = (New-Object Text.UTF8Encoding($false)).GetBytes((($output -join "`r`n") + "`r`n"))
$stream = [IO.File]::Open($report, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
try { $stream.Write($bytes, 0, $bytes.Length) } finally { $stream.Close() }
Write-Host ("{0}: guest script exit {1}; report saved as {2}" -f $stem, $code, (Split-Path $report -Leaf))
exit $code
