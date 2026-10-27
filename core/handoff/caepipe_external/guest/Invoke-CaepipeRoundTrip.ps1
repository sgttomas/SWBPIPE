<#
DEL-17-05 guest-side round trip for a user-owned CAEPIPE installation.

Runs inside the user's Windows environment. It imports and solves one MBF in
batch mode (CAEPIPE-BATCH, CAEPIPE-IMPORT-MBF), then, when batch mode does not
leave a CSV beside the MBF, opens the solved results in a fresh
instance and writes them to a CSV file through the documented print-to-file
command. It locates menu items and buttons by their visible captions. The script only stops processes it started.

Output: one JSON object on stdout describing what happened, including the
SHA-256 of the MBF actually solved (mbf_sha256). CAEPIPE outputs
stay in the work directory; the caller copies them to its private run folder.
#>
param(
    [Parameter(Mandatory = $true)][string]$Exe,
    [Parameter(Mandatory = $true)][string]$Mbf,
    [string]$CsvName = "",
    [int]$BatchTimeoutSec = 120,
    [int]$GuiTimeoutSec = 120
)
$ErrorActionPreference = "Stop"

Add-Type @"
using System; using System.Text; using System.Runtime.InteropServices; using System.Collections.Generic;
public class CaepipeUi {
  public delegate bool EnumProc(IntPtr h, IntPtr l);
  [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc p, IntPtr l);
  [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr w, EnumProc p, IntPtr l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern int GetClassName(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] static extern IntPtr GetMenu(IntPtr h);
  [DllImport("user32.dll")] static extern int GetMenuItemCount(IntPtr m);
  [DllImport("user32.dll")] static extern uint GetMenuItemID(IntPtr m, int i);
  [DllImport("user32.dll")] static extern IntPtr GetSubMenu(IntPtr m, int i);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern int GetMenuString(IntPtr m, uint i, StringBuilder s, int n, uint f);
  [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint msg, IntPtr w, IntPtr l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern IntPtr SendMessage(IntPtr h, uint msg, IntPtr w, string l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern IntPtr SendMessage(IntPtr h, uint msg, IntPtr w, StringBuilder l);
  [DllImport("user32.dll")] public static extern IntPtr SendMessage(IntPtr h, uint msg, IntPtr w, IntPtr l);
  [DllImport("user32.dll")] public static extern int GetDlgCtrlID(IntPtr h);
  public static string Text(IntPtr h) { var sb = new StringBuilder(2048); GetWindowText(h, sb, 2048); return sb.ToString(); }
  public static string Cls(IntPtr h) { var sb = new StringBuilder(256); GetClassName(h, sb, 256); return sb.ToString(); }
  public static List<IntPtr> Windows(uint pid) {
    var r = new List<IntPtr>();
    EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); if (p == pid && IsWindowVisible(h)) r.Add(h); return true; }, IntPtr.Zero);
    return r;
  }
  public static List<IntPtr> Children(IntPtr w) {
    var r = new List<IntPtr>(); EnumChildWindows(w, (c, l) => { r.Add(c); return true; }, IntPtr.Zero); return r;
  }
  static void Walk(IntPtr m, List<KeyValuePair<string, uint>> o) {
    int n = GetMenuItemCount(m);
    for (int i = 0; i < n; i++) {
      var sb = new StringBuilder(256); GetMenuString(m, (uint)i, sb, 256, 0x400);
      IntPtr sub = GetSubMenu(m, i);
      if (sub != IntPtr.Zero) Walk(sub, o); else o.Add(new KeyValuePair<string, uint>(sb.ToString(), GetMenuItemID(m, i)));
    }
  }
  public static List<KeyValuePair<string, uint>> MenuItems(IntPtr w) {
    var o = new List<KeyValuePair<string, uint>>(); IntPtr m = GetMenu(w); if (m != IntPtr.Zero) Walk(m, o); return o;
  }
}
"@

$WM_SETTEXT = 0x000C; $WM_COMMAND = 0x0111; $BM_CLICK = 0x00F5
$CB_GETCOUNT = 0x0146; $CB_GETLBTEXT = 0x0148; $CB_SETCURSEL = 0x014E; $CBN_SELCHANGE = 1

$log = [ordered]@{
    schema = "openpipestress.caepipe_guest_round_trip.v1"
    mbf = $Mbf
    mbf_sha256 = $null
    batch = [ordered]@{}
    gui = [ordered]@{ used = $false }
    csv_path = $null
    status = "started"
    diagnostics = @()
}
function Note([string]$m) { $script:log.diagnostics += $m }
function Caption([string]$t) { ($t -replace '&', '') -replace "`t.*$", '' }

function Wait-Window([uint32]$ProcId, [scriptblock]$Match, [int]$Sec) {
    $deadline = (Get-Date).AddSeconds($Sec)
    while ((Get-Date) -lt $deadline) {
        foreach ($h in [CaepipeUi]::Windows($ProcId)) { if (& $Match $h) { return $h } }
        Start-Sleep -Milliseconds 250
    }
    return $null
}
function Find-Child([IntPtr]$Win, [string]$Class, [string]$CaptionText) {
    foreach ($c in [CaepipeUi]::Children($Win)) {
        if ([CaepipeUi]::Cls($c) -eq $Class -and (Caption ([CaepipeUi]::Text($c))) -eq $CaptionText) { return $c }
    }
    return $null
}
function Find-ChildById([IntPtr]$Win, [string]$Class, [int]$Id) {
    foreach ($c in [CaepipeUi]::Children($Win)) {
        if ([CaepipeUi]::Cls($c) -eq $Class -and [CaepipeUi]::GetDlgCtrlID($c) -eq $Id) { return $c }
    }
    return $null
}
function Click([IntPtr]$Win, [string]$CaptionText) {
    $b = Find-Child $Win "Button" $CaptionText
    if (-not $b) { throw "button '$CaptionText' not found in '$([CaepipeUi]::Text($Win))'" }
    [CaepipeUi]::PostMessage($b, $BM_CLICK, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
}
function Invoke-Menu([IntPtr]$Win, [string]$CaptionText) {
    $item = [CaepipeUi]::MenuItems($Win) | Where-Object { (Caption $_.Key) -eq $CaptionText } | Select-Object -First 1
    if (-not $item) { throw "menu item '$CaptionText' not found in '$([CaepipeUi]::Text($Win))'" }
    [CaepipeUi]::PostMessage($Win, $WM_COMMAND, [IntPtr][int]$item.Value, [IntPtr]::Zero) | Out-Null
}
function Window-Report([uint32]$ProcId) {
    $out = @()
    foreach ($h in [CaepipeUi]::Windows($ProcId)) {
        $line = [CaepipeUi]::Text($h)
        foreach ($c in [CaepipeUi]::Children($h)) {
            if ([CaepipeUi]::Cls($c) -eq "Static") { $t = [CaepipeUi]::Text($c); if ($t) { $line += " | " + ($t -replace "[\r\n]+", " ") } }
        }
        $out += $line
    }
    return $out
}
function Wait-StableFile([string]$Path, [int]$Sec) {
    $deadline = (Get-Date).AddSeconds($Sec); $last = -1
    while ((Get-Date) -lt $deadline) {
        if (Test-Path $Path) { $len = (Get-Item $Path).Length; if ($len -gt 0 -and $len -eq $last) { return $true }; $last = $len }
        Start-Sleep -Milliseconds 750
    }
    return $false
}

$mbfItem = Get-Item $Mbf
# The host compares this hash with the MBF it sent before it attributes any output to that MBF.
$log.mbf_sha256 = (Get-FileHash -LiteralPath $Mbf -Algorithm SHA256).Hash.ToLowerInvariant()
$workDir = $mbfItem.DirectoryName
$stem = [IO.Path]::GetFileNameWithoutExtension($mbfItem.Name)
$resPath = Join-Path $workDir ($stem + ".res")
$batchCsv = Join-Path $workDir ($stem + ".csv")
if (-not $CsvName) { $CsvName = $stem + "_results.csv" }
$guiCsv = Join-Path $workDir $CsvName
foreach ($p in @($resPath, $batchCsv, $guiCsv)) { if (Test-Path $p) { Remove-Item $p -Force } }

# 1. Batch import and solve.
$started = Get-Date
$proc = Start-Process -FilePath $Exe -ArgumentList ('"' + $Mbf + '";batchmode=yes') -WorkingDirectory $workDir -PassThru
$log.batch.command_shape = '<exe> "<mbf>";batchmode=yes'
$deadline = $started.AddSeconds($BatchTimeoutSec)
while ((Get-Date) -lt $deadline -and -not $proc.HasExited) {
    $blocking = Window-Report ([uint32]$proc.Id) | Where-Object { $_ -match '^Caepipe: Error|^Caepipe Trial \|' }
    if ($blocking -and (Wait-StableFile $resPath 3)) { break }
    if ($blocking -and -not (Test-Path $resPath)) { break }
    Start-Sleep -Milliseconds 500
}
$log.batch.elapsed_s = [math]::Round(((Get-Date) - $started).TotalSeconds, 1)
if ($proc.HasExited) {
    $log.batch.exited = $true; $log.batch.exit_code = $proc.ExitCode
} else {
    $log.batch.exited = $false
    $log.batch.windows_at_stop = @(Window-Report ([uint32]$proc.Id))
    Stop-Process -Id $proc.Id -Force
    $log.batch.stopped_own_process = $true
}
$log.batch.res_present = Test-Path $resPath
$log.batch.csv_present = Test-Path $batchCsv
if ($log.batch.csv_present) { $log.csv_path = $batchCsv; $log.status = "csv_from_batch"; $log | ConvertTo-Json -Depth 6; exit 0 }
if (-not $log.batch.res_present) { $log.status = "no_results_after_batch"; $log | ConvertTo-Json -Depth 6; exit 2 }

# 2. Print the solved results to CSV through the GUI.
$log.gui.used = $true
$gui = Start-Process -FilePath $Exe -WorkingDirectory (Split-Path $Exe) -PassThru
$gp = [uint32]$gui.Id
try {
    $layout = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq "CaepipeLayout" } 60
    if (-not $layout) { throw "layout window did not appear" }
    # A fresh instance may still be loading its start-up model when the window appears; a command posted
    # then is dropped, so wait and post once more if the dialog does not open.
    Start-Sleep -Seconds 2
    Invoke-Menu $layout "Open Results..."
    $open = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq "Open Results" } 15
    if (-not $open) {
        Note "Open Results command re-posted"
        Invoke-Menu $layout "Open Results..."
        $open = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq "Open Results" } 20
    }
    if (-not $open) { throw "Open Results dialog did not appear" }
    $edit = Find-ChildById $open "Edit" 1152
    if (-not $edit) { throw "file name field not found" }
    [CaepipeUi]::SendMessage($edit, $WM_SETTEXT, [IntPtr]::Zero, $resPath) | Out-Null
    Click $open "Open"
    $chooser = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq "#32770" -and [CaepipeUi]::Text($h) -eq "Results" } 30
    if ($chooser) { Click $chooser "OK" }
    $results = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq "CaepipeResults" } 30
    if (-not $results) { throw "results window did not appear" }
    Start-Sleep -Milliseconds 500
    Invoke-Menu $results "Print..."
    $print = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq "Print Results" } 20
    if (-not $print) { throw "Print Results dialog did not appear" }
    Click $print "All"
    Start-Sleep -Milliseconds 500
    Click $print "To File"
    $save = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq "Print to File" } 20
    if (-not $save) { throw "Print to File dialog did not appear" }
    $type = Find-ChildById $save "ComboBox" 1136
    if (-not $type) { throw "file type field not found" }
    $count = [CaepipeUi]::SendMessage($type, $CB_GETCOUNT, [IntPtr]::Zero, [IntPtr]::Zero).ToInt32()
    $csvIndex = -1
    for ($i = 0; $i -lt $count; $i++) {
        $sb = New-Object Text.StringBuilder 512
        [CaepipeUi]::SendMessage($type, $CB_GETLBTEXT, [IntPtr]$i, $sb) | Out-Null
        if ($sb.ToString() -like "*.csv*") { $csvIndex = $i }
    }
    if ($csvIndex -lt 0) { throw "CSV file type not offered" }
    [CaepipeUi]::SendMessage($type, $CB_SETCURSEL, [IntPtr]$csvIndex, [IntPtr]::Zero) | Out-Null
    [CaepipeUi]::PostMessage($save, $WM_COMMAND, [IntPtr](1136 -bor ($CBN_SELCHANGE -shl 16)), $type) | Out-Null
    $name = Find-ChildById $save "Edit" 1152
    [CaepipeUi]::SendMessage($name, $WM_SETTEXT, [IntPtr]::Zero, $guiCsv) | Out-Null
    Start-Sleep -Milliseconds 300
    $commit = Find-ChildById $save "Button" 1
    [CaepipeUi]::PostMessage($commit, $BM_CLICK, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    if (Wait-StableFile $guiCsv $GuiTimeoutSec) { $log.csv_path = $guiCsv; $log.status = "csv_from_gui_print" }
    else { $log.status = "csv_not_written"; $log.gui.windows = @(Window-Report $gp) }
} catch {
    $log.status = "gui_failed"
    Note $_.Exception.Message
    $log.gui.windows = @(Window-Report $gp)
} finally {
    if (-not $gui.HasExited) { Stop-Process -Id $gui.Id -Force; $log.gui.stopped_own_process = $true }
}
$log | ConvertTo-Json -Depth 6
if ($log.csv_path) { exit 0 } else { exit 3 }
