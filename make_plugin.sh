#!/bin/sh

target="$(./targets.sh native)"

rustc -o obj/$target/liblrs_core_plugin.so src/core_plugin/lib.rs

for t in $(./targets.sh all); do
    if [[ ! -e obj/$t/liblrs_core_plugin.so ]]; then
        ln -s ../$target/liblrs_core_plugin.so obj/$t/liblrs_core_plugin.so
    fi
done
