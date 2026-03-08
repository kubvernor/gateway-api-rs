use std::{collections::BTreeMap, env};

use codegen::{Enum, Function, Scope, Variant};

fn main() {
    let task = env::args().nth(1);

    match task.as_deref() {
        Some("gen_enum_defaults") => gen_enum_defaults().unwrap(),
        Some("gen_condition_constants") => {
            let extension = env::args().nth(2);
            gen_condition_constants(extension).unwrap()
        }
        _ => print_help(),
    }
}

fn print_help() {
    eprintln!(
        "Tasks:

gen_enum_defaults generates Default trait impls for enums
gen_constants generates constants used for Conditions
"
    )
}

type DynError = Box<dyn std::error::Error>;

fn gen_condition_constants(extension: Option<String>) -> Result<(), DynError> {
    let mut scope = Scope::new();
    match extension {
        None => {
            let gateway_class_condition_types = env::var("GATEWAY_CLASS_CONDITION_CONSTANTS")?;
            let gateway_class_reason_types = env::var("GATEWAY_CLASS_REASON_CONSTANTS")?;
            let gateway_condition_types = env::var("GATEWAY_CONDITION_CONSTANTS")?;
            let gateway_reason_types = env::var("GATEWAY_REASON_CONSTANTS")?;
            let listener_condition_types = env::var("LISTENER_CONDITION_CONSTANTS")?;
            let listener_reason_types = env::var("LISTENER_REASON_CONSTANTS")?;
            let route_condition_types = env::var("ROUTE_CONDITION_CONSTANTS")?;
            let route_reason_types = env::var("ROUTE_REASON_CONSTANTS")?;

            gen_const_enums(&mut scope, gateway_class_condition_types);
            gen_const_enums(&mut scope, gateway_class_reason_types);
            gen_const_enums(&mut scope, gateway_condition_types);
            gen_const_enums(&mut scope, gateway_reason_types);
            gen_const_enums(&mut scope, listener_condition_types);
            gen_const_enums(&mut scope, listener_reason_types);
            gen_const_enums(&mut scope, route_condition_types);
            gen_const_enums(&mut scope, route_reason_types);
        }
        Some(extension) if extension == "inference" => {
            let inference_extension_failure_types =
                env::var("EXT_INFERENCE_FAILURE_MODE_CONSTANTS")?;

            gen_const_enums(&mut scope, inference_extension_failure_types);
        }
        Some(extension) => println!("{} is not a supported extension", extension),
    }

    println!("{}", gen_generated_file_warning());
    println!("{}", scope.to_string());
    Ok(())
}

fn gen_const_enums(scope: &mut Scope, constants: String) {
    let enum_type_and_variants: Vec<&str> = constants.split('=').collect();
    let enum_type = enum_type_and_variants[0];
    let variants: Vec<&str> = enum_type_and_variants[1].split(',').collect();

    let mut enumeration = Enum::new(enum_type);
    enumeration.derive("Debug");
    enumeration.derive("PartialEq");
    enumeration.derive("Eq");
    enumeration.vis("pub");

    for variant in variants {
        let var = Variant::new(variant);
        enumeration.push_variant(var);
    }
    scope.push_enum(enumeration);

    gen_display_impl(scope, enum_type);
}

fn gen_display_impl(scope: &mut Scope, ty: &str) {
    let mut func = Function::new("fmt".to_string());
    func.arg_ref_self();
    func.arg("f", "&mut std::fmt::Formatter");
    func.ret("std::fmt::Result");
    func.line("write!(f, \"{:?}\", self)");
    scope
        .new_impl(ty)
        .impl_trait("std::fmt::Display")
        .push_fn(func);
}

fn gen_enum_defaults() -> Result<(), DynError> {
    // GATEWAY_API_ENUMS provides the enum names along with their default variant to be used in the
    // generated Default impl. For eg: GATEWAY_API_ENUMS=enum1=default1,enum2=default2.
    let gw_api_experimental = env::var("GATEWAY_API_EXPERIMENTAL");
    let gw_api = env::var("GATEWAY_API").unwrap_or("true".to_owned());
    let gw_api_inference = env::var("GATEWAY_API_INFERENCE");
    let gw_api_reduced = env::var("GATEWAY_API_REDUCED");
    let gw_api_enums = env::var("GATEWAY_API_ENUMS")?;
    let inference_enums = env::var("GATEWAY_API_INFERENCE_ENUMS");

    eprintln!(
        "Inputs: standard {gw_api} experimental {gw_api_experimental:?} inference {gw_api_inference:?} {gw_api_enums:?} {inference_enums:?} "
    );
    let enums_with_defaults = get_enums_with_defaults_map(gw_api_enums);
    let mut scope = Scope::new();
    let mut inference_scope = Scope::new();

    for (e, d) in enums_with_defaults? {
        // The `fn default()` function.
        let mut func = Function::new("default".to_string());
        func.ret("Self").line(format!("{}::{}", e, d));

        // The impl Default for <enum> implementation.
        scope
            .new_impl(e.as_str())
            .impl_trait("Default")
            .push_fn(func);
    }

    if let Ok(inference_enums) = inference_enums.map(|s| {
        if s.is_empty() {
            Ok(BTreeMap::new())
        } else {
            get_enums_with_defaults_map(s)
        }
    }) {
        for (e, d) in inference_enums? {
            // The `fn default()` function.
            let mut func = Function::new("default".to_string());
            func.ret("Self").line(format!("{}::{}", e, d));

            // The impl Default for <enum> implementation.
            inference_scope
                .new_impl(e.as_str())
                .impl_trait("Default")
                .push_fn(func);
        }
    }

    println!("{}", gen_generated_file_warning());

    let gw_api = gw_api.parse::<bool>().map_err(|e| e.to_string())?;

    let gw_api_experimental = gw_api_experimental
        .map_err(|e| e.to_string())
        .and_then(|s| s.parse::<bool>().map_err(|e| e.to_string()));
    let gw_api_inference = gw_api_inference
        .map_err(|e| e.to_string())
        .and_then(|s| s.parse::<bool>().map_err(|e| e.to_string()));
    let gw_api_reduced = gw_api_reduced
        .map_err(|e| e.to_string())
        .and_then(|s| s.parse::<bool>().map_err(|e| e.to_string()));

    let standard_only = "
            pub use super::super::backendtlspolicies::*;
            pub use super::super::gatewayclasses::*;
            pub use super::super::gateways::*;
            pub use super::super::grpcroutes::*;
            pub use super::super::httproutes::*;
            pub use super::super::listenersets::*;
            pub use super::super::referencegrants::*;
            pub use super::super::tlsroutes::*;
            ";

    let standard_inference = "
            pub use super::super::inferencepools::*;
        ";

    let experimental = "
            pub use super::super::backendtlspolicies::*;
            pub use super::super::gatewayclasses::*;
            pub use super::super::gateways::*;
            pub use super::super::grpcroutes::*;
            pub use super::super::httproutes::*;
            pub use super::super::listenersets::*;
            pub use super::super::referencegrants::*;
            pub use super::super::tlsroutes::*;
            pub use super::super::tcproutes::*;
            pub use super::super::udproutes::*;
           ";

    let experimental_inference = "
            pub use super::super::inferencepools::*;
            pub use super::super::inferenceobjectives::*;
           ";

    let reduced = "
            pub use super::super::common::*;
           ";

    println!(r#"#[allow(unused_imports)]"#);
    println!("pub mod prelude {{");
    match (gw_api, gw_api_inference) {
        (true, Ok(true)) => {
            if let Ok(true) = gw_api_experimental {
                println!("{experimental}");
                println!("{experimental_inference}");
            } else {
                println!("{standard_only}");
                println!("{standard_inference}");
            }
        }

        (true, _) => {
            if let Ok(true) = gw_api_experimental {
                println!("{experimental}");
            } else {
                println!("{standard_only}");
            }
        }
        (false, Ok(true)) => {
            if let Ok(true) = gw_api_experimental {
                println!("{experimental_inference}");
            } else {
                println!("{standard_inference}");
            }
        }
        (false, _) => {}
    }
    if let Ok(true) = gw_api_reduced {
        println!("{reduced}");
    }
    println!("}}\nuse prelude::*;");

    println!("{}", scope.to_string());
    println!("{}", inference_scope.to_string());
    Ok(())
}

fn gen_generated_file_warning() -> String {
    "// WARNING: generated file - manual changes will be overriden\n".into()
}

fn get_enums_with_defaults_map(env_var_val: String) -> Result<BTreeMap<String, String>, String> {
    use std::fs::read_to_string;
    let mut enums_with_defaults = BTreeMap::new();
    read_to_string(env_var_val)
        .map_err(|e| e.to_string())?
        .lines()
        .for_each(|enum_with_default| {
            if enum_with_default.len() > 1 && enum_with_default.contains("=") {
                let enum_and_default: Vec<&str> = enum_with_default.split('=').collect();
                enums_with_defaults.insert(
                    enum_and_default[0].to_string(),
                    enum_and_default[1].to_string(),
                );
            }
        });

    eprintln!("Enums with defaults: {:#?}", &enums_with_defaults);

    Ok(enums_with_defaults)
}
