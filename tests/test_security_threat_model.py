#!/usr/bin/env python3
"""Focused checks for DEL-12-05 security threat-model SCA-004 coverage."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
THREAT_MODEL = ROOT / "docs" / "security" / "threat_model.md"


def threat_model_text() -> str:
    return THREAT_MODEL.read_text(encoding="utf-8").lower()


def test_sca004_export_workflows_are_covered():
    text = threat_model_text()
    required_phrases = {
        "sca-004",
        "target profiles",
        "stable id maps",
        "loss reports",
        "caepipe mbf",
        "external-harness evidence",
        "stress-neutral csv/json",
        "conservative pcf",
        "glb/gltf review geometry",
        "native open json",
        "export adapter sdk",
    }

    missing = sorted(phrase for phrase in required_phrases if phrase not in text)
    assert not missing


def test_sca004_controls_preserve_security_and_data_boundaries():
    text = threat_model_text()
    required_phrases = {
        "do not bundle",
        "bypass licenses",
        "reverse engineer protected formats",
        "proprietary examples",
        "leak private data by default",
        "telemetry is disabled by default",
        "no direct sql",
        "raw sqlite",
        "denied by default",
        "user-owned at external-tool boundaries",
        "non-authoritative evidence labels",
    }

    missing = sorted(phrase for phrase in required_phrases if phrase not in text)
    assert not missing


def test_threat_model_avoids_positive_reliance_and_target_claims():
    text = threat_model_text()
    prohibited_positive_phrases = {
        "is code compliant",
        "are code compliant",
        "certifies engineering",
        "security certified",
        "professionally approved",
        "approved for reliance",
        "validated by caepipe",
        "caepipe-compatible",
        "vendor-compatible",
        "asme table",
        "b31j",
        "real client",
    }

    present = sorted(phrase for phrase in prohibited_positive_phrases if phrase in text)
    assert not present


if __name__ == "__main__":
    test_sca004_export_workflows_are_covered()
    test_sca004_controls_preserve_security_and_data_boundaries()
    test_threat_model_avoids_positive_reliance_and_target_claims()
    print("PASS: DEL-12-05 security threat-model checks")
