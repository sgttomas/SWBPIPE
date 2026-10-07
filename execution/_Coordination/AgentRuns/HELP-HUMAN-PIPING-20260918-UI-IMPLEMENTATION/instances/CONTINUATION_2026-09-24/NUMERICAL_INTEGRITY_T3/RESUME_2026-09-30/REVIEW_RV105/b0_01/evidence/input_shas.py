# RV105: recompute the I68 probe input sha256 values from the committed test helpers' construction.
import json, hashlib, struct, sys
P = sys.argv[1]
def f64(bits): return struct.unpack('>d', bits.to_bytes(8, 'big'))[0]
def ser(v):
    return json.dumps(v, sort_keys=True, separators=(',', ':'), ensure_ascii=False)
def sha(v): return hashlib.sha256(ser(v).encode()).hexdigest()
milestone = json.load(open(P + '/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json'))
p = "invented_t3_g5_witness_input_no_library_data"
def two_body_a():
    raw = json.loads(json.dumps(milestone)); m = raw['model']
    m['nodes'] += [{"id": "node:section-a", "position": {"x": 5.0, "y": 0.0, "z": 0.0}, "provenance": p},
                   {"id": "node:section-b", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": p}]
    m['pipe_segments'].append({"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b",
        "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
        "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p})
    m['materials'].append({"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p})
    m['supports'].append({"id": "support:section-a", "node": "node:section-a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p})
    return raw
def two_body_b():
    raw = two_body_a(); tip = f64(0x0031fa182c40c60d)
    raw['model']['load_cases'][0]['primitive_loads'] = [
        {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y", "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
        {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x", "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]
    return raw
print('tip', repr(f64(0x0031fa182c40c60d)))
print('two_body_a', sha(two_body_a()))
print('two_body_b', sha(two_body_b()))
print('milestone', sha(milestone))
