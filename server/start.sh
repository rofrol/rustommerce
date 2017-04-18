#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

export PATH=~/.cargo/bin:$PATH

# https://github.com/SergioBenitez/Rocket/pull/256
# from https://github.com/SergioBenitez/Rocket/blob/v0.2.6/codegen/build.rs#L11
~/rustup-helpers/setup_specific_rust_nightly.sh 2017-04-15 2017-04-16

cargo run
