<#
DEL-17-05 guest-side changing operations for the harness's run phase, each confined by pattern.

The host calls this script by a short command, `-File` naming its
content-addressed copy `<exchange root>\_harness\<sha256>\Write-SwbpipeGuest.ps1`,
with one operation (-Op) and its parameters as base64 of UTF-8 JSON
(-Payload). -Data names a data file `<exchange root>\_harness\data\<sha256>.json`
whose JSON object is merged into the payload; the script hashes that file and
refuses with exit 1 when its SHA-256 differs, before using it.

The first line of every reply is the echo, exactly
  SWBPIPE-WRITE v1 script=<sha256 of this file> data=<sha256 of the data file|none>
The host refuses any reply whose echo is missing, malformed, or names digests
other than the ones it installed. Then each operation prints one answer:
  claim      SWBPIPE-CLAIM v1 created | exists | held
             the folder (idempotent), then the exclusive create (CreateNew, never an
             overwrite) of one claim file with the given ASCII content; `exists` is
             ERROR_FILE_EXISTS, `held` is a sharing violation (another holds the name)
  regset     `The operation completed successfully.` after setting one value of an existing
             key of the current user's hive to REG_BINARY data (a value of another type is
             replaced, as a restore to a REG_BINARY first reading needs)
  regdelete  the same after deleting one REG_BINARY value (a value of another type is refused)
  keycreate  the same after creating the harness's exercise key (only that key)
  keydelete  the same after deleting the harness's exercise key and its values (only that key)
A missing key or value: stderr `ERROR: The system was unable to find the specified
registry key or value.`, exit 1. Any other error goes to stderr with a nonzero exit.

What it may change, and nothing else:
  - a claim file named `<32 hex>.claim` (an attempt's fence) in a folder on C:\, or
    `SWBPIPE-harness-exercise-<name>` in C:\oracle\SWBPIPE-harness-exercise-<YYYYMMDD>, and that
    folder, created with any missing parent when it is absent;
  - one value of an existing key under HKCU\Software: set to REG_BINARY data, or deleted
    when it is REG_BINARY;
  - the key HKCU\Software\SWBPIPE-harness-exercise-<YYYYMMDD> (create, delete).
It starts no process and reads or writes no other path.
#>
param(
  [Parameter(Mandatory = $true)][string]$Op,
  [Parameter(Mandatory = $true)][string]$Payload,
  [string]$Data = ''
)
$ProgressPreference = 'SilentlyContinue'
$ErrorActionPreference = 'Stop'
$missing = 'ERROR: The system was unable to find the specified registry key or value.'
$ok = 'The operation completed successfully.'
$exerciseFolder = '^C:\\oracle\\SWBPIPE-harness-exercise-[0-9]{8}\z'
$exerciseKey = '^Software\\SWBPIPE-harness-exercise-[0-9]{8}\z'

function Hex([byte[]]$bytes) { ([BitConverter]::ToString($bytes) -replace '-', '').ToLowerInvariant() }

function FileSha([string]$path) {
  $fs = [IO.File]::Open($path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
  try { $sha = [Security.Cryptography.SHA256]::Create(); return (Hex $sha.ComputeHash($fs)) } finally { $fs.Dispose() }
}

$root = [IO.Path]::GetFullPath((Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $PSCommandPath))))

# A folder on C:\ given whole: absolute, no stream name, no `.` or `..` component (its full path is itself).
function OnC([string]$path) {
  if ($path -notmatch '^C:\\[A-Za-z0-9 ._()\\-]+\z' -or $path.EndsWith('\')) { throw "folder refused: $path" }
  if ([IO.Path]::GetFullPath($path) -cne $path) { throw "folder refused: $path" }
  return $path
}

function RegKey([string]$key) {
  if ($key -notmatch '^Software\\[A-Za-z0-9 ._()-]+(\\[A-Za-z0-9 ._()-]+)*\z') { throw "key refused: $key" }
  return $key
}

function OpClaim($p) {
  $folder = OnC ([string]$p.folder)
  $name = [string]$p.name
  $attempt = $name -cmatch '^(SWBPIPE-harness-exercise-)?[0-9a-f]{32}\.claim\z'
  $exercise = ($folder -cmatch $exerciseFolder) -and ($name -cmatch '^SWBPIPE-harness-exercise-[A-Za-z0-9-]{1,64}\z')
  if (-not ($attempt -or $exercise)) { throw "claim name refused: $name" }
  $content = [string]$p.content
  if ($content -cnotmatch '^swbpipe-claim v1\r\n([\x20-\x7e]*\r\n){1,8}\z' -or $content.Length -gt 512) {
    throw 'claim content refused'
  }
  [void][IO.Directory]::CreateDirectory($folder)
  try { $s = [IO.File]::Open($folder + '\' + $name, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None) }
  catch {
    $e = $_.Exception
    while ($e -and -not ($e -is [IO.IOException])) { $e = $e.InnerException }
    if ($e -and $e.HResult -eq -2147024816) { return "SWBPIPE-CLAIM v1 exists`r`n" }
    if ($e -and $e.HResult -eq -2147024864) { return "SWBPIPE-CLAIM v1 held`r`n" }
    throw
  }
  try { $b = [Text.Encoding]::ASCII.GetBytes($content); $s.Write($b, 0, $b.Length); $s.Flush($true) }
  finally { $s.Dispose() }
  return "SWBPIPE-CLAIM v1 created`r`n"
}

function OpRegValue($p, [string]$action) {
  $keyName = RegKey ([string]$p.key)
  $name = [string]$p.name
  if ($name -notmatch '^[A-Za-z0-9 ._()\\:-]{1,260}\z') { throw "value name refused" }
  $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey($keyName, $true)
  if ($null -eq $key) { [Console]::Error.Write($missing); exit 1 }
  try {
    if ($action -eq 'set') {
      $hex = [string]$p.data
      if ($hex -cnotmatch '^([0-9A-F]{2})*\z') { throw 'data refused: REG_BINARY as upper-case hex' }
      $bytes = [byte[]]::new($hex.Length / 2)
      for ($i = 0; $i -lt $bytes.Length; $i++) { $bytes[$i] = [Convert]::ToByte($hex.Substring(2 * $i, 2), 16) }
      $key.SetValue($name, $bytes, [Microsoft.Win32.RegistryValueKind]::Binary)
    } else {
      if (-not ($key.GetValueNames() -ccontains $name)) { [Console]::Error.Write($missing); exit 1 }
      if ($key.GetValueKind($name) -ne [Microsoft.Win32.RegistryValueKind]::Binary) {
        throw "only a REG_BINARY value may be deleted: $name"
      }
      $key.DeleteValue($name, $true)
    }
  } finally { $key.Close() }
  return $ok
}

function OpKey($p, [string]$action) {
  $keyName = [string]$p.key
  if ($keyName -cnotmatch $exerciseKey) { throw "only the exercise key may be created or deleted: $keyName" }
  if ($action -eq 'create') {
    $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey($keyName)
    if ($null -eq $key) { throw 'the key could not be created' }
    $key.Close()
  } else {
    if ($null -eq [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey($keyName)) { [Console]::Error.Write($missing); exit 1 }
    [Microsoft.Win32.Registry]::CurrentUser.DeleteSubKeyTree($keyName, $true)
  }
  return $ok
}

try {
  $script = FileSha $PSCommandPath
  $dataSha = 'none'
  if ($Data) {
    if ($Data -notmatch '^[0-9a-f]{64}\z') { throw 'data digest refused' }
    $dataPath = $root.TrimEnd('\') + '\_harness\data\' + $Data + '.json'
    $dataSha = FileSha $dataPath
  }
  [Console]::Out.Write("SWBPIPE-WRITE v1 script=$script data=$dataSha`r`n")
  if ($Data -and $dataSha -ne $Data) { [Console]::Error.Write('the data file differs from its digest'); exit 1 }
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
    'claim' { $answer = OpClaim $p }
    'regset' { $answer = OpRegValue $p 'set' }
    'regdelete' { $answer = OpRegValue $p 'delete' }
    'keycreate' { $answer = OpKey $p 'create' }
    'keydelete' { $answer = OpKey $p 'delete' }
    default { throw "unknown operation: $Op" }
  }
  [Console]::Out.Write($answer)
} catch { [Console]::Error.Write($_.Exception.Message); exit 1 }
exit 0
