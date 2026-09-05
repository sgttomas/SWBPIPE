from pathlib import Path
import subprocess,json,hashlib,datetime,plistlib,time,platform
root=Path(subprocess.check_output(["git","rev-parse","--show-toplevel"],text=True).strip())
project=root/"projects/chirality-piping"
out=Path(__file__).resolve().parent
before=json.loads((out/"SOURCE_BEFORE.json").read_text())
build=json.loads((out/"BUILD_RESULT.json").read_text())
assert build["exit_code"]==0
changed=[p for p,h in before["inputs"].items() if not (root/p).is_file() or hashlib.sha256((root/p).read_bytes()).hexdigest()!=h]
assert not changed,changed
bundle=project/"apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app"
assert bundle.is_dir(),bundle
info=plistlib.loads((bundle/"Contents/Info.plist").read_bytes())
binary=bundle/"Contents/MacOS"/info["CFBundleExecutable"]
assets={}
for label,base in [("bundle",bundle),("frontend",project/"apps/desktop/dist"),("operation_wasm",project/"apps/desktop/public/wasm-engine"),("self_weight_wasm",project/"apps/desktop/public/self-weight-engine")]:
 files={}
 for p in sorted(base.rglob("*")):
  if p.is_symlink():files[str(p.relative_to(base))]={"symlink":str(p.readlink())}
  elif p.is_file():files[str(p.relative_to(base))]=hashlib.sha256(p.read_bytes()).hexdigest()
 assets[label]={"path":str(base),"files":files,"inventory_sha256":hashlib.sha256(json.dumps(files,sort_keys=True,separators=(",",":")).encode()).hexdigest()}
cmd=[str(binary),"--self-test-saved-edited-load"]
started=time.monotonic();run=subprocess.run(cmd,capture_output=True,text=True)
(out/"SELF_TEST.stdout.json").write_text(run.stdout)
(out/"SELF_TEST.stderr.log").write_text(run.stderr)
signature=subprocess.run(["codesign","-dv","--verbose=4",str(bundle)],capture_output=True,text=True)
(out/"CODESIGN_INSPECT.log").write_text(signature.stdout+signature.stderr)
result={"utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),"source_head":before["head"],"source_before_sha256":hashlib.sha256((out/"SOURCE_BEFORE.json").read_bytes()).hexdigest(),"bound_input_count":len(before["inputs"]),"source_inputs_unchanged":not changed,"bundle_path":str(bundle),"binary_path":str(binary),"binary_sha256":hashlib.sha256(binary.read_bytes()).hexdigest(),"assets":assets,"self_test":{"command":cmd,"exit_code":run.returncode,"duration_seconds":time.monotonic()-started,"evidence":json.loads(run.stdout) if run.returncode==0 else None},"signature_inspection_exit_code":signature.returncode,"host":platform.platform(),"limits":["Compiled native backend self-test; no GUI interaction or usability proof.","Working-tree build, not release publication or lifecycle acceptance.","No association asserted with any previously running user app."]}
(out/"BUNDLE_VERIFICATION.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({k:v for k,v in result.items() if k!="assets"},indent=2))
raise SystemExit(run.returncode)
