from pathlib import Path
import datetime,hashlib,json,sys
root=Path(sys.argv[1]).resolve()
assert "instances/NATIVE/EARLY_NATIVE_V1" in str(root)
now=datetime.datetime.now(datetime.timezone.utc).isoformat()
result={
 "schema":"early-native-result-v1",
 "sealed_at":now,
 "status":"BLOCKED_BY_PERSISTED_MODEL_ENVELOPE_INTEGRITY",
 "candidate_commit_declared":"6bb26b118fc038c451b97d3b86e3ae27d8ba8e91",
 "brief":{"path":"projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/briefs/EARLY_NATIVE_EXECUTION_V1.md","sha256":"45f8c72365c0cd78d3cc805440635ff8bb080a659b23d763991b68669af0a5bd"},
 "source_binding":{"checkpoint_sha256":"8c2c7154aaf150f8d0db5eed481a9c1d1a93d9a2501511113fc45d5539a94d6b","file_count":64,"before_build":"PASS","after_build":"PASS","after_gui":"PASS"},
 "build":{"exit_code":0,"bound_executable":"/private/tmp/piping-foundation-native-early-20260914/bound/OpenPipeStress Foundation Early 20260914.app/Contents/MacOS/openpipestress-desktop","executable_sha256":"9fe4ba364c7ef2db9e503bb495ece904cec92d7c6fe5b727131070b783ef0459","bundle_tree_sha256":"b049ec1b8cb3bba9f39e2498f51be6611338c59704b560ac18a1e8a4f5931fa7","dist_tree_sha256":"c285b00972a8b9101f4f2e42837d1db2221d12e43f24c43ebc7eaac17a926f83","bundle_identifier":"org.openpipestress.foundation-early-20260914"},
 "passing_observations":[
  "Fresh exact-candidate Tauri build completed offline and the preserved bound application was executed.",
  "First actual native backend solve established Current with model identity match, exact in-session input-manifest hash, MECHANICS_SOLVED and 830 rows.",
  "Saved analysis envelope is strict schema version 0.2.0.",
  "Independent checked authority matched the full-record self-exclusion checksum, received-result checksum and all 830 result-row checksums.",
  "Normal GUI quit removed PID 48752; same exact bundle/store relaunched as PID 50089.",
  "Restored analysis was correctly designated Historical; Current overlays, rule checks, comparisons and report readiness remained unavailable, and the missing manifest payload was not invented.",
  "Unchanged native GUI save preserved all 11 model/result/analysis/hash/attachment fields exactly; only store update metadata changed.",
  "Fresh actual native solve re-established Current against a new in-session input manifest and reproduced the mechanics content exactly.",
  "Fresh solve/save produced matching persisted model and project-envelope hashes.",
  "Final normal GUI quit removed PID 50089 and no exact candidate process remained."
 ],
 "blocker":{"finding_id":"EARLY-NATIVE-INTEGRITY-001","evidence":"validation/EARLY_NATIVE_INTEGRITY_FINDING_V1.json","summary":"The first native save persisted a migrated model while retaining pre-migration/in-session model and project-envelope hash claims. Existing rfc8785_jcs WASM recomputation proves both Historical mismatches are genuine.","model_hash":{"stored":"sha256:b2cec8ee447c44fa7efbdf43c0970a5c690b8c63e24577a105ce12e7f613ccc3","recomputed":"sha256:405db1a2316c1a937c308a91966f72f22de4018a156bb13dffa7160c67e9d3a2"},"project_envelope_hash":{"stored":"sha256:555ace20b7db1987bf9a887617988b3dfa57b1766a73f1892bf953b8c31101b8","recomputed":"sha256:c995d111b094445aa417b4db23410ec0c47d1ad6ef0bb027e69ab375f6dd7529"},"ui_codes":["HISTORICAL_MODEL_HASH_MISMATCH","HISTORICAL_ENVELOPE_HASH_MISMATCH"],"acceptance_effect":"The first reopened Historical record fails the frozen requirement that stored bytes and hashes remain verifiable/readable. Fresh recovery does not close the original persisted integrity defect.","repair_attempted":False},
 "process":{"launch_1_pid":48752,"launch_2_pid":50089,"new_identity":True,"final_process_gone":True},
 "store":{"isolated_path":"/Users/ryan/Library/Application Support/org.openpipestress.foundation-early-20260914/openpipestress-projects.sqlite3","synthetic_only":True,"snapshots":["store/after_first_save.sqlite3","store/after_unchanged_save.sqlite3","store/after_fresh_solve_save.sqlite3"]},
 "limitations":["This early run does not replace the final authored 350 N to 500 N GUI witness, final stress-neutral deliveries, complete candidate review, accepted legacy fixture backcheck or registered DEC-025."],
 "effects":{"product_source_modified":False,"git_action":False,"network_action":False,"private_or_user_model_used":False}
}
(root/"EARLY_NATIVE_RESULT_V1.json").write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
release={
 "schema":"native-build-gui-lease-release-v1",
 "released_at":now,
 "lease":"exclusive native/WASM/desktop build and native GUI surfaces",
 "state":"RELEASED",
 "result_status":result["status"],
 "final_candidate_process_gone":True,
 "final_process_evidence":"process/final_quit.json",
 "bound_application_preserved":result["build"]["bound_executable"].rsplit("/Contents/",1)[0],
 "statement":"No further build or GUI action will be performed for this checkpoint. Root may acquire the native/build/GUI lease for repair or backcheck."
}
(root/"LEASE_RELEASE.json").write_text(json.dumps(release,indent=2,sort_keys=True)+"\n")
excluded={"EVIDENCE_MANIFEST.json","SEALED_RESULT.json"}
files=[]
for path in sorted(root.rglob("*")):
    if not path.is_file() or path.name in excluded: continue
    data=path.read_bytes(); files.append({"path":str(path.relative_to(root)),"bytes":len(data),"sha256":hashlib.sha256(data).hexdigest()})
lines="".join(f'{x["path"]}\0{x["bytes"]}\0{x["sha256"]}\n' for x in files).encode()
manifest={"schema":"early-native-evidence-manifest-v1","created_at":now,"file_count":len(files),"tree_sha256":hashlib.sha256(lines).hexdigest(),"excludes":sorted(excluded),"files":files}
(root/"EVIDENCE_MANIFEST.json").write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n")
seal={
 "schema":"early-native-seal-v1",
 "sealed_at":now,
 "status":result["status"],
 "candidate_commit_declared":result["candidate_commit_declared"],
 "result_sha256":hashlib.sha256((root/"EARLY_NATIVE_RESULT_V1.json").read_bytes()).hexdigest(),
 "lease_release_sha256":hashlib.sha256((root/"LEASE_RELEASE.json").read_bytes()).hexdigest(),
 "evidence_manifest_sha256":hashlib.sha256((root/"EVIDENCE_MANIFEST.json").read_bytes()).hexdigest(),
 "evidence_tree_sha256":manifest["tree_sha256"],
 "blocker_sha256":hashlib.sha256((root/"validation/EARLY_NATIVE_INTEGRITY_FINDING_V1.json").read_bytes()).hexdigest(),
 "lease_released":True
}
(root/"SEALED_RESULT.json").write_text(json.dumps(seal,indent=2,sort_keys=True)+"\n")
print(json.dumps(seal,sort_keys=True))

