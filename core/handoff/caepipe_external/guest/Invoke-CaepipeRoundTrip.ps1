<#
DEL-17-05 guest-side round trip for a user-owned CAEPIPE installation.

Runs inside the user's Windows environment. It imports and solves one MBF in
batch mode (CAEPIPE-BATCH, CAEPIPE-IMPORT-MBF), then, when batch mode does not
leave a CSV beside the MBF, opens the solved results in a fresh
instance and writes them to a CSV file through the documented print-to-file
command. It locates menu items and buttons by their visible captions. The script only stops processes it started.

One invocation is one attempt, named by the host's attempt token T
(-AttemptToken), in the case's work folder W (-WorkRoot). The program is started
only by an invocation that has won the claim on T. In order, and nothing before
step 2 but step 1:
  1. W is created if absent (idempotent; it touches no attempt's state).
  2. Claim: W\T.claim is created exclusively (CreateNew; it fails if the name
     exists and never overwrites) and holds T, "script", this process's id and
     creation time, and a nonce. On any failure the script exits without effect.
     The claim is never deleted or rewritten.
  3. The named job object swbpipe-T is created and this process assigned to it;
     every process it starts is a member from creation. Then W\T.job is created
     exclusively. On any failure the script exits: nothing is started.
  4. The attempt folder W\T\ is created, refusing if it exists.
  5. The staged MBF (-Mbf) is copied to W\T\model.mbf and its SHA-256 compared
     with -ExpectedMbfSha256. On a mismatch the terminal report says
     input_mismatch and nothing is started.
  6. The program is started in W\T\ (batchmode=yes) and polled; the script stops
     its own process at the time limit. Without -NoGuiFallback, a batch run that
     leaves no CSV falls back to printing through the GUI, inside the same claim
     and job.
  7. Terminal report: W\T.finished is created exclusively, holding T and the
     report; the report is then printed to stdout as well.
The script never writes into the exchange folder; the host reads W\T\ itself.

Without -AttemptToken nothing is started: a solve (-Mbf) is refused as
claim_required. The export (-ExportMod) is held: it starts the program only
inside a won claim and job, which it has not adopted, so it is refused as
export_held and starts nothing (DEL-17-05, Scope of work).

With -PidFile the script appends the id of every process it starts to that file at
once, so the host can confirm from the process list that each has ended even when
it never receives this script's report. With -NoGuiFallback a batch run that leaves
no CSV ends as batch_no_csv instead of falling back to the GUI.

With -Exercise (the harness's controlled exercise, never a solve) the same attempt
runs in C:\oracle\SWBPIPE-harness-exercise-<YYYYMMDD> with every name prefixed
SWBPIPE-harness-exercise-: claim, job, folder and start path as above, with a
fixed harmless process (powershell.exe -NoProfile -NonInteractive -Command
"Start-Sleep -Seconds 5") started in place of the program and no MBF; its
terminal report says exercise_ended or exercise_failed.

Output: one JSON object on stdout describing what happened, including the
attempt token and the SHA-256 of the MBF actually solved (mbf_sha256).
CAEPIPE outputs stay in the attempt folder; the host reads them from there.
#>
param(
    [string]$Exe = "",
    [string]$Mbf = "",
    [string]$CsvName = "",
    [string]$ExportMod = "",
    [string]$ExportName = "exported.mbf",
    [int]$BatchTimeoutSec = 120,
    [int]$GuiTimeoutSec = 120,
    [string]$PidFile = "",
    [switch]$NoGuiFallback,
    [string]$AttemptToken = "",
    [string]$ExpectedMbfSha256 = "",
    [string]$WorkRoot = "",
    [switch]$Exercise
)
# The first statement (PowerShell requires the param block before it): no progress record, a module's first-use
# preparation included, is serialised to the host's stderr. Warnings and errors stay visible.
$ProgressPreference = 'SilentlyContinue'
$ErrorActionPreference = "Stop"

# Nothing below starts a process before the claim is won (Start-Attempt); these refusals start nothing.
if ($ExportMod) {
    [ordered]@{ schema = "openpipestress.caepipe_guest_mbf_export.v1"; status = "export_held"
                diagnostics = @("the export starts the program only inside a won claim and job, which it has not adopted") } |
        ConvertTo-Json -Compress
    exit 6
}
if (-not $Exercise -and (-not $Exe -or -not $Mbf)) { throw "-Exe and -Mbf are required" }
if (-not $AttemptToken) {
    [ordered]@{ schema = "openpipestress.caepipe_guest_round_trip.v1"; status = "claim_required"
                diagnostics = @("the program is started only by an attempt that has won its claim (-AttemptToken)") } |
        ConvertTo-Json -Compress
    exit 7
}
if ($AttemptToken -cnotmatch '^[0-9a-f]{32}$' -or -not $WorkRoot -or
    (-not $Exercise -and $ExpectedMbfSha256 -cnotmatch '^[0-9a-f]{64}$')) {
    throw "-AttemptToken (32 lower-case hex), -ExpectedMbfSha256 (64 lower-case hex) and -WorkRoot are required"
}
if ($Exercise -and ($WorkRoot -cnotmatch '^C:\\oracle\\SWBPIPE-harness-exercise-[0-9]{8}$' -or $Exe -or $Mbf)) {
    throw "-Exercise runs only in C:\oracle\SWBPIPE-harness-exercise-<YYYYMMDD>, with no -Exe and no -Mbf"
}

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
public class CaepipeJob {
  [DllImport("kernel32.dll", SetLastError=true, CharSet=CharSet.Unicode)] static extern IntPtr CreateJobObject(IntPtr attributes, string name);
  [DllImport("kernel32.dll", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
  [DllImport("kernel32.dll")] static extern IntPtr GetCurrentProcess();
  const int ERROR_ALREADY_EXISTS = 183;
  // Creates the named job and assigns this process to it. The handle is kept open for the life of this process.
  // Refuses (throws) when the name already exists: only this attempt's claim owner may create it.
  public static IntPtr CreateAndJoin(string name) {
    IntPtr job = CreateJobObject(IntPtr.Zero, name);
    int error = Marshal.GetLastWin32Error();
    if (job == IntPtr.Zero) throw new InvalidOperationException("CreateJobObject failed: " + error);
    if (error == ERROR_ALREADY_EXISTS) throw new InvalidOperationException("the job object already exists");
    if (!AssignProcessToJobObject(job, GetCurrentProcess()))
      throw new InvalidOperationException("AssignProcessToJobObject failed: " + Marshal.GetLastWin32Error());
    return job;
  }
}
"@

$WM_SETTEXT = 0x000C; $WM_COMMAND = 0x0111; $BM_CLICK = 0x00F5
$CB_GETCOUNT = 0x0146; $CB_GETLBTEXT = 0x0148; $CB_SETCURSEL = 0x014E; $CBN_SELCHANGE = 1

$log = [ordered]@{
    schema = "openpipestress.caepipe_guest_round_trip.v1"
    attempt_token = $AttemptToken
    mbf = $Mbf
    mbf_sha256 = $null
    batch = [ordered]@{}
    gui = [ordered]@{ used = $false }
    csv_path = $null
    owned_pids = @()
    status = "started"
    diagnostics = @()
}
function Note([string]$m) { $script:log.diagnostics += $m }
function Record-Owned($p) {
    $script:log.owned_pids += [int]$p.Id
    if ($PidFile) { Add-Content -LiteralPath $PidFile -Value $p.Id -Encoding Ascii }
}
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

# Held (see the header): never called while the export has not adopted the claim and job; kept for that change.
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
# ---------------------------------------------------------------------------------------------------------------
# The attempt. Steps 1 to 7 of the header; nothing is started before steps 2 and 3 succeed in this invocation.
# ---------------------------------------------------------------------------------------------------------------
# The host builds the same names with the same join: <W>\<P><T>.claim, <W>\<P><T>.job, <W>\<P><T>.finished and
# <W>\<P><T>\, the prefix P empty except in the exercise.
$prefix = if ($Exercise) { 'SWBPIPE-harness-exercise-' } else { '' }
$claimPath = $WorkRoot + '\' + $prefix + $AttemptToken + '.claim'
$jobPath = $WorkRoot + '\' + $prefix + $AttemptToken + '.job'
$finishedPath = $WorkRoot + '\' + $prefix + $AttemptToken + '.finished'
$attemptDir = $WorkRoot + '\' + $prefix + $AttemptToken

function New-ExclusiveFile([string]$Path, [string[]]$Lines) {
    # CreateNew fails with ERROR_FILE_EXISTS when the name exists and never overwrites; FileShare.None keeps every
    # reader out until the content, written through the creating handle, is complete and the handle closed.
    $stream = [IO.File]::Open($Path, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
    try {
        $bytes = [Text.Encoding]::ASCII.GetBytes(($Lines -join "`r`n") + "`r`n")
        $stream.Write($bytes, 0, $bytes.Length)
        $stream.Flush($true)
    } finally { $stream.Dispose() }
}
function Refuse-Attempt([string]$Status, [string]$Message, [int]$Code) {
    # Before the attempt folder exists: no terminal report, nothing started.
    [ordered]@{ schema = $script:log.schema; attempt_token = $AttemptToken; status = $Status; diagnostics = @($Message) } |
        ConvertTo-Json -Compress
    exit $Code
}
$script:finishing = $false
function Finish-Attempt([int]$Code) {
    # 7. The terminal report, created exclusively and holding T, then printed. ASCII only: any other character is
    # written as a JSON escape, so the file reads back byte for byte whatever the console's code page.
    $script:finishing = $true
    $report = [regex]::Replace(($script:log | ConvertTo-Json -Compress -Depth 6), '[^\x20-\x7E]',
                               { param($m) '\u{0:x4}' -f [int][char]$m.Value })
    New-ExclusiveFile $finishedPath @("swbpipe-finished v1", "token=$AttemptToken", "report=$report")
    $report
    exit $Code
}

# 1. The work folder, idempotent.
try { [void][IO.Directory]::CreateDirectory($WorkRoot) } catch { Refuse-Attempt "work_folder_failed" $_.Exception.Message 19 }

# 2. The claim. Its content is this process's identity, read before the claim is made.
try {
    $self = Get-CimInstance -ClassName Win32_Process -Filter "ProcessId=$PID"
    if (-not $self -or -not $self.CreationDate) { throw "this process's identity could not be read" }
    $created = ([DateTime]$self.CreationDate).ToUniversalTime().ToString('yyyyMMddHHmmss.ffffff')
    $nonceBytes = New-Object byte[] 16
    [Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($nonceBytes)
    $nonce = -join ($nonceBytes | ForEach-Object { $_.ToString('x2') })
    New-ExclusiveFile $claimPath @("swbpipe-claim v1", "token=$AttemptToken", "owner=script", "pid=$PID",
                                   "created=$created", "nonce=$nonce")
} catch { Refuse-Attempt "claim_not_won" $_.Exception.Message 20 }
$log.claim = [ordered]@{ pid = $PID; created = $created }

# 3. The job: this process joins it before anything is started, so every process it starts is a member from
# creation (no breakaway is ever requested). Then the marker.
try {
    $script:jobHandle = [CaepipeJob]::CreateAndJoin("swbpipe-" + $AttemptToken)
    New-ExclusiveFile $jobPath @("swbpipe-job v1", "token=$AttemptToken", "job=swbpipe-$AttemptToken")
} catch { Refuse-Attempt "job_not_made" $_.Exception.Message 21 }

$proc = $null; $gui = $null
try {
    # 4. The fresh attempt folder. Only the claim's owner reaches this step, so an existing one is a defect.
    if (Test-Path -LiteralPath $attemptDir) { $log.status = "attempt_folder_exists"; Finish-Attempt 22 }
    [void][IO.Directory]::CreateDirectory($attemptDir)
    if ($Exercise) {
        # The controlled exercise: this attempt's claim, job and folder, and the same start path, with a fixed
        # harmless process in place of the program.
        $started = Get-Date
        $proc = Start-Process -FilePath (Join-Path $PSHOME 'powershell.exe') -WorkingDirectory $attemptDir -PassThru `
            -ArgumentList '-NoProfile -NonInteractive -Command "Start-Sleep -Seconds 5"'
        Record-Owned $proc
        $log.exercise = [ordered]@{ command_shape = 'powershell.exe -NoProfile -NonInteractive -Command "Start-Sleep -Seconds 5"' }
        $deadline = $started.AddSeconds($BatchTimeoutSec)
        while ((Get-Date) -lt $deadline -and -not $proc.HasExited) { Start-Sleep -Milliseconds 500 }
        if ($proc.HasExited) {
            $log.exercise.exit_code = $proc.ExitCode
            $log.status = if ($proc.ExitCode -eq 0) { "exercise_ended" } else { "exercise_failed" }
        } else {
            Stop-Process -Id $proc.Id -Force
            $log.exercise.stopped_own_process = $true
            $log.status = "exercise_failed"
        }
        if ($log.status -eq "exercise_ended") { Finish-Attempt 0 } else { Finish-Attempt 3 }
    }
    # 5. The verified copy of the staged MBF.
    $workMbf = $attemptDir + '\model.mbf'
    Copy-Item -LiteralPath $Mbf -Destination $workMbf
    $log.mbf = $workMbf
    # The host compares this hash with the MBF it sent before it attributes any output to that MBF.
    $log.mbf_sha256 = (Get-FileHash -LiteralPath $workMbf -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($log.mbf_sha256 -cne $ExpectedMbfSha256) { $log.status = "input_mismatch"; Finish-Attempt 8 }
    if ($PidFile -and -not $PidFile.StartsWith($attemptDir + '\')) { throw "-PidFile must lie in the attempt folder" }

    $mbfItem = Get-Item -LiteralPath $workMbf
    $workDir = $mbfItem.DirectoryName
    $stem = [IO.Path]::GetFileNameWithoutExtension($mbfItem.Name)
    $resPath = Join-Path $workDir ($stem + ".res")
    $batchCsv = Join-Path $workDir ($stem + ".csv")
    # The batch print takes the extension saved in the tool's preferences: .csv or .txt.
    $batchTxt = Join-Path $workDir ($stem + ".txt")
    if (-not $CsvName) { $CsvName = $stem + "_results.csv" }
    $guiCsv = Join-Path $workDir $CsvName

    # 6. Batch import and solve, in the attempt folder.
    $started = Get-Date
    $proc = Start-Process -FilePath $Exe -ArgumentList ('"' + $workMbf + '";batchmode=yes') -WorkingDirectory $workDir -PassThru
    Record-Owned $proc
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
    $log.batch.txt_present = Test-Path $batchTxt
    # Two print files: which one is the print cannot be told, so neither is reported (the host refuses both).
    if ($log.batch.csv_present -and $log.batch.txt_present) { $log.status = "print_file_ambiguous"; Finish-Attempt 5 }
    if ($log.batch.csv_present) { $log.csv_path = $batchCsv; $log.status = "csv_from_batch"; Finish-Attempt 0 }
    # A .txt print is reported as the batch print; the host accepts it only if it parses as the print-CSV layout.
    if ($log.batch.txt_present) { $log.csv_path = $batchTxt; $log.status = "csv_from_batch"; Finish-Attempt 0 }
    if (-not $log.batch.res_present) { $log.status = "no_results_after_batch"; Finish-Attempt 2 }
    if ($NoGuiFallback) { $log.status = "batch_no_csv"; Finish-Attempt 3 }

    # 2. Fallback when the batch route yields no result file: print the solved results to CSV through the GUI.
    $log.gui.used = $true
    $gui = Start-Process -FilePath $Exe -WorkingDirectory (Split-Path $Exe) -PassThru
    Record-Owned $gui
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
} catch {
    if ($script:finishing) { throw }  # the terminal report itself failed: no report, the host reconciles
    if ($proc -and -not $proc.HasExited) { Stop-Process -Id $proc.Id -Force; $log.batch.stopped_own_process = $true }
    if ($gui -and -not $gui.HasExited) { Stop-Process -Id $gui.Id -Force; $log.gui.stopped_own_process = $true }
    $log.status = "attempt_failed"
    Note $_.Exception.Message
}
if ($log.csv_path) { Finish-Attempt 0 } else { Finish-Attempt 3 }
