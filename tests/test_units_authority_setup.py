"""Explicit setup/adapter behavior; no alternative conversion implementation."""
from pathlib import Path
from types import SimpleNamespace
import importlib.util
import math
import os
import sys
import pytest
from core.units.adapter import convert_quantities_to_canonical, units_executable

ROOT=Path(__file__).resolve().parents[1]
def module(path, name):
    spec=importlib.util.spec_from_file_location(name,path); loaded=importlib.util.module_from_spec(spec); spec.loader.exec_module(loaded);return loaded

def test_controller_prepares_units_even_with_existing_json_authority(monkeypatch, tmp_path):
    conftest=module(ROOT/'tests/conftest.py','units_setup_test')
    monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN','/configured/checked-json')
    monkeypatch.delenv('OPENPIPESTRESS_UNITS_BIN',raising=False)
    calls=[]; target=tmp_path/'units-authority'
    monkeypatch.setitem(sys.modules,'build_units_authority',SimpleNamespace(build=lambda:(calls.append('units') or target)))
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))
    assert calls==['units'];assert os.environ['OPENPIPESTRESS_UNITS_BIN']==str(target)

def test_workers_and_configured_controller_do_not_build(monkeypatch):
    conftest=module(ROOT/'tests/conftest.py','units_setup_worker_test')
    def forbidden():raise AssertionError('unexpected build')
    monkeypatch.setitem(sys.modules,'build_units_authority',SimpleNamespace(build=forbidden))
    monkeypatch.setitem(sys.modules,'build_checked_json',SimpleNamespace(build=forbidden))
    monkeypatch.delenv('OPENPIPESTRESS_UNITS_BIN',raising=False);monkeypatch.delenv('OPENPIPESTRESS_CHECKED_JSON_BIN',raising=False)
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace(workerinput={})))
    monkeypatch.setenv('OPENPIPESTRESS_UNITS_BIN','/configured/units');monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN','/configured/json')
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))

def test_explicit_build_uses_locked_release_cli_and_target(monkeypatch,tmp_path):
    helper=module(ROOT/'tools/units/build_units_authority.py','units_build_test'); calls=[]
    monkeypatch.setattr(helper.subprocess,'run',lambda args,**kwargs:calls.append((args,kwargs)))
    result=helper.build(tmp_path/'target')
    assert result==(tmp_path/'target/release/openpipestress_units').resolve()
    args,options=calls[0];assert '--locked' in args and '--release' in args
    assert args[args.index('--features')+1]=='cli';assert options['cwd']==ROOT/'core/units';assert options['check'] is True

def test_adapter_requires_configured_absolute_authority(monkeypatch):
    monkeypatch.delenv('OPENPIPESTRESS_UNITS_BIN',raising=False)
    with pytest.raises(RuntimeError,match='UNITS-AUTHORITY-MISSING'):units_executable()
    monkeypatch.setenv('OPENPIPESTRESS_UNITS_BIN','relative/path')
    with pytest.raises(RuntimeError,match='UNITS-AUTHORITY-MISSING'):units_executable()

def test_actual_authority_batch_and_dimension_refusal():
    rows=convert_quantities_to_canonical([{'id':'E','value':200.,'unit':'GPa','dimension':'stress'}, {'id':'T','value':68.,'unit':'degF','dimension':'temperature'}])
    assert rows[0]=={'id':'E','value':2e11,'unit':'Pa','dimension':'stress'}
    assert rows[1]['unit']=='K' and math.isfinite(rows[1]['value'])
    with pytest.raises(ValueError,match='UNITS_CONVERSION_REFUSED'):
        convert_quantities_to_canonical([{'id':'wrong','value':1.,'unit':'m','dimension':'stress'}])
    for value in [True,float('inf'),float('nan'),10**10000]:
        with pytest.raises(ValueError,match='UNITS-REQUEST-NONFINITE'):
            convert_quantities_to_canonical([{'id':'bad','value':value,'unit':'Pa','dimension':'stress'}])
