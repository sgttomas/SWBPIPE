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
$p7 = Get-Content (Join-Path $project 'tools/release/provenance_scope.json') -Raw | ConvertFrom-Json -Depth 100
$fixtureFiles = @(Get-ChildItem (Join-Path $root 'fixtures/*.json') | Sort-Object Name)
if (($fixtureFiles.Name -join ',') -cne 'missing_payload.json,solve.json,validate_metadata.json') { throw 'Changed approved fixture inventory' }
foreach ($file in $fixtureFiles) {
    $relative = 'validation/platform/windows_runner_startup_probe/fixtures/' + $file.Name
    $entry = @($p7.members | Where-Object { $_.path -ceq $relative })
    $record = Get-Content $file.FullName -Raw | ConvertFrom-Json -Depth 100
    if ($entry.Count -ne 1 -or $entry[0].sha256 -cne (Get-FileHash $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -or $record.request.provenance.redistribution_status -cne 'invented_non_engineering_example' -or $record.request.provenance.review_status -cne 'pending') { throw 'Approved invented P7 fixture bytes/labels differ from candidate' }
}
Write-Host 'P7 committed fixture hashes/labels checked; solve fixture remains inactive'
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
foreach ($relative in @('fs.rs','sys/fs/windows.rs','sys/pal/windows/handle.rs')) {
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
if ($official['fs.rs'] -notmatch 'fn read_to_string' -or $official['sys/pal/windows/handle.rs'] -notmatch 'ERROR_BROKEN_PIPE') { throw 'Official pinned pipe-read source needs review; compatibility not established' }
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
$expected = @('validate_metadata','malformed_json','missing_payload','blocked_timeout')
$blockedExpected = @('solve_first=BLOCKED_UNVERIFIED_CAPTURE_CAP','solve_repeat=BLOCKED_UNVERIFIED_CAPTURE_CAP','ten_live_solve_mutations=UNVERIFIED')
$blockedActual = @(& $controller --blocked-inventory)
if ($LASTEXITCODE -ne 0 -or ($blockedActual -join ',') -cne ($blockedExpected -join ',')) { throw 'Blocked solve inventory mismatch' }
$blockedActual | ForEach-Object { Write-Host "NOT_EXECUTED $_" }
Write-Host 'COVERAGE=PARTIAL metadata/typed-error/blocked-input/cleanup; solve=BLOCKED_UNVERIFIED'

$actual = @(& $controller --inventory)
if ($LASTEXITCODE -ne 0 -or ($actual -join ',') -cne ($expected -join ',')) { throw 'Missing or extra case selection' }
& $controller --supervise wrong_candidate $runner unused unused validate-input 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) { throw 'Wrong selection control falsely passed' }
& $controller 2>&1 | Out-Null
if ($LASTEXITCODE -eq 0) { throw 'Empty selection control falsely passed' }
# The closed input guard must reject another approved fixture before spawning
# any runner, while the independent observer still confirms case cleanup.
$substitution = @(& $controller --supervise validate_metadata $runner (Join-Path $root 'fixtures/missing_payload.json') (Join-Path $work 'rejected.output') validate-input 2>&1 | ForEach-Object { Write-Host $_; $_ })
if ($LASTEXITCODE -eq 0 -or -not (($substitution -join "`n").Contains('phase=JOB_DRAINED case=validate_metadata active=0 case_exit_observed=true')) -or (($substitution -join "`n").Contains('phase=CREATED_SUSPENDED'))) { throw 'Substitute-input guard or cleanup falsely passed' }
Write-Host 'INPUT_SUBSTITUTION_REJECTED cleanup_active=0 runner_created=false'
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
$synthetic = @('CREATED_SUSPENDED','RESUMED','PAYLOAD_SENT_EOF','OUTPUT_COMPLETE','EXIT_OBSERVED') | ForEach-Object { "phase=$_ case=validate_metadata" }
$synthetic += 'phase=JOB_DRAINED case=validate_metadata active=0 case_exit_observed=true'
$caught = $false
try { Assert-Markers $synthetic 'validate_metadata' $false } catch { $caught = $true }
if (-not $caught) { throw 'Missing-marker negative control falsely passed' }
# Wrong candidate identity check uses the actual expected-head predicate.
$caught = $false
try { Assert-Candidate ('0' * 40) } catch { $caught = $true }
if (-not $caught) { throw 'Wrong candidate negative control falsely passed' }
Write-Host 'identity/selection/missing-marker negative controls rejected'
# Source-bound small outputs only: metadata carries no runner/mechanics result;
# each refusal carries one fixed-shape diagnostic and no mechanics rows. At most
# 40 scalar leaves, paths <128 ASCII bytes and fixed messages <512 bytes give a
# conservative <128 KiB serialized control/audit bound, below the unchanged 1 MiB
# runtime guard. This bound does not apply to a solve response.
function Assert-Object($value, [string]$label) {
    if ($value -isnot [pscustomobject]) { throw "Missing or malformed $label object" }
}
function Assert-EmptyDiagnostics($owner, [string]$label) {
    Assert-Object $owner $label
    if ($null -eq $owner.PSObject.Properties['diagnostics'] -or $owner.diagnostics -isnot [array] -or @($owner.diagnostics).Count -ne 0) { throw "Missing or non-clean $label diagnostics" }
}
function Get-LeafPaths($value, [string]$path) {
    if ($value -is [pscustomobject]) {
        foreach ($property in $value.PSObject.Properties) { Get-LeafPaths $property.Value ($path + '.' + $property.Name) }
    } elseif ($value -is [array]) {
        for ($index=0; $index -lt $value.Count; $index++) { Get-LeafPaths $value[$index] ($path + '[' + $index + ']') }
    } else { $path }
}
function Assert-InputBinding($context, [string]$case) {
    Assert-Object $context 'bound fixture'
    Assert-Object $context.request 'request'
    $operation = if ($case -eq 'validate_metadata') { 'validate_input' } else { 'solve' }
    $r = $context.request
    if ($r.request_id -cne 'invented-startup-diagnostic' -or $r.operation -cne $operation -or $r.operation_ref.ref_type -cne 'api_operation' -or $r.operation_ref.ref_id -cne $operation) { throw 'Request/operation reference binding mismatch' }
    if ($r.project_ref.ref_type -cne 'project' -or $r.project_ref.ref_id -cne 'project:precision-connected-fixture' -or $r.model_ref.ref_type -cne 'model' -or $r.model_ref.ref_id -cne 'project:precision-connected-fixture') { throw 'Project/model reference binding mismatch' }
    if ($r.provenance.redistribution_status -cne 'invented_non_engineering_example' -or $r.provenance.review_status -cne 'pending' -or $null -ne $context.PSObject.Properties['solve']) { throw 'Diagnostic fixture must be genuinely invented metadata without a solve payload' }
}
function Assert-Controlled($controlled) {
    Assert-Object $controlled 'controlled export'
    if ($controlled.blocked -isnot [bool] -or $controlled.blocked -ne $false -or $controlled.decisions -isnot [array] -or $controlled.findings -isnot [array]) { throw 'Malformed or unexpectedly blocked privacy outcome' }
    Assert-Object $controlled.payload 'controlled payload'
    Assert-Object $controlled.summary 'privacy summary'
    $leaves = @(Get-LeafPaths $controlled.payload '$' | Sort-Object)
    $decisions = @($controlled.decisions.path | Sort-Object)
    if ($leaves.Count -eq 0 -or ($leaves -join "`n") -cne ($decisions -join "`n")) { throw 'Missing or extra privacy audit decision paths' }
    $warningPaths = @($controlled.decisions | Where-Object { $_.action -ceq 'warning_only' } | ForEach-Object { $_.path } | Sort-Object)
    $findingPaths = @($controlled.findings.path | Sort-Object)
    if (($warningPaths -join "`n") -cne ($findingPaths -join "`n")) { throw 'Privacy warnings were removed or fabricated' }
    foreach ($decision in $controlled.decisions) {
        if ($decision.action -cnotin @('include','warning_only') -or [string]::IsNullOrWhiteSpace($decision.reason_code)) { throw 'Unexpected privacy decision' }
    }
    foreach ($finding in $controlled.findings) {
        if ($finding.severity -cne 'WARNING' -or $finding.action -cne 'warning_only' -or [string]::IsNullOrWhiteSpace($finding.code)) { throw 'Unexpected blocking or malformed privacy finding' }
    }
    if ($controlled.summary.decision_count -ne $decisions.Count -or $controlled.summary.finding_count -ne $findingPaths.Count -or $controlled.summary.warning_count -ne $findingPaths.Count -or $controlled.summary.blocking_count -ne 0 -or $controlled.summary.redacted_count -ne 0 -or $controlled.summary.omitted_count -ne 0 -or $controlled.summary.cloud_transmission_attempted -ne $false -or $controlled.summary.professional_claims_made -ne $false) { throw 'Privacy summary disagrees with retained audit' }
}
function Assert-DiagnosticOutput($controlled, [string]$case, $context, [int]$observedExit) {
    Assert-Controlled $controlled
    $p = $controlled.payload
    $verb = if ($case -eq 'validate_metadata') { 'validate-input' } else { 'solve' }
    $operation = if ($case -eq 'validate_metadata') { 'validate_input' } else { 'solve' }
    $expectedExit = switch ($case) { 'validate_metadata' { 0 } 'malformed_json' { 2 } 'missing_payload' { 1 } default { throw 'Not an output-bearing diagnostic case' } }
    if ($observedExit -ne $expectedExit -or $p.artifact -cne 'openpipestress.headless_runner_cli_output' -or $p.schema_version -cne '1.0.0' -or $p.deliverable_id -cne 'DEL-10-05' -or $p.package_id -cne 'PKG-10' -or $p.command -cne $verb -or $p.operation -cne $operation) { throw 'Wrong diagnostic identity, operation or observed exit' }
    Assert-Object $p.policy 'CLI policy'
    if ($p.policy.policy_ref -cne 'DEC-065' -or $p.policy.binary -cne 'openpipestress-runner') { throw 'Wrong existing CLI policy identity' }
    foreach ($key in @('stdout_default','explicit_output_path_only','local_foreground_process','network_allowed','telemetry_allowed','daemon_allowed','hidden_filesystem_mutation_allowed','direct_sql_access_allowed')) {
        $wanted = $key -cin @('stdout_default','explicit_output_path_only','local_foreground_process')
        if ($p.policy.$key -isnot [bool] -or $p.policy.$key -ne $wanted) { throw 'Wrong existing CLI policy flag' }
    }
    foreach ($key in @('runner_result','mechanics_envelope','result_validation','suite_run')) {
        if ($null -eq $p.PSObject.Properties[$key] -or $null -ne $p.$key) { throw 'Metadata/refusal case must not claim a completed job or mechanics result' }
    }
    if ($p.diagnostics -isnot [array]) { throw 'Missing diagnostics array' }
    if ($case -eq 'validate_metadata') {
        Assert-InputBinding $context $case
        Assert-EmptyDiagnostics $p.request_validation 'actual request metadata validation'
        if ($p.diagnostics.Count -ne 0) { throw 'Unexpected metadata diagnostic' }
        return
    }
    if ($p.diagnostics.Count -ne 1) { throw 'Expected exactly one typed diagnostic' }
    $d = $p.diagnostics[0]
    $expectedCode = if ($case -eq 'malformed_json') { 'HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID' } else { 'HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING' }
    if ($d.code -cne $expectedCode -or $d.class -cne 'RUNNER_BLOCKING' -or $d.severity -cne 'blocking' -or $d.source.ref_type -cne 'headless_runner' -or $d.source.ref_id -cne 'DEL-10-05') { throw 'Wrong typed refusal code, severity, class or source' }
    if ($case -eq 'missing_payload') {
        Assert-InputBinding $context $case
        Assert-EmptyDiagnostics $p.request_validation 'pre-refusal metadata validation'
        if ($d.affected_object.ref_type -cne 'runner_request' -or $d.affected_object.ref_id -cne $context.request.request_id -or $d.message -cne 'solve command requires solve.preview_model in the schema-first input JSON') { throw 'Wrong missing-payload refusal binding' }
    } else {
        if ($null -ne $p.request_validation -or $d.affected_object.ref_type -cne 'cli_input' -or $d.affected_object.ref_id -cne 'stdin-or-file' -or -not $d.message.StartsWith('input must be a JSON object with request metadata: ')) { throw 'Malformed input unexpectedly reached request validation or has wrong typed binding' }
    }
    if ($d.provenance.redistribution_status -cne 'invented_non_engineering_example' -or $d.provenance.source_location -cne 'core/runner/headless') { throw 'Wrong existing diagnostic provenance' }
    $intentWarnings = @($controlled.findings | Where-Object { $_.path -ceq '$.diagnostics[0].provenance.source_location' -and $_.code -ceq 'PRIVATE_LOCAL_ALLOWED_WITH_WARNING' })
    if ($intentWarnings.Count -ne 1) { throw 'Explicit diagnostic intent warning was not preserved' }
}
function Assert-DiagnosticMutationControls($captured, [string]$case, $context, [int]$observedExit) {
    $mutations = @('empty','malformed','missing_payload','wrong_operation','wrong_exit','wrong_runner_status','wrong_validation','wrong_refs','typed_diagnostics','missing_audit','stripped_warnings','blocked_privacy')
    foreach ($kind in $mutations) {
        $changed = $captured | ConvertTo-Json -Depth 100 -Compress | ConvertFrom-Json -Depth 100
        $bound = if ($null -eq $context) { $null } else { $context | ConvertTo-Json -Depth 100 -Compress | ConvertFrom-Json -Depth 100 }
        $changedExit = $observedExit
        switch ($kind) {
            'empty' { $changed=[pscustomobject]@{} }
            'malformed' { $changed='not a controlled object' }
            'missing_payload' { $changed.payload=$null }
            'wrong_operation' { $changed.payload.operation='run_benchmark' }
            'wrong_exit' { $changedExit=99 }
            'wrong_runner_status' { $changed.payload.runner_result=[pscustomobject]@{job=[pscustomobject]@{state='COMPLETED'}} }
            'wrong_validation' { $changed.payload.request_validation=if ($case -eq 'malformed_json') { [pscustomobject]@{diagnostics=@()} } else { $null } }
            'wrong_refs' { if ($null -ne $bound) { $bound.request.operation_ref.ref_id='wrong-reference' } else { $changed.payload.diagnostics[0].affected_object.ref_id='wrong-reference' } }
            'typed_diagnostics' { if ($case -eq 'validate_metadata') { $changed.payload.diagnostics=@([pscustomobject]@{code='INVENTED_BAD_DIAGNOSTIC'}) } else { $changed.payload.diagnostics[0].code='INVENTED_WRONG_CODE' } }
            'missing_audit' { $changed.decisions=@(); $changed.summary.decision_count=0 }
            'stripped_warnings' { $changed.findings=@(); $changed.summary.finding_count=0; $changed.summary.warning_count=0 }
            'blocked_privacy' { $changed.blocked=$true }
            default { throw 'Unknown diagnostic mutation selection' }
        }
        $rejected=$false
        try { Assert-DiagnosticOutput $changed $case $bound $changedExit } catch { $rejected=$true }
        if (-not $rejected) { throw "Diagnostic mutation falsely accepted: case=$case mutation=$kind" }
        Write-Host "diagnostic_mutation=$kind case=$case rejected=true"
    }
    Write-Host "DIAGNOSTIC_OUTPUT_NEGATIVE_CONTROLS case=$case count=$($mutations.Count)"
}
$executed = 0
$mutationCount = 0
foreach ($case in $expected) {
    $verb = if ($case -in @('validate_metadata','blocked_timeout')) { 'validate-input' } else { 'solve' }
    $payloadPath = if ($case -eq 'missing_payload') { Join-Path $root 'fixtures/missing_payload.json' } elseif ($case -eq 'malformed_json') { $bad } else { Join-Path $root 'fixtures/validate_metadata.json' }
    $context = if ($case -eq 'malformed_json') { $null } else { Get-Content $payloadPath -Raw | ConvertFrom-Json -Depth 100 }
    if ($case -eq 'validate_metadata' -or $case -eq 'missing_payload') { Assert-InputBinding $context $case }
    $output = Join-Path $work ($case + '.output')
    Write-Host "START PARTIAL case=$case"
    $lines = @(& $controller --supervise $case $runner $payloadPath $output $verb | ForEach-Object { Write-Host $_; $_ })
    if ($LASTEXITCODE -ne 0) { throw "Partial diagnostic case failed: $case" }
    Assert-Markers $lines $case ($case -eq 'blocked_timeout')
    if ($case -ne 'blocked_timeout') {
        if ((Get-Item $output).Length -gt 1048576) { throw 'Output cap exceeded' }
        $exitMatches = @($lines | Where-Object { $_ -cmatch "^runner_exit_code=([0-9]+) case=$case`$" })
        if ($exitMatches.Count -ne 1 -or $exitMatches[0] -cnotmatch "^runner_exit_code=([0-9]+) case=$case`$") { throw 'Missing or ambiguous observed exit evidence' }
        $observedExit = [int]$Matches[1]
        $captured = Get-Content $output -Raw | ConvertFrom-Json -Depth 100
        Assert-DiagnosticOutput $captured $case $context $observedExit
        Assert-DiagnosticMutationControls $captured $case $context $observedExit
        $mutationCount += 12
    }
    $executed++
    Write-Host "END PARTIAL case=$case statement=pass"
}
if ($executed -ne 4 -or $mutationCount -ne 36) { throw 'Partial execution/mutation count mismatch' }
if ((Get-FileHash $runner -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash) { throw 'Runner artifact changed during cases' }
Write-Host "PARTIAL_DIAGNOSTICS executed=$executed captured_output_mutations=$mutationCount solve=BLOCKED_UNVERIFIED live_solve_mutations=UNVERIFIED"
# No raw JSON, PE, compiler logs or absolute paths are uploaded by this script.
Remove-Item -Recurse -Force $work
