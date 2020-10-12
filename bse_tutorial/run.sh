#!/bin/bash
source env_setup.sh

ddlog -i bse.dl &&
(cd bse_ddlog && cargo build --release) &&
./bse_ddlog/target/release/bse_cli < bse.dat
