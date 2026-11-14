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
  job      SWBPIPE-JOB v1 present active=<n|unknown> (opened, whatever the canary) | absent
           (not opened, error 2, canary 6) | error=canary-<n> (not opened, canary not 6) | error=<n>
  chunk    SWBPIPE-CHUNK v1 absent | size=.. sha256=.. offset=.. length=.. chunk_sha256=..<CRLF><base64>
  regread  the value as `reg query` prints it (REG_BINARY only); not found:
           stderr `ERROR: The system was unable to find the specified registry key or value.`, exit 1
  storage  SWBPIPE-STORAGE v1 <one JSON array: per folder, absent, or its final path, volume path,
           DriveFormat, DriveType and each path component's reparse and directory attributes; at a
           reparse component the walk stops, and final, volume, format and type are null>
  hash     SWBPIPE-HASH v1 absent | size=<n> sha256=<hex>
Any error goes to stderr with a nonzero exit.

Read-only by construction: the script's own operations write, create, delete and
set nothing, and it starts no process. It names no path outside C:\ and the
exchange root (`Confined`, a lexical check), and it refuses a path through a
reparse point before opening it (`Unreparsed`, which walks the path's components
by their attributes and does not follow the last); the storage answer reports a
reparse component and stops its walk there, opening nothing through it. In
answering the `cim` query the WMI service may start its own provider host
(WmiPrvSE.exe); `job`'s OpenJobObject, `regread`'s read-only OpenSubKey and the
storage, chunk and hash opens create nothing and start nothing. Windows
PowerShell itself, as on every call, may write its own event-log records (for
this script, an automatic warning-level script-block record of its
Reflection.Emit content among them) and its module analysis cache and startup
profile data under %LOCALAPPDATA%, and its command discovery lists the folders
on PSModulePath; these are not the script's operations.
Native calls are made through Reflection.Emit in memory (DefineDynamicAssembly
with Run access, DefineMethod with PinvokeImpl and a DllImportAttribute that sets
SetLastError, CreateType): no compiler and no temporary file. The pattern is
taken from NATIVE in the route check's fence_check.py (lines 85 to 96), with
`::new` in place of the cmdlet that makes objects, and each method defined by
DefineMethod rather than DefinePInvokeMethod, so that its DllImportAttribute
creates the P/Invoke map with the last-error flag. The job query's helper
(OpenJobChecked) is emitted the same way, with DefineMethod and its ILGenerator,
so that OpenJobObject's last error, and a canary's before it, are read in the
same frame as each call.
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

# Refuse a path through a reparse point, walking components by attributes (GetFileAttributes does not follow the last).
function Unreparsed([string]$path) {
  $parts = $path.TrimEnd('\').Split('\')
  for ($i = 2; $i -le $parts.Length; $i++) {
    $c = ($parts[0..($i - 1)] -join '\')
    try { $a = [IO.File]::GetAttributes($c) } catch [IO.FileNotFoundException], [IO.DirectoryNotFoundException] { return }
    if ($a -band [IO.FileAttributes]::ReparsePoint) { throw "path through a reparse point refused: $c" }
  }
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
  $native = @{}
  foreach ($d in $defs) {
    # A plain static method marked PinvokeImpl, whose DllImportAttribute then creates its P/Invoke map with
    # SetLastError (the last-error flag). DefinePInvokeMethod would write that map first, without the flag, and the
    # attribute could not add it afterwards.
    $m = $tb.DefineMethod($d[0], 'Public,Static,PinvokeImpl', $d[1], [Type[]]$d[2])
    $m.SetImplementationFlags('PreserveSig')
    $m.SetCustomAttribute([Reflection.Emit.CustomAttributeBuilder]::new($da.GetConstructor(@([string])),
                                                                       @('kernel32.dll'), $fields, $values))
    $native[$d[0]] = $m
  }
  # OpenJobChecked(name, out error, out canary): in one frame, so no other call on the thread can overwrite the saved
  # last error in between. First the canary: CloseHandle(NULL) fails with ERROR_INVALID_HANDLE (6), so a canary of 6
  # shows the emitted SetLastError takes effect. Then OpenJobObject, its last error read at once. Returns the handle.
  $o = [Reflection.Emit.OpCodes]
  $lastError = [Runtime.InteropServices.Marshal].GetMethod('GetLastWin32Error')
  $helper = $tb.DefineMethod('OpenJobChecked', 'Public,Static', [IntPtr],
                             [Type[]]@([string], [int].MakeByRefType(), [int].MakeByRefType()))
  $il = $helper.GetILGenerator()
  $handle = $il.DeclareLocal([IntPtr])
  $il.Emit($o::Ldarg_2)
  $il.Emit($o::Ldsfld, [IntPtr].GetField('Zero'))
  $il.Emit($o::Call, [Reflection.MethodInfo]$native['CloseHandle'])
  $il.Emit($o::Pop)
  $il.Emit($o::Call, $lastError)
  $il.Emit($o::Stind_I4)
  $il.Emit($o::Ldarg_1)
  $il.Emit($o::Ldc_I4, [int]4)
  $il.Emit($o::Ldc_I4_0)
  $il.Emit($o::Ldarg_0)
  $il.Emit($o::Call, [Reflection.MethodInfo]$native['OpenJobObject'])
  $il.Emit($o::Stloc, $handle)
  $il.Emit($o::Call, $lastError)
  $il.Emit($o::Stind_I4)
  $il.Emit($o::Ldloc, $handle)
  $il.Emit($o::Ret)
  return $tb.CreateType()  # once, after every method is defined
}

function OpCim($p) {
  $found = @(Get-CimInstance -ClassName Win32_Process -Filter ('ProcessId=' + [int]$p.pid))
  if ($found.Count -eq 0) { return "SWBPIPE-CIM v1 none`r`n" }
  if ($found.Count -ne 1 -or -not $found[0].CreationDate) { [Console]::Error.Write('partial answer'); exit 3 }
  $c = ([DateTime]$found[0].CreationDate).ToUniversalTime().ToString('yyyyMMddHHmmss.ffffff')
  return 'SWBPIPE-CIM v1 pid=' + [int]$found[0].ProcessId + ' created=' + $c + "`r`n"
}

# The job by name: OpenJobObject fails with ERROR_FILE_NOT_FOUND (2) only when no member lives and no handle is open.
# Its last error is captured in the same frame as the call (OpenJobChecked), behind a canary that must be 6. In order:
# a handle is presence, whatever the canary says (the canary guards the reading of failure codes, not a handle); a
# failure under a canary other than 6 is an error; a failure with error 2 is absent; any other failure is its error.
function OpJob($p) {
  if ($p.job -notmatch '^swbpipe-[0-9a-f]{32}\z') { throw 'job name refused' }
  $n = Native
  [int]$openError = 0
  [int]$canary = 0
  $job = $n::OpenJobChecked([string]$p.job, [ref]$openError, [ref]$canary)
  if ($job -eq [IntPtr]::Zero) {
    if ($canary -ne 6) { return "SWBPIPE-JOB v1 error=canary-$canary`r`n" }
    if ($openError -eq 2) { return "SWBPIPE-JOB v1 absent`r`n" }
    return "SWBPIPE-JOB v1 error=$openError`r`n"
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
  Unreparsed $path
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
  if ([string]$p.key -notmatch '^[A-Za-z0-9 ._()-]+(\\[A-Za-z0-9 ._()-]+)*\z') { throw 'key refused' }
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
    $parts = $path.TrimEnd('\').Split('\')
    $comp = [Collections.ArrayList]::new()
    $absent = $false; $through = $false
    for ($i = 2; $i -le $parts.Length; $i++) {
      $c = ($parts[0..($i - 1)] -join '\')
      try { $a = [IO.File]::GetAttributes($c) }
      catch [IO.FileNotFoundException], [IO.DirectoryNotFoundException] { $absent = $true; break }
      $r = [bool]($a -band [IO.FileAttributes]::ReparsePoint)
      [void]$comp.Add([ordered]@{ path = $c; reparse = $r; directory = [bool]($a -band [IO.FileAttributes]::Directory) })
      if ($r) { $through = $true; break }
    }
    if ($absent) { [void]$out.Add([ordered]@{ folder = $path; absent = $true }); continue }
    if ($through) {
      [void]$out.Add([ordered]@{ folder = $path; absent = $false; final = $null; volume = $null; format = $null
                                 type = $null; components = $comp.ToArray() }); continue
    }
    $h = $n::CreateFile($path, 0, 7, [IntPtr]::Zero, 3, 0x02200000, [IntPtr]::Zero)
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
  Unreparsed $path
  if (-not [IO.File]::Exists($path)) { return "SWBPIPE-HASH v1 absent`r`n" }
  $size = ([IO.FileInfo]::new($path)).Length
  return "SWBPIPE-HASH v1 size=$size sha256=" + (FileSha $path) + "`r`n"
}

try {
  $script = FileSha $PSCommandPath
  $dataSha = 'none'
  $merged = @{}
  if ($Data) {
    if ($Data -notmatch '^[0-9a-f]{64}\z') { throw 'data digest refused' }
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
