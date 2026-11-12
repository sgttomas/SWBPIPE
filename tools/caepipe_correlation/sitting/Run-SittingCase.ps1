<#
SWBPIPE sitting pack: run one MBF of the pack through the documented batch pattern with the guest script.

Runs Invoke-CaepipeRoundTrip.ps1 (beside this script) on the MBF given, with no GUI fallback unless
-AllowGuiFallback is given, and saves the script's report, which carries the SHA-256 of the MBF it solved, as
<stem>.guest.json beside the MBF. The results the program writes appear beside the MBF as well. Nothing else is
written, and the MBF is not changed.

    powershell -NoProfile -ExecutionPolicy Bypass -File Run-SittingCase.ps1 -Exe "<program>" -Mbf "<case folder>\<stem>.mbf"
#>
param(
    [Parameter(Mandatory = $true)][string]$Exe,
    [Parameter(Mandatory = $true)][string]$Mbf,
    [switch]$AllowGuiFallback
)
$ErrorActionPreference = "Stop"
$mbfItem = Get-Item -LiteralPath $Mbf
$stem = [IO.Path]::GetFileNameWithoutExtension($mbfItem.Name)
$report = Join-Path $mbfItem.DirectoryName ($stem + ".guest.json")
$script = Join-Path $PSScriptRoot "Invoke-CaepipeRoundTrip.ps1"
$arguments = @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $script, "-Exe", $Exe, "-Mbf", $mbfItem.FullName)
if (-not $AllowGuiFallback) { $arguments += "-NoGuiFallback" }
$output = & powershell @arguments 2>&1 | ForEach-Object { "$_" }
$code = $LASTEXITCODE
[IO.File]::WriteAllText($report, (($output -join "`r`n") + "`r`n"), (New-Object Text.UTF8Encoding($false)))
Write-Host ("{0}: guest script exit {1}; report saved as {2}" -f $stem, $code, (Split-Path $report -Leaf))
exit $code
