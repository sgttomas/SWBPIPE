"""Lease-only backup of the unique final-native SQLite store."""
from pathlib import Path
import argparse, hashlib, json, sqlite3
p=argparse.ArgumentParser(); p.add_argument("--store",required=True); p.add_argument("--output",required=True); p.add_argument("--project",required=True); a=p.parse_args()
store=Path(a.store).resolve(); output=Path(a.output).resolve()
assert "org.openpipestress.foundation-final-20260915" in str(store)
assert "instances/NATIVE/FINAL_NATIVE_V2/_run_records/store" in str(output)
assert store.is_file() and not output.exists()
src=sqlite3.connect("file:"+str(store)+"?mode=ro",uri=True); dst=sqlite3.connect(output); src.backup(dst); dst.close(); src.close()
b=output.read_bytes(); c=sqlite3.connect("file:"+str(output)+"?mode=ro&immutable=1",uri=True); c.row_factory=sqlite3.Row
integrity=c.execute("pragma integrity_check").fetchone()[0]; assert integrity=="ok"
row=c.execute("select * from local_projects where project_id=?",(a.project,)).fetchone(); assert row is not None
columns=[d[0] for d in c.execute("select * from local_projects limit 0").description]; version=c.execute("pragma user_version").fetchone()[0]; c.close()
output.with_suffix(".row.json").write_text(json.dumps(dict(row),indent=2,sort_keys=True)+"\n")
output.with_suffix(".capture.json").write_text(json.dumps({"source":str(store),"method":"sqlite online backup from read-only source connection; immutable offline-copy read","output":str(output),"output_sha256":hashlib.sha256(b).hexdigest(),"output_bytes":len(b),"integrity_check":integrity,"store_schema_version":version,"columns":columns,"project_id":row["project_id"]},indent=2,sort_keys=True)+"\n")
