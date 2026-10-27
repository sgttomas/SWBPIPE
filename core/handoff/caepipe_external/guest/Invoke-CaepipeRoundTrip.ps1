<#
DEL-17-05 guest-side round trip for a user-owned CAEPIPE installation.

Runs inside the user's Windows environment. It imports and solves one MBF in
batch mode (CAEPIPE-BATCH, CAEPIPE-IMPORT-MBF), then, when batch mode does not
leave a CSV beside the MBF, opens the solved results in a fresh
instance and writes them to a CSV file through the documented print-to-file
command. It locates menu items and buttons by their visible captions. The script only stops processes it started.

With -ExportMod it instead opens that solved model in a fresh instance (File >
Open...), writes it as MBF through File > Export to MBF... into -ExportName
beside the model, and stops the instance (DEL-17-05 Design §3).

Output: one JSON object on stdout describing what happened, including the
SHA-256 of the MBF actually solved (mbf_sha256), or for an export the SHA-256
of the model opened (mod_sha256) and of the MBF written (exported_mbf_sha256).
CAEPIPE outputs stay in the work directory; the caller copies them to its
private run folder.
#>
param(
    [Parameter(Mandatory = $true)][string]$Exe,
    [string]$Mbf = "",
    [string]$CsvName = "",
    [string]$ExportMod = "",
    [string]$ExportName = "exported.mbf",
    [int]$BatchTimeoutSec = 120,
    [int]$GuiTimeoutSec = 120
)
$ErrorActionPreference = "Stop"

# The target tool's user interface as the round trip addresses it.
$Ui = @{
    BlockingTitle = '^Caepipe: Error|^Caepipe Trial \|'
    LayoutClass   = 'CaepipeLayout'
    FrameClass    = '^Caepipe'
    ResultsClass  = 'CaepipeResults'
    DialogClass   = '#32770'
    ModelOpenMenu = 'Open...'
    ModelOpenDialog = 'Open'
    ExportMenu    = 'Export to MBF...'
    ExportDialog  = 'Export Model to Batch File'
    MbfType       = 'Model batch file (*.mbf)'
    OpenMenu      = 'Open Results...'
    OpenDialog    = 'Open Results'
    OpenButton    = 'Open'
    ChooserDialog = 'Results'
    OkButton      = 'OK'
    PrintMenu     = 'Print...'
    PrintDialog   = 'Print Results'
    AllButton     = 'All'
    ToFileButton  = 'To File'
    SaveDialog    = 'Print to File'
    CsvType       = '*.csv*'
    FileNameId    = 1152
    FileTypeId    = 1136
    CommitId      = 1
}

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
  // Synchronous messages go through SendMessageTimeout, so a window that stops responding cannot hold the
  // script before its finally block stops the processes it started.
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern IntPtr SendMessageTimeout(IntPtr h, uint msg, IntPtr w, string l, uint flags, uint ms, out IntPtr result);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern IntPtr SendMessageTimeout(IntPtr h, uint msg, IntPtr w, StringBuilder l, uint flags, uint ms, out IntPtr result);
  [DllImport("user32.dll")] static extern IntPtr SendMessageTimeout(IntPtr h, uint msg, IntPtr w, IntPtr l, uint flags, uint ms, out IntPtr result);
  const uint SMTO_ABORTIFHUNG = 0x0002;
  const uint SEND_TIMEOUT_MS = 10000;
  static IntPtr Checked(IntPtr ok, IntPtr result) { if (ok == IntPtr.Zero) throw new TimeoutException("window did not respond within the send timeout"); return result; }
  public static IntPtr Send(IntPtr h, uint msg, IntPtr w, string l) { IntPtr r; return Checked(SendMessageTimeout(h, msg, w, l, SMTO_ABORTIFHUNG, SEND_TIMEOUT_MS, out r), r); }
  public static IntPtr Send(IntPtr h, uint msg, IntPtr w, StringBuilder l) { IntPtr r; return Checked(SendMessageTimeout(h, msg, w, l, SMTO_ABORTIFHUNG, SEND_TIMEOUT_MS, out r), r); }
  public static IntPtr Send(IntPtr h, uint msg, IntPtr w, IntPtr l) { IntPtr r; return Checked(SendMessageTimeout(h, msg, w, l, SMTO_ABORTIFHUNG, SEND_TIMEOUT_MS, out r), r); }
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
function Control-Report([IntPtr]$Win) {
    @([CaepipeUi]::Children($Win) | ForEach-Object {
        "{0} id={1} '{2}'" -f [CaepipeUi]::Cls($_), [CaepipeUi]::GetDlgCtrlID($_), (([CaepipeUi]::Text($_)) -replace "[\r\n]+", " ")
    })
}
function Window-Controls([uint32]$ProcId) {
    @([CaepipeUi]::Windows($ProcId) | ForEach-Object {
        [ordered]@{ window = "{0} '{1}'" -f [CaepipeUi]::Cls($_), [CaepipeUi]::Text($_); controls = Control-Report $_ }
    })
}
function Has-Menu([IntPtr]$Win, [string]$CaptionText) {
    [bool]([CaepipeUi]::MenuItems($Win) | Where-Object { (Caption $_.Key) -eq $CaptionText } | Select-Object -First 1)
}

function Export-SolvedModel {
    $log = [ordered]@{
        schema = "openpipestress.caepipe_guest_mbf_export.v1"
        mod = $ExportMod
        mod_sha256 = $null
        exported_mbf_path = $null
        exported_mbf_sha256 = $null
        gui = [ordered]@{}
        status = "started"
        diagnostics = @()
    }
    $modItem = Get-Item $ExportMod
    # The host compares this hash with the solved model it holds before it attributes the export to that solve.
    $log.mod_sha256 = (Get-FileHash -LiteralPath $ExportMod -Algorithm SHA256).Hash.ToLowerInvariant()
    $target = Join-Path $modItem.DirectoryName $ExportName
    if (Test-Path $target) { Remove-Item $target -Force }
    $gui = Start-Process -FilePath $Exe -WorkingDirectory (Split-Path $Exe) -PassThru
    $gp = [uint32]$gui.Id
    try {
        # A fresh instance starts either with the layout window of a start-up model or with a frame window
        # without a model; both offer File > Open....
        $start = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -match $Ui.FrameClass -and (Has-Menu $h $Ui.ModelOpenMenu) } 60
        if (-not $start) { throw "no window offering the open-model command appeared" }
        $log.gui.start_window = [CaepipeUi]::Cls($start)
        # As for Open Results: a command posted while the start-up model loads is dropped, so post it once more.
        Start-Sleep -Seconds 2
        Invoke-Menu $start $Ui.ModelOpenMenu
        $isOpen = { param($h) [CaepipeUi]::Cls($h) -eq $Ui.DialogClass -and [CaepipeUi]::Text($h) -eq $Ui.ModelOpenDialog }
        $open = Wait-Window $gp $isOpen 15
        if (-not $open) {
            $log.diagnostics += "open-model command re-posted"
            Invoke-Menu $start $Ui.ModelOpenMenu
            $open = Wait-Window $gp $isOpen 20
        }
        if (-not $open) { throw "open-model dialog did not appear" }
        $edit = Find-ChildById $open "Edit" $Ui.FileNameId
        if (-not $edit) { $log.gui.open_controls = Control-Report $open; throw "file name field not found" }
        [CaepipeUi]::Send($edit, $WM_SETTEXT, [IntPtr]::Zero, $modItem.FullName) | Out-Null
        Click $open $Ui.OpenButton
        # The layout window's title names the open model once it has loaded.
        $stem = [regex]::Escape([IO.Path]::GetFileNameWithoutExtension($modItem.Name))
        $layout = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq $Ui.LayoutClass -and [CaepipeUi]::Text($h) -match $stem } 60
        if (-not $layout) { throw "no layout window names the opened model" }
        $log.gui.layout_title = [CaepipeUi]::Text($layout)
        Start-Sleep -Milliseconds 500
        Invoke-Menu $layout $Ui.ExportMenu
        $export = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq $Ui.ExportDialog } 20
        if (-not $export) { throw "export dialog did not appear" }
        $log.gui.export_controls = Control-Report $export
        # The current-version MBF type is selected explicitly; the dialog also offers an older format.
        $type = Find-ChildById $export "ComboBox" $Ui.FileTypeId
        if (-not $type) { throw "export file type field not found" }
        $count = [CaepipeUi]::Send($type, $CB_GETCOUNT, [IntPtr]::Zero, [IntPtr]::Zero).ToInt32()
        $types = @(); $mbfIndex = -1
        for ($i = 0; $i -lt $count; $i++) {
            $sb = New-Object Text.StringBuilder 512
            [CaepipeUi]::Send($type, $CB_GETLBTEXT, [IntPtr]$i, $sb) | Out-Null
            $types += $sb.ToString()
            if ($sb.ToString() -eq $Ui.MbfType) { $mbfIndex = $i }
        }
        $log.gui.export_types = $types
        if ($mbfIndex -lt 0) { throw "MBF file type not offered" }
        [CaepipeUi]::Send($type, $CB_SETCURSEL, [IntPtr]$mbfIndex, [IntPtr]::Zero) | Out-Null
        [CaepipeUi]::PostMessage($export, $WM_COMMAND, [IntPtr]($Ui.FileTypeId -bor ($CBN_SELCHANGE -shl 16)), $type) | Out-Null
        Start-Sleep -Milliseconds 300
        $name = Find-ChildById $export "Edit" $Ui.FileNameId
        if (-not $name) { throw "export file name field not found" }
        [CaepipeUi]::Send($name, $WM_SETTEXT, [IntPtr]::Zero, $target) | Out-Null
        Start-Sleep -Milliseconds 300
        $commit = Find-ChildById $export "Button" $Ui.CommitId
        if (-not $commit) { throw "export commit button not found" }
        [CaepipeUi]::PostMessage($commit, $BM_CLICK, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
        if (Wait-StableFile $target $GuiTimeoutSec) {
            $log.exported_mbf_path = $target
            $log.exported_mbf_sha256 = (Get-FileHash -LiteralPath $target -Algorithm SHA256).Hash.ToLowerInvariant()
            $log.status = "mbf_exported"
        } else { $log.status = "mbf_not_written"; $log.gui.windows = Window-Controls $gp }
    } catch {
        $log.status = "export_failed"
        $log.diagnostics += $_.Exception.Message
        $log.gui.windows = Window-Controls $gp
    } finally {
        if (-not $gui.HasExited) { Stop-Process -Id $gui.Id -Force; $log.gui.stopped_own_process = $true }
    }
    $log | ConvertTo-Json -Depth 6
    if ($log.exported_mbf_sha256) { exit 0 } else { exit 4 }
}
if ($ExportMod) { Export-SolvedModel }
if (-not $Mbf) { throw "either -Mbf or -ExportMod is required" }

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
    $blocking = Window-Report ([uint32]$proc.Id) | Where-Object { $_ -match $Ui.BlockingTitle }
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

# 2. Fallback when the batch route yields no result file: print the solved results to CSV through the GUI.
$log.gui.used = $true
$gui = Start-Process -FilePath $Exe -WorkingDirectory (Split-Path $Exe) -PassThru
$gp = [uint32]$gui.Id
try {
    $layout = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq $Ui.LayoutClass } 60
    if (-not $layout) { throw "layout window did not appear" }
    # A fresh instance may still be loading its start-up model when the window appears; a command posted
    # then is dropped, so wait and post once more if the dialog does not open.
    Start-Sleep -Seconds 2
    Invoke-Menu $layout $Ui.OpenMenu
    $open = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq $Ui.OpenDialog } 15
    if (-not $open) {
        Note "open-results command re-posted"
        Invoke-Menu $layout $Ui.OpenMenu
        $open = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq $Ui.OpenDialog } 20
    }
    if (-not $open) { throw "open-results dialog did not appear" }
    $edit = Find-ChildById $open "Edit" $Ui.FileNameId
    if (-not $edit) { throw "file name field not found" }
    [CaepipeUi]::Send($edit, $WM_SETTEXT, [IntPtr]::Zero, $resPath) | Out-Null
    Click $open $Ui.OpenButton
    $chooser = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq $Ui.DialogClass -and [CaepipeUi]::Text($h) -eq $Ui.ChooserDialog } 30
    if ($chooser) { Click $chooser $Ui.OkButton }
    $results = Wait-Window $gp { param($h) [CaepipeUi]::Cls($h) -eq $Ui.ResultsClass } 30
    if (-not $results) { throw "results window did not appear" }
    Start-Sleep -Milliseconds 500
    Invoke-Menu $results $Ui.PrintMenu
    $print = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq $Ui.PrintDialog } 20
    if (-not $print) { throw "print dialog did not appear" }
    Click $print $Ui.AllButton
    Start-Sleep -Milliseconds 500
    Click $print $Ui.ToFileButton
    $save = Wait-Window $gp { param($h) [CaepipeUi]::Text($h) -eq $Ui.SaveDialog } 20
    if (-not $save) { throw "print-to-file dialog did not appear" }
    $type = Find-ChildById $save "ComboBox" $Ui.FileTypeId
    if (-not $type) { throw "file type field not found" }
    $count = [CaepipeUi]::Send($type, $CB_GETCOUNT, [IntPtr]::Zero, [IntPtr]::Zero).ToInt32()
    $csvIndex = -1
    for ($i = 0; $i -lt $count; $i++) {
        $sb = New-Object Text.StringBuilder 512
        [CaepipeUi]::Send($type, $CB_GETLBTEXT, [IntPtr]$i, $sb) | Out-Null
        if ($sb.ToString() -like $Ui.CsvType) { $csvIndex = $i }
    }
    if ($csvIndex -lt 0) { throw "CSV file type not offered" }
    [CaepipeUi]::Send($type, $CB_SETCURSEL, [IntPtr]$csvIndex, [IntPtr]::Zero) | Out-Null
    [CaepipeUi]::PostMessage($save, $WM_COMMAND, [IntPtr]($Ui.FileTypeId -bor ($CBN_SELCHANGE -shl 16)), $type) | Out-Null
    $name = Find-ChildById $save "Edit" $Ui.FileNameId
    [CaepipeUi]::Send($name, $WM_SETTEXT, [IntPtr]::Zero, $guiCsv) | Out-Null
    Start-Sleep -Milliseconds 300
    $commit = Find-ChildById $save "Button" $Ui.CommitId
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
