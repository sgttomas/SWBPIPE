from pathlib import Path
import sys,json,sqlite3
raw=Path(__file__).parent
store=Path('/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.workflow-contact-20260913/openpipestress-projects.sqlite3')
name=sys.argv[1]
p=raw/(name+'.sqlite3'); assert not p.exists(); p.write_bytes(store.read_bytes())
c=sqlite3.connect(p);c.row_factory=sqlite3.Row
query='select * from local_projects'+(' where project_id=?' if len(sys.argv)>2 else ' order by updated_at_unix desc limit 1')
x=dict(c.execute(query,(sys.argv[2],) if len(sys.argv)>2 else ()).fetchone())
(raw/(name+'.json')).write_text(json.dumps(x,indent=2)+'\n')
print(x['project_id'])
