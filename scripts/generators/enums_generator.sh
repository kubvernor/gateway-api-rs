#!/bin/bash
set -eou pipefail

export GATEWAY_API_ENUMS=$1
export GATEWAY_API_INFERENCE_ENUMS=$2
export GATEWAY_API_EXPERIMENTAL=false
cargo xtask gen_enum_defaults > $APIS_DIR/standard/enum_defaults.rs
echo "pub mod enum_defaults;" >> $APIS_DIR/standard/mod.rs
sort -ur $APIS_DIR/standard/mod.rs -o $APIS_DIR/standard/mod.rs

export GATEWAY_API_EXPERIMENTAL=true

export GATEWAY_API_ENUMS=$3
export GATEWAY_API_INFERENCE_ENUMS=$4

cargo xtask gen_enum_defaults > $APIS_DIR/experimental/enum_defaults.rs
echo "pub mod enum_defaults;" >> $APIS_DIR/experimental/mod.rs
sort -ur $APIS_DIR/experimental/mod.rs -o $APIS_DIR/experimental/mod.rs
