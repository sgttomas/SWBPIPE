<#
DEL-17-05 guest-side read-only queries for the harness's run phase.

The host calls this script by a short command, `-File` naming its
content-addressed copy `<exchange root>\_harness\<sha256>\Read-SwbpipeGuest.ps1`,
with one operation (-Op) and its parameters as base64 of UTF-8 JSON
(-Payload). -Data names a data file `<exchange root>\_harness\data\<sha256>.json`
whose JSON object is merged into the payload; the script hashes that file and
refuses with exit 1 when its SHA-256 differs, before using it.

The first line of every reply is the echo, exactly
  SWBPIPE-READ v1 script=<sha256 of this file> data=<sha256 of the data file|none>
The host refuses any reply whose echo is missing, malformed, or names digests
other than the ones it installed. Then each operation prints exactly one answer
in its fixed form, beginning with its own sentinel:
  cim      SWBPIPE-CIM v1 none | pid=<n> created=<yyyyMMddHHmmss.ffffff>
  job      SWBPIPE-JOB v1 absent | present active=<n|unknown> | error=<n>
  chunk    SWBPIPE-CHUNK v1 absent | size=.. sha256=.. offset=.. length=.. chunk_sha256=..<CRLF><base64>
  regread  the value as `reg query` prints it (REG_BINARY only); not found:
           stderr `ERROR: The system was unable to find the specified registry key or value.`, exit 1
  storage  SWBPIPE-STORAGE v1 <one JSON array: per folder, absent, or its final path, volume path,
           DriveFormat, DriveType and each path component's reparse and directory attributes>
  hash     SWBPIPE-HASH v1 absent | size=<n> sha256=<hex>
Any error goes to stderr with a nonzero exit.

Read-only by construction: it writes, creates, deletes and sets nothing, starts
no process, and opens no path outside C:\ and the exchange root (`Confined`).
Native calls are made through Reflection.Emit in memory (DefineDynamicAssembly
with Run access, DefinePInvokeMethod, CreateType): no compiler and no temporary
file. The pattern is taken from NATIVE in the route check's fence_check.py
(lines 85 to 96), with `::new` in place of the cmdlet that makes objects.
#>
param(
  [Parameter(Mandatory = $true)][string]$Op,
  [Parameter(Mandatory = $true)][string]$Payload,
  [string]$Data = ''
)
$ProgressPreference = 'SilentlyContinue'
$ErrorActionPreference = 'Stop'
$missing = 'ERROR: The system was unable to find the specified registry key or value.'

function Hex([byte[]]$bytes) { ([BitConverter]::ToString($bytes) -replace '-', '').ToLowerInvariant() }

function FileSha([string]$path) {
  $fs = [IO.File]::Open($path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
  try { $sha = [Security.Cryptography.SHA256]::Create(); return (Hex $sha.ComputeHash($fs)) } finally { $fs.Dispose() }
}

# The exchange root: three levels above this file (<root>\_harness\<sha256>\Read-SwbpipeGuest.ps1).
$root = [IO.Path]::GetFullPath((Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSCommandPath))))

# A path the script may open: absolute, on a drive, no stream name, and inside C:\ or the exchange root.
function Confined([string]$path) {
  if ($path -notmatch '^[A-Za-z]:\\' -or $path.IndexOf(':', 2) -ge 0) { throw "path refused: $path" }
  $full = [IO.Path]::GetFullPath($path)
  $c = $full.StartsWith('C:\', [StringComparison]::OrdinalIgnoreCase)
  $x = $full.Equals($root, [StringComparison]::OrdinalIgnoreCase) -or
       $full.StartsWith($root.TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase)
  if (-not ($c -or $x)) { throw "path outside C:\ and the exchange root refused: $path" }
  return $full
}

# In-memory P/Invoke through Reflection.Emit (NATIVE of fence_check.py, lines 85 to 96).
function Native {
  $ab = [AppDomain]::CurrentDomain.DefineDynamicAssembly([Reflection.AssemblyName]::new('SwbRead'), 'Run')
  $tb = $ab.DefineDynamicModule('SwbRead').DefineType('SwbReadN', 'Public,Class')
  $da = [Runtime.InteropServices.DllImportAttribute]
  $fields = [Reflection.FieldInfo[]]@($da.GetField('SetLastError'), $da.GetField('CharSet'))
  $values = [object[]]@($true, [Runtime.InteropServices.CharSet]::Unicode)
  $defs = @(
    @('OpenJobObject', [IntPtr], @([UInt32], [bool], [string])),
    @('QueryInformationJobObject', [bool], @([IntPtr], [Int32], [IntPtr], [Int32], [IntPtr])),
    @('CloseHandle', [bool], @([IntPtr])),
    @('CreateFile', [IntPtr], @([string], [UInt32], [UInt32], [IntPtr], [UInt32], [UInt32], [IntPtr])),
    @('GetFinalPathNameByHandle', [UInt32], @([IntPtr], [Text.StringBuilder], [UInt32], [UInt32])),
    @('GetVolumePathName', [bool], @([string], [Text.StringBuilder], [UInt32]))
  )
  foreach ($d in $defs) {
    $m = $tb.DefinePInvokeMethod($d[0], 'kernel32.dll', 'Public,Static,PinvokeImpl', 'Standard', $d[1],
                                 [Type[]]$d[2], 'Winapi', 'Unicode')
    $m.SetImplementationFlags('PreserveSig')
    $m.SetCustomAttribute([Reflection.Emit.CustomAttributeBuilder]::new($da.GetConstructor(@([string])),
                                                                       @('kernel32.dll'), $fields, $values))
  }
  return $tb.CreateType()
}

function OpCim($p) {
  $found = @(Get-CimInstance -ClassName Win32_Process -Filter ('ProcessId=' + [int]$p.pid))
  if ($found.Count -eq 0) { return "SWBPIPE-CIM v1 none`r`n" }
  if ($found.Count -ne 1 -or -not $found[0].CreationDate) { [Console]::Error.Write('partial answer'); exit 3 }
  $c = ([DateTime]$found[0].CreationDate).ToUniversalTime().ToString('yyyyMMddHHmmss.ffffff')
  return 'SWBPIPE-CIM v1 pid=' + [int]$found[0].ProcessId + ' created=' + $c + "`r`n"
}

# The job by name: OpenJobObject fails with ERROR_FILE_NOT_FOUND (2) only when no member lives and no handle is open.
function OpJob($p) {
  if ($p.job -notmatch '^swbpipe-[0-9a-f]{32}$') { throw 'job name refused' }
  $n = Native
  $job = $n::OpenJobObject(0x0004, $false, [string]$p.job)
  if ($job -eq [IntPtr]::Zero) {
    $e = [Runtime.InteropServices.Marshal]::GetLastWin32Error()
    if ($e -eq 2) { return "SWBPIPE-JOB v1 absent`r`n" }
    return "SWBPIPE-JOB v1 error=$e`r`n"
  }
  try {
    $info = [Runtime.InteropServices.Marshal]::AllocHGlobal(48)
    try {
      if (-not $n::QueryInformationJobObject($job, 1, $info, 48, [IntPtr]::Zero)) {
        return "SWBPIPE-JOB v1 present active=unknown`r`n"
      }
      return 'SWBPIPE-JOB v1 present active=' + [Runtime.InteropServices.Marshal]::ReadInt32($info, 40) + "`r`n"
    } finally { [Runtime.InteropServices.Marshal]::FreeHGlobal($info) }
  } finally { [void]$n::CloseHandle($job) }
}

# One chunk of one file, opened for reading only: the whole file's size and SHA-256, the chunk's offset, length and
# SHA-256, then the chunk in base64. A missing file is `absent`.
function OpChunk($p) {
  $path = Confined ([string]$p.path)
  if (-not [IO.File]::Exists($path)) { return "SWBPIPE-CHUNK v1 absent`r`n" }
  $fs = [IO.File]::Open($path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
  try {
    $size = $fs.Length
    $sha = [Security.Cryptography.SHA256]::Create()
    $whole = Hex $sha.ComputeHash($fs)
    $offset = [int64]$p.offset
    $n = [int][Math]::Max(0, [Math]::Min([int64]$p.length, $size - $offset))
    $buf = [byte[]]::new($n)
    $fs.Position = $offset
    $read = 0
    while ($read -lt $n) { $k = $fs.Read($buf, $read, $n - $read); if ($k -le 0) { throw 'short read' }; $read += $k }
    $part = Hex $sha.ComputeHash($buf)
    return "SWBPIPE-CHUNK v1 size=$size sha256=$whole offset=$offset length=$n chunk_sha256=$part`r`n" +
           [Convert]::ToBase64String($buf) + "`r`n"
  } finally { $fs.Dispose() }
}

# One REG_BINARY value of the current user's hive, read through the registry API with the key opened read-only.
function OpRegread($p) {
  if ([string]$p.key -notmatch '^[A-Za-z0-9 ._()-]+(\\[A-Za-z0-9 ._()-]+)*$') { throw 'key refused' }
  $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey([string]$p.key)
  if ($null -eq $key) { [Console]::Error.Write($missing); exit 1 }
  try {
    if (-not ($key.GetValueNames() -ccontains $p.name)) { [Console]::Error.Write($missing); exit 1 }
    if ($key.GetValueKind($p.name) -ne [Microsoft.Win32.RegistryValueKind]::Binary) { exit 2 }
    $hex = ([BitConverter]::ToString([byte[]]$key.GetValue($p.name))) -replace '-', ''
    return "`r`nHKEY_CURRENT_USER\" + $p.key + "`r`n    " + $p.name + '    REG_BINARY    ' + $hex + "`r`n"
  } finally { $key.Close() }
}

# Where each folder lives: its final path (opened with access 0 and FILE_FLAG_BACKUP_SEMANTICS), its volume path,
# the volume's DriveFormat and DriveType, and each path component's reparse and directory attributes.
function OpStorage($p) {
  $n = Native
  $out = [Collections.ArrayList]::new()
  foreach ($folder in @($p.folders)) {
    $path = Confined ([string]$folder)
    if (-not (Test-Path -LiteralPath $path -PathType Container)) {
      [void]$out.Add([ordered]@{ folder = $path; absent = $true }); continue
    }
    $parts = $path.TrimEnd('\').Split('\')
    $comp = [Collections.ArrayList]::new()
    for ($i = 2; $i -le $parts.Length; $i++) {
      $c = ($parts[0..($i - 1)] -join '\')
      $a = (Get-Item -LiteralPath $c -Force).Attributes
      [void]$comp.Add([ordered]@{ path = $c; reparse = [bool]($a -band [IO.FileAttributes]::ReparsePoint)
                                  directory = [bool]($a -band [IO.FileAttributes]::Directory) })
    }
    $h = $n::CreateFile($path, 0, 7, [IntPtr]::Zero, 3, 0x02000000, [IntPtr]::Zero)
    $final = $null
    if ($h -ne [IntPtr](-1)) {
      $b = [Text.StringBuilder]::new(1024)
      $k = $n::GetFinalPathNameByHandle($h, $b, 1024, 0)
      if ($k -gt 0 -and $k -lt 1024) { $final = $b.ToString() }
      [void]$n::CloseHandle($h)
    }
    $vb = [Text.StringBuilder]::new(1024)
    $vol = if ($n::GetVolumePathName($path, $vb, 1024)) { $vb.ToString() } else { $null }
    $drive = if ($vol) { [IO.DriveInfo]::new($vol.Substring(0, 1)) } else { $null }
    [void]$out.Add([ordered]@{ folder = $path; absent = $false; final = $final; volume = $vol
                               format = $(if ($drive) { $drive.DriveFormat }); type = $(if ($drive) { "$($drive.DriveType)" })
                               components = $comp.ToArray() })
  }
  return 'SWBPIPE-STORAGE v1 ' + (ConvertTo-Json -InputObject $out.ToArray() -Depth 5 -Compress) + "`r`n"
}

function OpHash($p) {
  $path = Confined ([string]$p.path)
  if (-not [IO.File]::Exists($path)) { return "SWBPIPE-HASH v1 absent`r`n" }
  $size = ([IO.FileInfo]::new($path)).Length
  return "SWBPIPE-HASH v1 size=$size sha256=" + (FileSha $path) + "`r`n"
}

try {
  $script = FileSha $PSCommandPath
  $dataSha = 'none'
  $merged = @{}
  if ($Data) {
    if ($Data -notmatch '^[0-9a-f]{64}$') { throw 'data digest refused' }
    $dataPath = Confined ($root.TrimEnd('\') + '\_harness\data\' + $Data + '.json')
    $dataSha = FileSha $dataPath
  }
  [Console]::Out.Write("SWBPIPE-READ v1 script=$script data=$dataSha`r`n")
  if ($Data -and $dataSha -ne $Data) { [Console]::Error.Write('the data file differs from its digest'); exit 1 }
  # The parameters, in an in-memory table: the payload's members, then the data file's (never both naming one).
  $p = @{}
  $given = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String($Payload)) | ConvertFrom-Json
  foreach ($q in $given.PSObject.Properties) { $p[$q.Name] = $q.Value }
  if ($Data) {
    $extra = [Text.Encoding]::UTF8.GetString([IO.File]::ReadAllBytes($dataPath)) | ConvertFrom-Json
    foreach ($q in $extra.PSObject.Properties) {
      if ($p.ContainsKey($q.Name)) { throw "payload and data both name $($q.Name)" }
      $p[$q.Name] = $q.Value
    }
  }
  switch -CaseSensitive ($Op) {
    'cim' { $answer = OpCim $p }
    'job' { $answer = OpJob $p }
    'chunk' { $answer = OpChunk $p }
    'regread' { $answer = OpRegread $p }
    'storage' { $answer = OpStorage $p }
    'hash' { $answer = OpHash $p }
    default { throw "unknown operation: $Op" }
  }
  [Console]::Out.Write($answer)
} catch { [Console]::Error.Write($_.Exception.Message); exit 1 }
exit 0
