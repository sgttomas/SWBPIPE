param([string]$ExpectedCandidate = '')
$ErrorActionPreference = 'Stop'
if (-not $IsWindows -or [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() -ne 'X64' -or [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture.ToString() -ne 'X64') {
    throw 'The fixture requires native x64 Windows and x64 PowerShell; a host crosscheck is not execution.'
}
Push-Location $PSScriptRoot
try {
    $candidate = git rev-parse HEAD
    if ($LASTEXITCODE -ne 0 -or ($ExpectedCandidate -and $candidate -ne $ExpectedCandidate)) { throw 'Wrong candidate checkout' }
    $identity = @(rustup run 1.97.1 rustc -vV)
    if ($LASTEXITCODE -ne 0 -or $identity -notcontains 'host: x86_64-pc-windows-msvc' -or $identity -notcontains 'release: 1.97.1') { throw 'Wrong Rust host or version' }
    Write-Host "candidate=$candidate; native_os=X64; process=X64"
    $identity
    $cases = @(
        'tests::current_ack_preferred_and_stale_ack',
        'tests::transient_samples_and_release_wakeup_resample',
        'tests::no_ack_or_late_ack_cannot_extend_the_absolute_window',
        'tests::cancellation_dominates_late_ack_and_resume',
        'windows_accounting::mapping_commit_touch_and_release_are_distinct_observations',
        'native_fixture::native_grow_blocked_control_and_cancellation',
        'native_fixture::native_grow_grant_only_allocates_after_current_resume',
        'native_fixture::native_grow_waits_for_transient_or_persistent_headroom',
        'native_fixture::native_cancel_reset_across_worker_phases',
        'native_fixture::native_admission_fresh_samples_release_and_absolute_deadline',
        'native_fixture::native_uncheckpointed_work_fails_quiescence_but_supervisor_reaches_deadline',
        'native_fixture::native_parent_death_kills_parked_worker_before_lease_release',
        'native_fixture::native_retained_job_handle_requires_external_cleanup'
    )
    if ($cases.Count -ne 13) { throw 'Native worker case inventory is incomplete' }
    Write-Host 'phase=compile-link-enumerate'
    rustup run 1.97.1 cargo test --locked --target x86_64-pc-windows-msvc --lib -- --list 2>&1 | Tee-Object -Variable listing
    if ($LASTEXITCODE -ne 0) { throw 'Cannot enumerate the fixture cases' }
    foreach ($case in $cases) {
        if (@($listing | Where-Object { "$_" -eq "${case}: test" }).Count -ne 1) { throw "Missing or ambiguous case: $case" }
        $started = [System.Diagnostics.Stopwatch]::StartNew()
        Write-Host "phase=native-case-start case=$case"
        rustup run 1.97.1 cargo test --locked --target x86_64-pc-windows-msvc --lib $case -- --exact --nocapture --test-threads=1 2>&1 | Tee-Object -Variable output
        $status = $LASTEXITCODE
        Write-Host "phase=native-case-return case=$case elapsed_seconds=$($started.Elapsed.TotalSeconds) exit=$status"
        if ($status -ne 0) { throw "Fixture case failed: $case" }
        $passed = @($output | Where-Object { "$_" -match '^test result: ok[.] 1 passed; 0 failed; 0 ignored;' })
        if ($passed.Count -ne 1) { throw "Case did not execute exactly once and pass: $case" }
    }
    Write-Host "All $($cases.Count) required fixture cases executed; production coverage remains unestablished."
}
finally { Pop-Location }
