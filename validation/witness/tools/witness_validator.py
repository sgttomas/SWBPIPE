#!/usr/bin/env python3
"""Validate and render validation-local OpenMath calculation witnesses."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from decimal import Decimal, getcontext
import hashlib
from html import escape
import json
from pathlib import Path
import sys
from typing import Any

from jsonschema import Draft202012Validator


ROOT = Path(__file__).resolve().parents[3]
DEFAULT_WITNESS_PATH = (
    ROOT
    / "validation"
    / "witness"
    / "fixtures"
    / "tp_phys_015_section_property_stress_witness.json"
)
SCHEMA_PATH = (
    ROOT
    / "validation"
    / "witness"
    / "schemas"
    / "openmath_calculation_witness.schema.json"
)

getcontext().prec = 50

DIMENSIONS: dict[str, tuple[int, int]] = {
    "dimensionless": (0, 0),
    "length": (1, 0),
    "force": (0, 1),
    "moment": (1, 1),
    "stress": (-2, 1),
    "area": (2, 0),
    "section_modulus": (3, 0),
    "second_moment_area": (4, 0),
}
DIMENSION_NAMES = {value: key for key, value in DIMENSIONS.items()}
SUPPORTED_ARITH = {"plus", "minus", "times", "divide", "power"}


class WitnessError(Exception):
    """Raised when a formal witness is invalid or generated artifacts are stale."""


@dataclass(frozen=True)
class QuantityValue:
    decimal: Decimal
    unit: str
    dimension: tuple[int, int]
    dimension_name: str


@dataclass(frozen=True)
class FormulaEvaluation:
    formula_id: str
    display: str
    expression: str
    value: QuantityValue


@dataclass(frozen=True)
class Comparison:
    formula_id: str
    result_id: str
    witness_value: Decimal
    ops_value: Decimal
    abs_delta: Decimal
    tolerance: Decimal
    passed: bool


@dataclass(frozen=True)
class WitnessEvaluation:
    witness_id: str
    witness_sha256: str
    formula_evaluations: list[FormulaEvaluation]
    comparisons: list[Comparison]


def load_json(path: str | Path) -> Any:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def canonical_sha256(witness: dict[str, Any]) -> str:
    payload = json.dumps(witness, sort_keys=True, separators=(",", ":")).encode(
        "utf-8"
    )
    return hashlib.sha256(payload).hexdigest()


def repo_path(path: str | Path) -> Path:
    candidate = Path(path)
    if candidate.is_absolute():
        return candidate
    return ROOT / candidate


def decimal_text(value: Decimal) -> str:
    if value == 0:
        return "0"
    return format(value.normalize(), "f")


def dimension_name(vector: tuple[int, int]) -> str:
    return DIMENSION_NAMES.get(vector, f"derived{vector}")


def validate_schema(witness: dict[str, Any]) -> None:
    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(witness), key=lambda error: error.path)
    if errors:
        first = errors[0]
        path = ".".join(str(part) for part in first.path) or "<root>"
        raise WitnessError(
            f"witness schema validation failed at {path}: {first.message}"
        )


def quantity_from_binding(binding: dict[str, Any]) -> QuantityValue:
    quantity = binding["quantity"]
    dimension_id = quantity["dimension"]
    return QuantityValue(
        decimal=Decimal(quantity["decimal"]),
        unit=quantity["unit"],
        dimension=DIMENSIONS[dimension_id],
        dimension_name=dimension_id,
    )


def arith_result(name: str, args: list[QuantityValue]) -> QuantityValue:
    if name not in SUPPORTED_ARITH:
        raise WitnessError(f"unsupported OpenMath operator: arith1.{name}")
    if name in {"plus", "minus"}:
        first_dimension = args[0].dimension
        if any(arg.dimension != first_dimension for arg in args):
            raise WitnessError(f"dimension mismatch for arith1.{name}")
        if name == "plus":
            value = sum((arg.decimal for arg in args), Decimal("0"))
        else:
            if len(args) != 2:
                raise WitnessError("arith1.minus requires exactly two arguments")
            value = args[0].decimal - args[1].decimal
        return QuantityValue(value, args[0].unit, first_dimension, dimension_name(first_dimension))

    if name == "times":
        value = Decimal("1")
        dimension = (0, 0)
        for arg in args:
            value *= arg.decimal
            dimension = (
                dimension[0] + arg.dimension[0],
                dimension[1] + arg.dimension[1],
            )
        return QuantityValue(value, "", dimension, dimension_name(dimension))

    if name == "divide":
        if len(args) != 2:
            raise WitnessError("arith1.divide requires exactly two arguments")
        dimension = (
            args[0].dimension[0] - args[1].dimension[0],
            args[0].dimension[1] - args[1].dimension[1],
        )
        return QuantityValue(
            args[0].decimal / args[1].decimal,
            "",
            dimension,
            dimension_name(dimension),
        )

    if len(args) != 2:
        raise WitnessError("arith1.power requires exactly two arguments")
    if args[1].dimension != DIMENSIONS["dimensionless"]:
        raise WitnessError("dimension mismatch for arith1.power exponent")
    exponent = int(args[1].decimal)
    if Decimal(exponent) != args[1].decimal:
        raise WitnessError("arith1.power exponent must be an integer")
    dimension = (args[0].dimension[0] * exponent, args[0].dimension[1] * exponent)
    return QuantityValue(
        args[0].decimal**exponent,
        "",
        dimension,
        dimension_name(dimension),
    )


def evaluate_openmath(
    node: dict[str, Any],
    values: dict[str, QuantityValue],
    formulas: dict[str, dict[str, Any]],
    stack: list[str],
) -> QuantityValue:
    om_type = node["om_type"]
    if om_type == "OMV":
        name = node["name"]
        if name in values:
            return values[name]
        if name in stack:
            raise WitnessError(f"formula graph cycle detected: {' -> '.join(stack + [name])}")
        if name in formulas:
            return evaluate_formula(name, values, formulas, stack)
        raise WitnessError(f"unknown OpenMath variable: {name}")
    if om_type in {"OMI", "OMF"}:
        return QuantityValue(
            Decimal(node["decimal"]),
            "1",
            DIMENSIONS["dimensionless"],
            "dimensionless",
        )
    if om_type == "OMS":
        if node["cd"] == "nums1" and node["name"] == "pi":
            if "pi" in values:
                return values["pi"]
            raise WitnessError("nums1.pi binding missing")
        raise WitnessError(f"unsupported OpenMath symbol: {node['cd']}.{node['name']}")
    if om_type == "OMA":
        operator = node["operator"]
        if operator.get("om_type") != "OMS" or operator.get("cd") != "arith1":
            cd = operator.get("cd", "<missing>")
            name = operator.get("name", "<missing>")
            raise WitnessError(f"unsupported OpenMath operator: {cd}.{name}")
        args = [
            evaluate_openmath(argument, values, formulas, stack)
            for argument in node["arguments"]
        ]
        return arith_result(operator["name"], args)
    raise WitnessError(f"unsupported OpenMath node type: {om_type}")


def evaluate_formula(
    formula_id: str,
    values: dict[str, QuantityValue],
    formulas: dict[str, dict[str, Any]],
    stack: list[str],
) -> QuantityValue:
    if formula_id in values:
        return values[formula_id]
    if formula_id in stack:
        raise WitnessError(
            f"formula graph cycle detected: {' -> '.join(stack + [formula_id])}"
        )
    formula = formulas[formula_id]
    value = evaluate_openmath(formula["openmath"], values, formulas, stack + [formula_id])
    expected_dimension = DIMENSIONS[formula["expected_dimension"]]
    if value.dimension != expected_dimension:
        raise WitnessError(
            f"dimension mismatch for {formula_id}: expected "
            f"{formula['expected_dimension']} got {value.dimension_name}"
        )
    values[formula_id] = QuantityValue(
        value.decimal,
        formula["expected_unit"],
        value.dimension,
        formula["expected_dimension"],
    )
    return values[formula_id]


def display_map(witness: dict[str, Any]) -> dict[str, str]:
    result: dict[str, str] = {}
    for section in ("constants", "inputs", "formulas"):
        for item in witness[section]:
            result[item["id"]] = item["display"]
    return result


def expression_for(node: dict[str, Any], displays: dict[str, str]) -> str:
    om_type = node["om_type"]
    if om_type == "OMV":
        return displays.get(node["name"], node["name"])
    if om_type in {"OMI", "OMF"}:
        return decimal_text(Decimal(node["decimal"]))
    if om_type == "OMS":
        if node["cd"] == "nums1" and node["name"] == "pi":
            return "pi"
        raise WitnessError(f"unsupported OpenMath symbol: {node['cd']}.{node['name']}")
    if om_type != "OMA":
        raise WitnessError(f"unsupported OpenMath node type: {om_type}")

    operator = node["operator"]
    if operator.get("om_type") != "OMS" or operator.get("cd") != "arith1":
        cd = operator.get("cd", "<missing>")
        name = operator.get("name", "<missing>")
        raise WitnessError(f"unsupported OpenMath operator: {cd}.{name}")
    name = operator["name"]
    args = [expression_for(argument, displays) for argument in node["arguments"]]
    if name == "plus":
        return f"({' + '.join(args)})"
    if name == "minus":
        return f"({args[0]} - {args[1]})"
    if name == "times":
        return f"({' * '.join(args)})"
    if name == "divide":
        return f"({args[0]} / {args[1]})"
    if name == "power":
        return f"({args[0]}^{args[1]})"
    raise WitnessError(f"unsupported OpenMath operator: arith1.{name}")


def flatten_results(result_export: dict[str, Any]) -> dict[str, dict[str, Any]]:
    results: dict[str, dict[str, Any]] = {}
    for result_set in result_export["result_envelope"]["result_sets"]:
        for value in result_set["values"]:
            results[value["result_id"]] = value
    return results


def evaluate_witness(witness: dict[str, Any]) -> WitnessEvaluation:
    validate_schema(witness)
    values: dict[str, QuantityValue] = {}
    for binding in witness["constants"] + witness["inputs"]:
        values[binding["id"]] = quantity_from_binding(binding)

    formulas = {formula["id"]: formula for formula in witness["formulas"]}
    displays = display_map(witness)
    formula_evaluations: list[FormulaEvaluation] = []
    for formula in witness["formulas"]:
        value = evaluate_formula(formula["id"], values, formulas, [])
        expression = f"{formula['display']} = {expression_for(formula['openmath'], displays)}"
        formula_evaluations.append(
            FormulaEvaluation(
                formula_id=formula["id"],
                display=formula["display"],
                expression=expression,
                value=value,
            )
        )

    result_export = load_json(repo_path(witness["result_export"]["path"]))
    result_values = flatten_results(result_export)
    comparisons: list[Comparison] = []
    for output in witness["outputs"]:
        formula_value = values[output["formula_id"]]
        try:
            result_value = result_values[output["result_id"]]
        except KeyError as exc:
            raise WitnessError(f"missing OPS result: {output['result_id']}") from exc
        if formula_value.dimension_name != result_value["dimension"]:
            raise WitnessError(
                f"OPS result dimension mismatch for {output['result_id']}: "
                f"expected {formula_value.dimension_name} got {result_value['dimension']}"
            )
        ops_value = Decimal(str(result_value["magnitude"]))
        tolerance = Decimal(output["abs_tolerance_decimal"])
        delta = abs(formula_value.decimal - ops_value)
        passed = delta <= tolerance
        comparisons.append(
            Comparison(
                formula_id=output["formula_id"],
                result_id=output["result_id"],
                witness_value=formula_value.decimal,
                ops_value=ops_value,
                abs_delta=delta,
                tolerance=tolerance,
                passed=passed,
            )
        )
        if not passed:
            raise WitnessError(
                f"OPS result mismatch for {output['result_id']}: "
                f"witness {decimal_text(formula_value.decimal)} vs "
                f"OPS {decimal_text(ops_value)} delta {decimal_text(delta)} "
                f"> tolerance {decimal_text(tolerance)}"
            )

    return WitnessEvaluation(
        witness_id=witness["witness_id"],
        witness_sha256=canonical_sha256(witness),
        formula_evaluations=formula_evaluations,
        comparisons=comparisons,
    )


def render_markdown(witness: dict[str, Any]) -> str:
    evaluation = evaluate_witness(witness)
    rows: list[str] = [
        f"# Generated Hand-Calc Witness: {witness['witness_id']}",
        "",
        "This Markdown is generated from the machine-readable witness artifact.",
        "Do not edit this file as the authoritative calculation source.",
        "",
        f"- Witness title: {witness['title']}",
        f"- Witness SHA-256 over canonical JSON: `sha256:{evaluation.witness_sha256}`",
        f"- Semantic source: {witness['openmath_binding']['semantic_source']}",
        f"- JSON binding: `{witness['openmath_binding']['json_binding']}`",
        f"- MathML rendering: `{witness['openmath_binding']['mathml_rendering']}`",
        "",
        "## Provenance",
        "",
        "| Field | Value |",
        "|---|---|",
    ]
    for field, value in witness["provenance"].items():
        rows.append(f"| `{field}` | {value} |")

    rows.extend(
        [
            "",
            "## Inputs",
            "",
            "| ID | Symbol | Value | Unit | Dimension |",
            "|---|---|---:|---|---|",
        ]
    )
    for binding in witness["constants"] + witness["inputs"]:
        quantity = binding["quantity"]
        rows.append(
            f"| `{binding['id']}` | `{binding['display']}` | "
            f"{quantity['decimal']} | `{quantity['unit']}` | "
            f"`{quantity['dimension']}` |"
        )

    rows.extend(
        [
            "",
            "## Formula Evaluation",
            "",
            "| Formula | OpenMath-derived expression | Evaluated value | Unit | Dimension |",
            "|---|---|---:|---|---|",
        ]
    )
    for formula in evaluation.formula_evaluations:
        rows.append(
            f"| `{formula.display}` | `{formula.expression}` | "
            f"{decimal_text(formula.value.decimal)} | `{formula.value.unit}` | "
            f"`{formula.value.dimension_name}` |"
        )

    rows.extend(
        [
            "",
            "## Result Export Comparison",
            "",
            "| Output | OPS result ID | Witness value | OPS value | Abs delta | Tolerance |",
            "|---|---|---:|---:|---:|---:|",
        ]
    )
    output_by_formula = {output["formula_id"]: output for output in witness["outputs"]}
    for comparison in evaluation.comparisons:
        output = output_by_formula[comparison.formula_id]
        rows.append(
            f"| `{output['id']}` | `{comparison.result_id}` | "
            f"{decimal_text(comparison.witness_value)} | "
            f"{decimal_text(comparison.ops_value)} | "
            f"{decimal_text(comparison.abs_delta)} | "
            f"{decimal_text(comparison.tolerance)} |"
        )

    rows.extend(
        [
            "",
            "## Boundary",
            "",
            "This witness is mechanics-only software verification evidence. It does not "
            "compare to allowables, classify code stress categories, apply fatigue rules, "
            "use SIF/flexibility factors, certify, seal, authenticate, approve, or "
            "declare code compliance.",
            "",
        ]
    )
    return "\n".join(rows)


def mathml_for(node: dict[str, Any], indent: int) -> list[str]:
    prefix = " " * indent
    om_type = node["om_type"]
    if om_type == "OMV":
        return [f"{prefix}<ci>{escape(node['name'])}</ci>"]
    if om_type in {"OMI", "OMF"}:
        return [f"{prefix}<cn>{escape(decimal_text(Decimal(node['decimal'])))}</cn>"]
    if om_type == "OMS" and node["cd"] == "nums1" and node["name"] == "pi":
        return [f"{prefix}<pi/>"]
    if om_type != "OMA":
        raise WitnessError(f"unsupported OpenMath node type for MathML: {om_type}")

    operator = node["operator"]
    if operator.get("om_type") != "OMS" or operator.get("cd") != "arith1":
        cd = operator.get("cd", "<missing>")
        name = operator.get("name", "<missing>")
        raise WitnessError(f"unsupported OpenMath operator for MathML: {cd}.{name}")
    name = operator["name"]
    if name not in SUPPORTED_ARITH:
        raise WitnessError(f"unsupported OpenMath operator for MathML: arith1.{name}")

    lines = [f"{prefix}<apply>", f"{' ' * (indent + 2)}<{name}/>"]
    for argument in node["arguments"]:
        lines.extend(mathml_for(argument, indent + 2))
    lines.append(f"{prefix}</apply>")
    return lines


def render_mathml(witness: dict[str, Any]) -> str:
    evaluation = evaluate_witness(witness)
    lines = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        (
            f'<ops-witness-mathml witness-id="{escape(witness["witness_id"])}" '
            f'witness-sha256="{evaluation.witness_sha256}">'
        ),
    ]
    for formula in witness["formulas"]:
        lines.extend(
            [
                (
                    f'  <math xmlns="http://www.w3.org/1998/Math/MathML" '
                    f'display="block" id="{escape(formula["id"])}">'
                ),
                "    <semantics>",
                "      <apply>",
                "        <eq/>",
                f"        <ci>{escape(formula['display'])}</ci>",
            ]
        )
        lines.extend(mathml_for(formula["openmath"], 8))
        lines.extend(
            [
                "      </apply>",
                (
                    '      <annotation encoding="application/x-openmath-json">'
                    f"{escape(formula['id'])}</annotation>"
                ),
                "    </semantics>",
                "  </math>",
            ]
        )
    lines.append("</ops-witness-mathml>")
    return "\n".join(lines) + "\n"


def assert_generated_artifacts_current(witness: dict[str, Any]) -> None:
    expected_markdown = render_markdown(witness)
    expected_mathml = render_mathml(witness)
    markdown_path = repo_path(witness["generated_artifacts"]["markdown_path"])
    mathml_path = repo_path(witness["generated_artifacts"]["mathml_path"])
    if markdown_path.read_text(encoding="utf-8") != expected_markdown:
        raise WitnessError(f"generated Markdown is stale: {markdown_path}")
    if mathml_path.read_text(encoding="utf-8") != expected_mathml:
        raise WitnessError(f"generated MathML is stale: {mathml_path}")


def write_generated_artifacts(witness: dict[str, Any]) -> None:
    markdown_path = repo_path(witness["generated_artifacts"]["markdown_path"])
    mathml_path = repo_path(witness["generated_artifacts"]["mathml_path"])
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    mathml_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text(render_markdown(witness), encoding="utf-8")
    mathml_path.write_text(render_mathml(witness), encoding="utf-8")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate and render OpenPipeStress formal calculation witnesses."
    )
    parser.add_argument(
        "witness",
        nargs="?",
        default=str(DEFAULT_WITNESS_PATH),
        help="Witness JSON path. Defaults to the TP-PHYS-015 pilot witness.",
    )
    parser.add_argument(
        "--write-generated",
        action="store_true",
        help="Regenerate Markdown and MathML artifacts from the witness JSON.",
    )
    parser.add_argument(
        "--check-generated",
        action="store_true",
        help="Check generated Markdown and MathML artifacts are current.",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    witness = load_json(args.witness)
    try:
        evaluate_witness(witness)
        if args.write_generated:
            write_generated_artifacts(witness)
        if args.check_generated:
            assert_generated_artifacts_current(witness)
    except WitnessError as exc:
        print(f"witness validation failed: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
