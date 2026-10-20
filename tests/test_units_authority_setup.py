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
    conftest=module(ROOT/'tests/conftest.py','units_setup_test'); monkeypatch.delenv('OPENPIPESTRESS_NO_CARGO_BUILD',raising=False)
    monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN','/configured/checked-json')
    monkeypatch.setenv('OPENPIPESTRESS_RUNNER_BIN','/configured/runner')
    monkeypatch.delenv('OPENPIPESTRESS_UNITS_BIN',raising=False)
    calls=[]; target=tmp_path/'units-authority'
    monkeypatch.setitem(sys.modules,'build_units_authority',SimpleNamespace(build=lambda:(calls.append('units') or target)))
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))
    assert calls==['units'];assert os.environ['OPENPIPESTRESS_UNITS_BIN']==str(target)

def test_controller_prepares_the_runner_even_with_existing_authorities(monkeypatch, tmp_path):
    conftest=module(ROOT/'tests/conftest.py','runner_setup_test'); monkeypatch.delenv('OPENPIPESTRESS_NO_CARGO_BUILD',raising=False)
    monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN','/configured/checked-json')
    monkeypatch.setenv('OPENPIPESTRESS_UNITS_BIN','/configured/units')
    monkeypatch.delenv('OPENPIPESTRESS_RUNNER_BIN',raising=False)
    calls=[]; target=tmp_path/'openpipestress-runner'
    monkeypatch.setitem(sys.modules,'build_runner',SimpleNamespace(build=lambda:(calls.append('runner') or target)))
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))
    assert calls==['runner'];assert os.environ['OPENPIPESTRESS_RUNNER_BIN']==str(target)

def test_workers_and_configured_controller_do_not_build(monkeypatch):
    conftest=module(ROOT/'tests/conftest.py','units_setup_worker_test'); monkeypatch.delenv('OPENPIPESTRESS_NO_CARGO_BUILD',raising=False)
    def forbidden():raise AssertionError('unexpected build')
    monkeypatch.setitem(sys.modules,'build_units_authority',SimpleNamespace(build=forbidden))
    monkeypatch.setitem(sys.modules,'build_checked_json',SimpleNamespace(build=forbidden))
    monkeypatch.setitem(sys.modules,'build_runner',SimpleNamespace(build=forbidden))
    for name in ('OPENPIPESTRESS_UNITS_BIN','OPENPIPESTRESS_CHECKED_JSON_BIN','OPENPIPESTRESS_RUNNER_BIN'):monkeypatch.delenv(name,raising=False)
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace(workerinput={})))
    monkeypatch.setenv('OPENPIPESTRESS_UNITS_BIN','/configured/units');monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN','/configured/json')
    monkeypatch.setenv('OPENPIPESTRESS_RUNNER_BIN','/configured/runner')
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))

SESSION_BINS={'OPENPIPESTRESS_CHECKED_JSON_BIN':('build_checked_json','openpipestress_jcs_ijson'),
    'OPENPIPESTRESS_UNITS_BIN':('build_units_authority','openpipestress_units'),'OPENPIPESTRESS_RUNNER_BIN':('build_runner','openpipestress-runner')}

def no_build_session(monkeypatch, tmp_path, name):
    conftest=module(ROOT/'tests/conftest.py',name)
    def forbidden():raise AssertionError('unexpected build')
    for module_name,_ in SESSION_BINS.values():
        monkeypatch.setitem(sys.modules,module_name,SimpleNamespace(build=forbidden,DEFAULT_TARGET=tmp_path/module_name))
    monkeypatch.setenv('OPENPIPESTRESS_NO_CARGO_BUILD','1'); monkeypatch.setenv('PATH',os.environ.get('PATH',''))
    return conftest

def test_no_cargo_build_uses_presets_and_existing_builds_and_refuses_cargo(monkeypatch, tmp_path):
    conftest=no_build_session(monkeypatch,tmp_path,'no_cargo_presets_test'); preset=tmp_path/'preset-json'; preset.write_text('')
    monkeypatch.setenv('OPENPIPESTRESS_CHECKED_JSON_BIN',str(preset)); monkeypatch.delenv('OPENPIPESTRESS_RUNNER_BIN',raising=False)
    built=tmp_path/'build_units_authority/release/openpipestress_units'; built.parent.mkdir(parents=True); built.write_text('')
    monkeypatch.setenv('OPENPIPESTRESS_UNITS_BIN','')
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace()))
    assert os.environ['OPENPIPESTRESS_CHECKED_JSON_BIN']==str(preset.resolve()); assert os.environ['OPENPIPESTRESS_UNITS_BIN']==str(built.resolve())
    assert 'OPENPIPESTRESS_RUNNER_BIN' not in os.environ; assert conftest._ABSENT=={'OPENPIPESTRESS_RUNNER_BIN'}
    if os.name=='posix':
        import shutil,subprocess
        cargo=shutil.which('cargo'); assert cargo and Path(cargo).parent.name.startswith('openpipestress-no-cargo-')
        refused=subprocess.run([cargo,'build'],capture_output=True,text=True); assert refused.returncode==1 and 'OPENPIPESTRESS_NO_CARGO_BUILD=1' in refused.stderr

def test_no_cargo_build_workers_record_absent_authorities_without_building(monkeypatch, tmp_path):
    conftest=no_build_session(monkeypatch,tmp_path,'no_cargo_worker_test'); path=os.environ['PATH']
    for name in SESSION_BINS:monkeypatch.delenv(name,raising=False)
    conftest.pytest_sessionstart(SimpleNamespace(config=SimpleNamespace(workerinput={})))
    assert conftest._ABSENT==set(SESSION_BINS) and os.environ['PATH']==path

def test_explicit_build_uses_locked_release_cli_and_target(monkeypatch,tmp_path):
    helper=module(ROOT/'tools/units/build_units_authority.py','units_build_test'); calls=[]
    monkeypatch.setattr(helper.subprocess,'run',lambda args,**kwargs:calls.append((args,kwargs)))
    result=helper.build(tmp_path/'target')
    assert result==(tmp_path/'target/release/openpipestress_units').resolve()
    args,options=calls[0];assert '--locked' in args and '--release' in args
    assert args[args.index('--features')+1]=='cli';assert options['cwd']==ROOT/'core/units';assert options['check'] is True

def test_explicit_runner_build_uses_locked_release_binary_and_target(monkeypatch,tmp_path):
    helper=module(ROOT/'tools/runner/build_runner.py','runner_build_test'); calls=[]
    monkeypatch.setattr(helper.subprocess,'run',lambda args,**kwargs:calls.append((args,kwargs)))
    result=helper.build(tmp_path/'target')
    assert result==(tmp_path/'target/release/openpipestress-runner').resolve()
    args,options=calls[0];assert '--locked' in args and '--release' in args
    assert args[args.index('--bin')+1]=='openpipestress-runner';assert options['cwd']==ROOT/'core/runner/headless';assert options['check'] is True

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
