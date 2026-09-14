from pathlib import Path
out=Path(__file__).resolve().parent
p=out/'prototype/workspace.js';s=p.read_text().replace("$('tools').innerHTML=(S.group==='Build'&&!q?", "$('tools').innerHTML=(S.group==='Properties'&&!q?'<button data-property=\"true\">Edit pipe properties</button>':'')+(S.group==='Build'&&!q?")
s=s.replace("const b=e.target.closest('[data-capability],[data-node]');", "const b=e.target.closest('[data-capability],[data-node],[data-property]');")
s=s.replace("if(b.dataset.node){", "if(b.dataset.property){activate(17,'property');return;}if(b.dataset.node){")
s=s.replace("$('task-title').textContent=S.tool==='node'?'Create node':", "$('task-title').textContent=S.tool==='property'?'Pipe properties':S.tool==='node'?'Create node':")
p.write_text(s)
# Row 17 is a contextual mapping; pipe property correction is a separate set_field shortcut.
p=out/'write_maps.py';s=p.read_text().replace(",17:'property'",'');p.write_text(s)
