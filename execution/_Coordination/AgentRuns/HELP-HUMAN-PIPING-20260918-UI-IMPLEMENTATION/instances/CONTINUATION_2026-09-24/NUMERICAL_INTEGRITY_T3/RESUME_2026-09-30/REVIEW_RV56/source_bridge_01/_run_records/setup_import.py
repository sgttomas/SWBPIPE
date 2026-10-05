"""Rebuild RV56's bounded private review harness from an explicit FK source.
Usage: python3 setup_import.py /absolute/path/to/frame_kernel /new/review/harness
Destination must not already exist; source is read-only. Never runs Cargo.
"""
from pathlib import Path
import json,sys,hashlib,datetime
src=Path(sys.argv[1]).resolve();dst=Path(sys.argv[2]).resolve();records=Path(__file__).resolve().parent
dst.mkdir()
def overlay(rel,overrides):
 (dst/rel).mkdir(parents=True,exist_ok=True)
 for item in (src/rel).iterdir():
  key=str(item.relative_to(src))
  if key not in overrides:(dst/key).symlink_to(item,target_is_directory=item.is_dir())
overlay('src',{'src/structural'})
overlay('src/structural',{'src/structural/retained'})
overlay('src/structural/retained',{'src/structural/retained/adaptive.rs'})
(dst/'src/structural/retained/adaptive.rs').write_bytes((src/'src/structural/retained/adaptive.rs').read_bytes())
overlay('tests',{'tests/retained_k4'})
overlay('tests/retained_k4',{'tests/retained_k4/source_bridge_tests.rs'})
(dst/'tests/retained_k4/source_bridge_tests.rs').write_bytes((src/'tests/retained_k4/source_bridge_tests.rs').read_bytes()+b'\n'+(records/'rv56_controls.rs').read_bytes())
(dst/'tests/retained_k4/review_vectors.rs').write_bytes((records/'review_vectors.rs').read_bytes())
(dst/'Cargo.toml').write_text('[package]\nname="rv56_source_bridge_review"\nversion="0.0.0"\nedition="2021"\n[lib]\npath="src/lib.rs"\n[features]\nmutation-controls=[]\n[workspace]\n')
(dst/'Cargo.lock').write_text('version = 4\n\n[[package]]\nname = "rv56_source_bridge_review"\nversion = "0.0.0"\n')
print(json.dumps({'source':str(src),'destination':str(dst),'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'adaptive_sha256':hashlib.sha256((src/'src/structural/retained/adaptive.rs').read_bytes()).hexdigest()},indent=2))
