# Timoshenko straight mass: source-blind exact element references

These references serve DEL-04-09 REQ-001, REQ-009, AC-012 and VER-009. They give the consistent mass of a two-node straight frame member whose bending stiffness is the Timoshenko element, defined in the problem statement in this package.

They establish the mass of that stated formulation, independently. That formulation is the exact homogeneous static interpolation, with independently supplied inertia coefficients. The references do not validate the formulation's physical adequacy.

## Provenance

The references were independently derived and refuted before their product comparison.

**The problem statement.**
- `PROBLEM.md`, SHA-256 `ab4a949c2533603d91d27cdeff561207d0339997dbe1e53bbc783ca8cc00784d`, holds the mathematics, the interface conventions and the operands of five cases, with no coefficient and no expected value.
- `ADDENDUM-1.md`, SHA-256 `40b5954acac00205fe05db8fa167251f82b5c8a21850eb86e8df74e614cc1d91`, adds three cases and one output rule: a Φ outside binary64 is given as an exact rational.
- SWBPIPE's Agent 0 froze both, after the INTERLOCUTOR read the statement.

**The deriver.**
- A fresh agent with no inherited conversation wrote `generator.py` from the statement alone. Its read list was closed: its role text, a copy of AGENTS.md and the statement, later the addendum. It read no product code and no other generator.
- The read boundary was held by instruction and checked afterwards against the agent's tool record. The file system did not enforce it.
- The host put conduct text and environment metadata into the agent's starting context. That held no product code and no expected value.

**The refuter.** A second fresh agent, on a different model, with the same boundary plus this package:
- confirmed the statement's equilibrium, mass definition and conventions;
- reproduced both outputs byte for byte;
- agreed with every value as an exact rational, through an independent derivation that shares no code with `generator.py`;
- detected five wrong-formulation and wrong-scale probes on every case.

It reported that the statement's invariant self-checks alone would pass a wrong rotary diagonal of a particular form. The substantive evidence is therefore the full-entry agreement of the two derivations.

**The addendum.** The deriver extended the generator for the addendum, and the refuter examined the additions.
- It reproduced both outputs and agreed with every added value as an exact rational.
- It found one defect in the value encoder: on in-range values outside the eight cases, a shear parameter whose nonzero low residual rounds to zero raised instead of being written.
- The deriver repaired it from the witness inputs alone. The refuter then confirmed the repair against eighteen boundary probes, with both outputs unchanged byte for byte.

At Φ = 2⁻²⁰⁰⁰ an Euler–Bernoulli substitution has the same rounded encodings as this element, but a different exact matrix. Only the exact-rational comparison tells them apart there.

## Files

| File | What it is |
|---|---|
| `PROBLEM.md` | The frozen problem statement. |
| `ADDENDUM-1.md` | The frozen addendum. |
| `generator.py` | The source-blind generator, at its refuted digest. Python 3 standard library only. It reads, imports and runs no product code. |
| `generator_README.md` | The deriver's own description of the generator, the file layout and the value encoding, verbatim. |
| `timoshenko_consistent_mass.json` | The statement's five cases. |
| `timoshenko_consistent_mass_addendum1.json` | The addendum's three cases. |
| `SHA256SUMS` | Hashes of every other file in the package, in `shasum -a 256` format. |

## Check

From the package directory:

```sh
python3 -I -B generator.py           # rebuild both outputs in memory and compare them byte for byte
python3 -I -B generator.py --write   # rewrite both outputs
```

## Limits

These are element references. They do not reference:
- the product's mass assembly, mass per length or rotary inertia;
- the shear-area derivation;
- the integrity re-formation or the readers;
- any frequency.

The continuum Timoshenko references of AC-012 are separate.
