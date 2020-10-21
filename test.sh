#!/bin/bash

ulimit -c unlimited

set -e

rustc -V

for i in {1..1000}
do
    echo $i
    rustc --crate-type rlib foo.rs --emit=metadata
done
tput bel
