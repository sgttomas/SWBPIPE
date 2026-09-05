from pathlib import Path
import json, subprocess
root=Path(subprocess.check_output(["git","rev-parse","--show-toplevel"],text=True).strip())
w=root/"projects/chirality-piping"
packet=Path(__file__).resolve().parent
scratch=Path("/tmp/piping-a2-witness-20260905")
(scratch/"src").mkdir(parents=True,exist_ok=True)
(packet/"fixtures").mkdir(exist_ok=True)
x=json.loads((w/"fixtures/product_preview/invented_preview_model.json").read_text())
# Invented explicit straight cantilever, retaining entered repository section/material values.
x["nodes"]=x["nodes"][:2]
x["nodes"][0]["position"]={"x":0,"y":0,"z":0}
x["nodes"][1]["position"]={"x":2,"y":0,"z":0}
x["pipe_segments"]=x["pipe_segments"][:1]
x["supports"]=x["supports"][:1]
x["components"]=[]; x["combinations"]=[]
x["load_cases"]=x["load_cases"][:1]
x["load_cases"][0]["primitive_loads"]=[{"id":"load:A2-invented","category":"occasional","target":{"type":"node","node":x["nodes"][1]["id"]},"direction":"global_y","magnitude":{"value":350,"unit":"N"},"dimension":"force","provenance":"invented_example_A2_deterministic_witness_no_engineering_acceptance"}]
for unit, factor in [("m",1),("mm",1000),("in",1/0.0254)]:
 y=json.loads(json.dumps(x)); y["project"]["units"]["length"]=unit
 for node in y["nodes"]:
  node["position"]={axis:value*factor for axis,value in node["position"].items()}
 (packet/"fixtures"/f"invented_linear_{unit}.json").write_text(json.dumps(y,indent=2)+"\n")
(scratch/"Cargo.toml").write_text(f'''[package]
name="piping-a2-witness"
version="0.1.0"
edition="2021"
[dependencies]
open_pipe_stress_product_physics={{path="{w}/core/product_physics"}}
serde_json="1"
rusqlite={{version="0.32",features=["bundled"]}}
serde={{version="1",features=["derive"]}}
''')
(scratch/"src/main.rs").write_text((packet/"native_witness.rs").read_text())
print(json.dumps({"root":str(root),"scratch":str(scratch),"head":subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()}))
