"""Future lease-only isolated SQLite backup; never run during preparation."""
from pathlib import Path
import argparse, json, sqlite3
p=argparse.ArgumentParser();p.add_argument("--store",required=True);p.add_argument("--output",required=True);p.add_argument("--project",required=True);a=p.parse_args()
store=Path(a.store).resolve(); output=Path(a.output).resolve()
assert "org.openpipestress.technical-preview.results-engineering-3d-20260913" in str(store)
assert "instances/NATIVE_VERIFY" in str(output)
assert not output.exists()
src=sqlite3.connect("file:"+str(store)+"?mode=ro",uri=True); dst=sqlite3.connect(output);src.backup(dst);dst.close();src.close()
c=sqlite3.connect(output);c.row_factory=sqlite3.Row
row=c.execute("select * from local_projects where project_id=?",(a.project,)).fetchone(); assert row is not None
output.with_suffix(".json").write_text(json.dumps(dict(row),indent=2)+"\n");c.close()
