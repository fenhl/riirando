#!/usr/local/bin/zsh

set -e

cargo run < ~/games/zelda/oot/oot-ntscu-1.0.z64 > default.zpf
python3 /opt/git/github.com/TestRunnerSRL/OoT-Randomizer/master/OoTRandomizer.py --settings=- <<EOF
{
    "generate_from_file": true,
    "rom": "/Users/fenhl/games/zelda/oot/oot-ntscu-1.0.z64",
    "patch_file": "/Users/fenhl/games/zelda/oot/riirando/default.zpf"
}
EOF
rm default.zpf
