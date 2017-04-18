#!/bin/bash

rm -rf ~/rustup-helpers && \
cd ~ && \
git clone https://github.com/rofrol/rustup-helpers && \
~/rustup-helpers/setup_rust.sh
