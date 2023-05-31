#!/usr/bin/env python3

import sys

import json
import pathlib
import subprocess

REPO_DIR = pathlib.Path(__file__).parent.parent
GENERATED_DIR = REPO_DIR / 'assets' / 'generated'
PATCH_PATH = GENERATED_DIR / 'default.zpf'
OOT_DIR = pathlib.Path.home() / 'games' / 'zelda' / 'oot'
BASE_ROM_PATH = OOT_DIR / 'oot-ntscu-1.0.n64' #TODO test with compressed base rom
PY_REPO_DIR = pathlib.Path.home() / 'git' / 'github.com' / 'OoTRandomizer' / 'OoT-Randomizer' / 'main'

GENERATED_DIR.mkdir(parents=True, exist_ok=True)
with BASE_ROM_PATH.open('rb') as base_rom:
    with PATCH_PATH.open('wb') as patch_file:
        subprocess.run(['cargo', 'run'], stdin=base_rom, stdout=patch_file, check=True)
subprocess.run([sys.executable, str(PY_REPO_DIR / 'OoTRandomizer.py'), '--settings=-'], input=json.dumps({ #TODO pull Python randomizer first
    'generate_from_file': True,
    'rom': str(BASE_ROM_PATH),
    'patch_file': str(PATCH_PATH),
    'output_dir': str(GENERATED_DIR),
}), encoding='utf-8', check=True)
