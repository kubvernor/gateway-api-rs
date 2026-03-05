#!/bin/bash
export APIS_DIR='gateway-api-inference-extension/src'
export GATEWAY_API=false
export GATEWAY_API_INFERENCE=true
export GATEWAY_API_REDUCED=false

echo "GENERATING GATEWAY API INFERENCE EXTENSION ONLY"

rm -rf $APIS_DIR/standard/
rm -rf $APIS_DIR/experimental/
cat << EOF > $APIS_DIR/mod.rs
pub mod experimental;
pub mod standard;
EOF

mkdir -p $APIS_DIR/standard/
mkdir -p $APIS_DIR/experimental/

echo "// WARNING! generated file do not edit" > $APIS_DIR/standard/mod.rs
echo "// WARNING! generated file do not edit" > $APIS_DIR/experimental/mod.rs


./scripts/generators/inference_generator.sh

export GATEWAY_API_REDUCED=true
./scripts/generators/enums_generator.sh ./scripts/generators/empy.txt ./scripts/generators/standard_inference_enum_names.txt ./scripts/generators/empty.txt ./scripts/generators/experimental_inference_enum_names.txt


./scripts/generators/reducer.sh standard
./scripts/generators/reducer.sh experimental


./scripts/generators/enums_generator.sh ./scripts/generators/empty.txt ./scripts/generators/reduced_standard_inference_enum_names.txt ./scripts/generators/empty.txt ./scripts/generators/reduced_experimental_inference_enum_names.txt

sed -i '/#\[kube(status = "InferenceModelRewriteStatus")\]/c\#\[kube(status = "InferenceStatus")\]' $APIS_DIR/experimental/inferencemodelrewrites.rs
sed -i '/#\[kube(status = "InferenceObjectiveStatus")\]/c\#\[kube(status = "InferenceStatus")\]' $APIS_DIR/experimental/inferenceobjectives.rs

cargo fmt


echo "GENERATING GATEWAY API INFERENCE EXTENSION ONLY Done"
