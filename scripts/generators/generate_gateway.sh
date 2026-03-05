#!/bin/bash
export APIS_DIR='gateway-api/src'
export GATEWAY_API=true
export GATEWAY_API_INFERENCE=false
export GATEWAY_API_REDUCED=false

rm -rf $APIS_DIR/standard/
rm -rf $APIS_DIR/experimental/
cat << EOF > $APIS_DIR/mod.rs
pub mod experimental;
pub mod standard;
EOF

mkdir -p $APIS_DIR/standard/
mkdir -p $APIS_DIR/experimental/


./scripts/generators/gateway_generator.sh
./scripts/generators/enums_generator.sh ./scripts/generators/standard_enum_names.txt ./scripts/generators/empty.txt ./scripts/generators/experimental_enum_names.txt ./scripts/generators/empty.txt
echo "use crate::backendtlspolicies::BackendTlsPolicyValidationSubjectAltNamesType;" >> $APIS_DIR/standard/enum_defaults.rs


./scripts/generators/reducer.sh standard
./scripts/generators/reducer.sh experimental

export GATEWAY_API_REDUCED=true
./scripts/generators/enums_generator.sh ./scripts/generators/reduced_standard_enum_names.txt ./scripts/generators/empty.txt ./scripts/generators/reduced_experimental_enum_names.txt ./scripts/generators/empty.txt


echo "use crate::backendtlspolicies::BackendTlsPolicyValidationSubjectAltNamesType;" >> $APIS_DIR/standard/enum_defaults.rs

sed -i '/#\[kube(status = "GrpcRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/standard/grpcroutes.rs
sed -i '/#\[kube(status = "HttpRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/standard/httproutes.rs
sed -i '/#\[kube(status = "TlsRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/standard/tlsroutes.rs


echo "use crate::experimental::backendtlspolicies::BackendTlsPolicyValidationSubjectAltNamesType;" >> $APIS_DIR/experimental/enum_defaults.rs

# sed -i '/#\[kube(status = "GrpcRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/grpcroutes.rs
# sed -i '/#\[kube(status = "HttpRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/httproutes.rs
# sed -i '/#\[kube(status = "TlsRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/tlsroutes.rs
# sed -i '/#\[kube(status = "UdpRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/udproutes.rs
# sed -i '/#\[kube(status = "TcpRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/tcproutes.rs


cargo fmt
