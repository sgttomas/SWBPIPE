$ErrorActionPreference = 'Stop'
if (-not $IsWindows -or [Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() -ne 'X64' -or [Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString() -ne 'X64') { throw 'Native x64 Windows required' }
$top = (& git rev-parse --show-toplevel)
if ($LASTEXITCODE -ne 0) { throw 'Repository root unavailable' }
Set-Location $top
$sourcePath = [IO.Path]::GetFullPath($top)
$head = (& git rev-parse HEAD)
if ($LASTEXITCODE -ne 0) { throw 'Candidate resolution failed' }
function Assert-Candidate([string]$expected) {
    if (-not $expected -or $head -cne $expected) { throw 'Wrong or unspecified candidate' }
}
Assert-Candidate $env:EXPECTED_HEAD
if ((& git status --porcelain --untracked-files=all)) { throw 'Committed clean candidate required' }
$root = $PSScriptRoot
$project = Join-Path $top 'projects/chirality-piping'
$lock = Join-Path $project 'core/runner/headless/Cargo.lock'
$lockHash = '14397ed0733124d4dd8ac34ea14ec15b0274b30a0a0c2a65f2955e57b9e35e2c'
if ((Get-FileHash $lock -Algorithm SHA256).Hash.ToLowerInvariant() -ne $lockHash) { throw 'Runner lock identity mismatch' }
$rust = (& rustc -Vv) -join "`n"
if ($LASTEXITCODE -ne 0 -or $rust -notmatch '(?m)^release: 1\.97\.1$' -or $rust -notmatch '(?m)^host: x86_64-pc-windows-msvc$') { throw 'Rust 1.97.1 native MSVC required' }
Write-Host "candidate=$head rust=1.97.1 host=x86_64-pc-windows-msvc lock_sha256=$lockHash"
if (-not $env:RUNNER_TEMP) { throw 'Hosted runner temporary directory required' }
$work = Join-Path $env:RUNNER_TEMP ('runner-startup-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $work | Out-Null
$target = Join-Path $work 'fresh-runner-target'
$controller = Join-Path $work 'controller.exe'
$tests = Join-Path $work 'pe-tests.exe'
# Build output intentionally streams. No Cargo cache/target reuse or changed features.
# Official pinned standard-library source: retain relevant context live for
# independent review, rather than inferring pipe support from a package graph.
# A network/source mismatch is a blocker, never an alternate runner wrapper.
$official = @{}
foreach ($relative in @('fs.rs','sys/fs/windows.rs','sys/handle/windows.rs')) {
    $url = 'https://raw.githubusercontent.com/rust-lang/rust/1.97.1/library/std/src/' + $relative
    $response = Invoke-WebRequest -Uri $url -TimeoutSec 30
    $text = [string]$response.Content
    $official[$relative] = $text
    $sha = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($text))).ToLowerInvariant()
    Write-Host "official_rust_source=$relative sha256=$sha"
    $sourceLines = $text -split "`n"
    $hits = @(for ($i=0; $i -lt $sourceLines.Count; $i++) { if ($sourceLines[$i] -match 'fn read_to_string|ERROR_BROKEN_PIPE|FILE_TYPE_PIPE') { $i } })
    foreach ($hit in ($hits | Select-Object -First 8)) {
        foreach ($i in ([Math]::Max(0,$hit-2))..([Math]::Min($sourceLines.Count-1,$hit+12))) { Write-Host ("official_source_line={0}:{1} {2}" -f $relative,($i+1),$sourceLines[$i]) }
    }
}
if ($official['fs.rs'] -notmatch 'fn read_to_string' -or $official['sys/handle/windows.rs'] -notmatch 'ERROR_BROKEN_PIPE') { throw 'Official pinned pipe-read source needs review; compatibility not established' }
Write-Host 'START unchanged runner fresh release compile/link'
& cargo build --locked --release --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml --bin openpipestress-runner --target x86_64-pc-windows-msvc --target-dir $target 2>&1 | ForEach-Object { "$_".Replace($sourcePath,'<source>').Replace($top,'<source>').Replace($env:USERPROFILE,'<account>').Replace($env:RUNNER_TEMP,'<temp>') }
if ($LASTEXITCODE -ne 0) { throw 'Unchanged runner build failed' }
if ((Get-FileHash $lock -Algorithm SHA256).Hash.ToLowerInvariant() -ne $lockHash -or (& git status --porcelain --untracked-files=all)) { throw 'Build changed candidate inputs' }
Write-Host 'START dependency-free controller compile/link'
& rustc --edition=2024 --remap-path-prefix "$top=<source>" --target x86_64-pc-windows-msvc -C opt-level=1 (Join-Path $root 'controller.rs') -o $controller 2>&1 | ForEach-Object { "$_".Replace($sourcePath,'<source>').Replace($top,'<source>').Replace($env:USERPROFILE,'<account>').Replace($env:RUNNER_TEMP,'<temp>') }
if ($LASTEXITCODE -ne 0) { throw 'Controller compile/link failed' }
& rustc --edition=2024 --remap-path-prefix "$top=<source>" --target x86_64-pc-windows-msvc --test (Join-Path $root 'pe.rs') -o $tests 2>&1 | ForEach-Object { "$_".Replace($sourcePath,'<source>').Replace($top,'<source>').Replace($env:USERPROFILE,'<account>').Replace($env:RUNNER_TEMP,'<temp>') }
if ($LASTEXITCODE -ne 0) { throw 'PE test compile/link failed' }
$inventory = @(& $tests --list --format terse)
if ($LASTEXITCODE -ne 0 -or $inventory.Count -ne 9) { throw 'PE test inventory mismatch' }
& $tests --nocapture --test-threads=1
if ($LASTEXITCODE -ne 0) { throw 'Malformed/truncated PE controls failed' }
$runner = Join-Path $target 'x86_64-pc-windows-msvc/release/openpipestress-runner.exe'
$hash = (Get-FileHash $runner -Algorithm SHA256).Hash.ToLowerInvariant()
Write-Host "artifact=openpipestress-runner.exe sha256=$hash"
& $controller --pe $runner
if ($LASTEXITCODE -ne 0) { throw 'Actual runner PE inspection failed' }
$expected = @('solve_first','solve_repeat','validate_metadata','malformed_json','missing_payload','blocked_timeout')
$actual = @(& $controller --inventory)
if ($LASTEXITCODE -ne 0 -or ($actual -join ',') -cne ($expected -join ',')) { throw 'Missing or extra case selection' }
& $controller --supervise wrong_candidate $runner unused unused solve 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) { throw 'Wrong selection control falsely passed' }
& $controller 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) { throw 'Empty selection control falsely passed' }
# Malformed JSON is created in hosted scratch from an explicitly invented
# one-character truncation. It is not a falsely valid checked-in JSON document.
$bad = Join-Path $work 'malformed-input'
[IO.File]::WriteAllText($bad, '{', [Text.UTF8Encoding]::new($false))
function Assert-Markers([string[]]$lines, [string]$case, [bool]$blocked) {
    $markers = @('CREATED_SUSPENDED','RESUMED','INPUT_CONNECTED_PAYLOAD_WITHHELD')
    if (-not $blocked) { $markers += @('PAYLOAD_SENT_EOF','OUTPUT_COMPLETE','EXIT_OBSERVED') }
    $markers += 'JOB_DRAINED'
    $last = -1
    foreach ($marker in $markers) {
        $indices = @(); for ($i=0; $i -lt $lines.Count; $i++) { if ($lines[$i] -match "^phase=$marker case=$case( |$)") { $indices += $i } }
        if ($indices.Count -ne 1 -or $indices[0] -le $last) { throw "Missing/duplicated/out-of-order marker $marker" }
        $last = $indices[0]
    }
    if (-not (($lines -join "`n").Contains("phase=JOB_DRAINED case=$case active=0 case_exit_observed=true"))) { throw 'Cleanup not confirmed' }
    if ($blocked -and -not ($lines -contains 'BLOCKED_DESCENDANT_TIMEOUT_CONFIRMED')) { throw 'Real blocked descendant was not exercised' }
}
# A removed gate must fail the same marker checker used for actual witnesses.
$synthetic = @('CREATED_SUSPENDED','RESUMED','PAYLOAD_SENT_EOF','OUTPUT_COMPLETE','EXIT_OBSERVED') | ForEach-Object { "phase=$_ case=solve_first" }
$synthetic += 'phase=JOB_DRAINED case=solve_first active=0 case_exit_observed=true'
$caught = $false
try { Assert-Markers $synthetic 'solve_first' $false } catch { $caught = $true }
if (-not $caught) { throw 'Missing-marker negative control falsely passed' }
# Wrong candidate identity check uses the actual expected-head predicate.
$caught = $false
try { Assert-Candidate ('0' * 40) } catch { $caught = $true }
if (-not $caught) { throw 'Wrong candidate negative control falsely passed' }
Write-Host 'identity/selection/missing-marker negative controls rejected'
foreach ($case in $expected) {
    $verb = 'solve'; $payloadPath = Join-Path $root 'fixtures/solve.json'
    if ($case -eq 'validate_metadata') { $verb='validate-input'; $payloadPath=Join-Path $root 'fixtures/validate_metadata.json' }
    if ($case -eq 'missing_payload') { $payloadPath=Join-Path $root 'fixtures/missing_payload.json' }
    if ($case -eq 'malformed_json') { $payloadPath=$bad }
    $output = Join-Path $work ($case + '.output')
    Write-Host "START case=$case"
    $lines = @(& $controller --supervise $case $runner $payloadPath $output $verb | ForEach-Object { Write-Host $_; $_ })
    if ($LASTEXITCODE -ne 0) { throw "Case failed: $case" }
    Assert-Markers $lines $case ($case -eq 'blocked_timeout')
    if ($case -eq 'blocked_timeout') { continue }
    if ((Get-Item $output).Length -gt 1048576) { throw 'Output limit exceeded' }
    $controlled = Get-Content $output -Raw | ConvertFrom-Json -Depth 100
    if ($controlled.blocked -ne $false -or $null -eq $controlled.payload) { throw 'Unexpected redaction outcome' }
    $p = $controlled.payload
    if ($p.artifact -cne 'openpipestress.headless_runner_cli_output' -or $p.schema_version -cne '1.0.0' -or $p.command -cne $verb -or $p.policy.policy_ref -cne 'DEC-065') { throw 'Wrong output envelope' }
    if ($case -in @('solve_first','solve_repeat')) {
        if ($p.operation -cne 'solve' -or $null -eq $p.runner_result -or $p.mechanics_envelope.status.mechanics -cne 'MECHANICS_SOLVED' -or @($p.diagnostics).Count -ne 0) { throw 'Supported invented sparse solve did not succeed' }
    } elseif ($case -eq 'validate_metadata') {
        if ($p.operation -cne 'validate_input' -or $null -eq $p.request_validation -or @($p.request_validation.diagnostics).Count -ne 0 -or @($p.diagnostics).Count -ne 0 -or $null -ne $p.runner_result -or $null -ne $p.mechanics_envelope) { throw 'Metadata-only validation contract mismatch' }
    } else {
        $code = if ($case -eq 'malformed_json') { 'HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID' } else { 'HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING' }
        if (@($p.diagnostics).Count -ne 1 -or $p.diagnostics[0].code -cne $code -or $null -ne $p.runner_result) { throw 'Typed refusal contract mismatch' }
    }
    Write-Host "END case=$case output_contract=pass"
}
if ((Get-FileHash $runner -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash) { throw 'Runner artifact changed during cases' }
Write-Host 'Finite startup diagnostic complete; first/repeated launches are not a cold-cache or performance proof.'
# No raw JSON, PE, compiler logs or absolute paths are uploaded by this script.
Remove-Item -Recurse -Force $work
