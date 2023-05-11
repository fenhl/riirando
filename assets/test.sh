#!/usr/local/bin/zsh

set -e

mkdir -p assets/generated
cargo run < ~/games/zelda/oot/oot-ntscu-1.0.z64 > assets/generated/default.zpf
python3 /opt/git/github.com/TestRunnerSRL/OoT-Randomizer/master/OoTRandomizer.py --settings=- <<EOF
{
    "generate_from_file": true,
    "rom": "/Users/fenhl/games/zelda/oot/oot-ntscu-1.0.z64",
    "patch_file": "/Users/fenhl/games/zelda/oot/riirando/assets/generated/default.zpf",
    "output_dir": "/Users/fenhl/games/zelda/oot/riirando/assets/generated"
}
EOF
