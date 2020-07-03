#!/bin/sh
set -eu
CXX=${CXX:-clang++}

${CXX} $* -Wwritable-strings -DBUILD_LINUX=1 -I. -O3 -o btime btime.c
cp btime ../btime