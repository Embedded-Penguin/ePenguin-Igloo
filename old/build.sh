#!/bin/sh

# Script assumes tools directory is in your path.
# For now, this is done by running shell.sh inside of tools directory

if [ ! -f tools/btime ]; then
    pushd tools/build-timer
    ./build.sh
    popd
fi

tools/btime --begin ePenguin-Software-Framework.aet

tools/btime --end ePenguin-Software-Framework.aet