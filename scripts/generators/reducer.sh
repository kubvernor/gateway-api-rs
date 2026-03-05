#!/bin/bash
set -eou pipefail
export RUST_LOG=info

API_TYPE=$1

echo " **** Starting Type Reducer - Collapsing Duplicative Types **** "
echo " **** Type Reducer - PHASE 1 - First Pass ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_0.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt
mv mapped_names.txt ${API_TYPE}_mapped_names_phase_1.txt
mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_1.txt
echo " **** PHASE 2 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_1.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt
mv mapped_names.txt ${API_TYPE}_mapped_names_phase_2.txt
mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_2.txt
echo " **** PHASE 3 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_2.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt
mv mapped_names.txt ${API_TYPE}_mapped_names_phase_3.txt
mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_3.txt
echo " **** PHASE 4 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_3.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt 
mv mapped_names.txt ${API_TYPE}_mapped_names_phase_4.txt
mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_4.txt
echo " **** PHASE 5 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_4.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt 
mv mapped_names.txt ${API_TYPE}_mapped_names_phase_5.txt
mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_5.txt
# echo " **** PHASE 6 ***** "
# cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE reduce --previous-pass-derived-type-names ./type-reducer/${API_TYPE}_reduced_types_pass_5.txt --current-pass-substitute-names ./type-reducer/${API_TYPE}_customized_mapped_names.txt 
# mv mapped_names.txt ${API_TYPE}_mapped_names_phase_6.txt
# mv mapped_types_to_names.txt ${API_TYPE}_mapped_types_to_names_phase_6.txt


echo " **** RENAMING PHASE ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/$API_TYPE --out-dir $APIS_DIR/$API_TYPE rename --rename-only-substitute-names ./type-reducer/${API_TYPE}_rename_only_mapped_names.txt
