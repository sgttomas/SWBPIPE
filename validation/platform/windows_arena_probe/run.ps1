$ErrorActionPreference = 'Stop'
$root = $PSScriptRoot
if (-not $IsWindows -or [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() -ne 'X64' -or [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString() -ne 'X64') { throw 'Native x64 Windows and x64 PowerShell required' }
$actual = git rev-parse HEAD
if ($LASTEXITCODE -ne 0 -or -not $env:EXPECTED_HEAD -or $actual -ne $env:EXPECTED_HEAD) { throw 'Wrong or unspecified probe candidate' }
Write-Host "candidate=$actual native_os=X64 process=X64"
if (-not $env:CARGO_TARGET_DIR) { throw 'Set CARGO_TARGET_DIR to an isolated temporary directory' }
$manifest = Join-Path $root 'Cargo.toml'
New-Item -ItemType Directory -Force -Path $env:CARGO_TARGET_DIR | Out-Null
$clock = [Diagnostics.Stopwatch]::StartNew()
Write-Host 'START compile-link'
& cargo test --locked --manifest-path $manifest --target x86_64-pc-windows-msvc --no-run 2>&1 | Tee-Object -FilePath (Join-Path $env:CARGO_TARGET_DIR 'compile.log')
if ($LASTEXITCODE -ne 0) { throw "compile-link failed: $LASTEXITCODE" }
Write-Host "END compile-link elapsed=$($clock.Elapsed)"
$bins = @(Get-ChildItem (Join-Path $env:CARGO_TARGET_DIR 'x86_64-pc-windows-msvc/debug/deps/windows_arena_probe-*.exe'))
if ($bins.Count -ne 1) { throw "Expected one test executable, found $($bins.Count)" }
$exe = $bins[0].FullName
$inventory = @(& $exe --list --format terse)
if ($LASTEXITCODE -ne 0) { throw 'case inventory failed' }
$cases = @(
    'arena::tests::native_arena',
    'ledger::tests::ledgers_and_cancellation',
    'ledger::tests::retry_classifier',
    'worker::native_recovery',
    'worker::native_persistent_headroom',
    'worker::native_nonheadroom_no_retry',
    'worker::native_cancel_and_reset',
    'worker::two_native_workers',
    'worker::blocked_child_termination',
    'worker::child_failure_negative_control',
    'worker::native_separate_deadline',
    'worker::native_commit_retry_deadline'
)
foreach ($case in $cases) { if (($inventory | Where-Object { $_ -eq "${case}: test" }).Count -ne 1) { throw "missing case $case" } }
if (($inventory | Where-Object { $_ -eq 'worker::child_entry: test' }).Count -ne 1 -or $inventory.Count -ne 13) { throw 'unexpected case inventory' }
Write-Host "Inventory: $($inventory -join ', ')"
function Run-Case([string]$case, [bool]$expectOne) {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    Write-Host "START execute $case"
    $out = Join-Path $env:CARGO_TARGET_DIR 'case.stdout'
    $err = Join-Path $env:CARGO_TARGET_DIR 'case.stderr'
    $log = Join-Path $env:CARGO_TARGET_DIR 'cases.log'
    $p = Start-Process -FilePath $exe -ArgumentList @('--exact', $case, '--nocapture', '--test-threads=1') -NoNewWindow -PassThru -RedirectStandardOutput $out -RedirectStandardError $err
    $positions = @(0,0); $lines = [Collections.Generic.List[string]]::new()
    try {
        $finished = $false
        do {
            foreach ($i in 0..1) {
                $path = @($out,$err)[$i]
                if (Test-Path $path) {
                    $stream = [IO.File]::Open($path, 'Open', 'Read', 'ReadWrite')
                    try {
                        $stream.Position = $positions[$i]; $reader = [IO.StreamReader]::new($stream)
                        while (-not $reader.EndOfStream) { $line = $reader.ReadLine(); $lines.Add($line); $line | Tee-Object -FilePath $log -Append }
                        $positions[$i] = $stream.Position
                    } finally { $stream.Dispose() }
                }
            }
            $p.Refresh()
            if ($timer.Elapsed.TotalSeconds -gt 30 -and -not $p.HasExited) { $p.Kill(); if (-not $p.WaitForExit(5000)) { throw "case cleanup not confirmed: $case" }; throw "case timeout: $case" }
            if ($p.HasExited) { if ($finished) { break }; $finished = $true }
            else { Start-Sleep -Milliseconds 100 }
        } while ($true)
        $p.WaitForExit()
        # Read the final complete output once for verdict; live output above is streamed.
        $final = (Get-Content $out -Raw) + (Get-Content $err -Raw)
        if ($p.ExitCode -ne 0) { throw "case failed: $case ($($p.ExitCode))" }
        $one = [regex]::Matches($final, '(?m)^test result: ok\. 1 passed; 0 failed; 0 ignored;').Count -eq 1
        if ($expectOne -and -not $one) { throw "empty, ignored, or failed selection: $case" }
        if (-not $expectOne -and $one) { throw 'negative selection control unexpectedly passed' }
        Write-Host "END execute $case elapsed=$($timer.Elapsed)"
    } finally {
        if (-not $p.HasExited) { $p.Kill(); if (-not $p.WaitForExit(5000)) { throw "case cleanup not confirmed: $case" } }
        Write-Host "END cleanup $case elapsed=$($timer.Elapsed)"
    }
}
Run-Case 'missing::negative_control' $false
foreach ($case in $cases) { Run-Case $case $true }

Write-Host "Completed all $($cases.Count) required cases plus missing-selection negative control; no production bound established."
