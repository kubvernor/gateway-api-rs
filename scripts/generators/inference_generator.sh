#!/bin/bash

# ------------------------------------------------------------------------------
# This script will automatically generate API updates for new Inference Extension
# releases. Update the $INFERENCE_EXT_VERSION to the new release version before
# executing.
#
# This script requires kopium, which can be installed with:
#
#   cargo install kopium
#
# See: https://github.com/kube-rs/kopium
# ------------------------------------------------------------------------------
set -euo pipefail


echo " **** Inference Extension Processing Starts **** "
INFERENCE_API_DIR=$APIS_DIR
INFERENCE_EXT_VERSION="v1.3.0"
REQUIRED_KOPIUM_VERSION="0.22.5"
KOPIUM_VERSION=$(kopium --version 2>/dev/null | grep -oP 'kopium \K[0-9]+\.[0-9]+\.[0-9]+' || echo "")
echo "Using Inference Extension version ${INFERENCE_EXT_VERSION}"

INFERENCE_EXT_STANDARD_APIS=(
    inferencepools
)

INFERENCE_EXT_EXPERIMENTAL_APIS=(
    inferencepools
    inferenceobjectives
    inferencemodelrewrites
    inferencepoolimports
)



mkdir -p $APIS_DIR/standard/
mkdir -p $APIS_DIR/experimental/


for API in "${INFERENCE_EXT_STANDARD_APIS[@]}"
do
    echo "generating inference extension standard api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api-inference-extension/${INFERENCE_EXT_VERSION}/config/crd/bases/inference.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $INFERENCE_API_DIR/standard/${API}.rs
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $INFERENCE_API_DIR/standard/${API}.rs
    echo "pub mod ${API};" >> $INFERENCE_API_DIR/standard/mod.rs
done

for API in "${INFERENCE_EXT_EXPERIMENTAL_APIS[@]}"
do
    echo "generating inference extension experimental api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api-inference-extension/${INFERENCE_EXT_VERSION}/config/crd/bases/inference.networking.x-k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $INFERENCE_API_DIR/experimental/${API}.rs
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $INFERENCE_API_DIR/experimental/${API}.rs
    echo "pub mod ${API};" >> $INFERENCE_API_DIR/experimental/mod.rs
done
