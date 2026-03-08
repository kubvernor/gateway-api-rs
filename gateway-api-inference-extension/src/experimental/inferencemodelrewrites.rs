// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// InferenceModelRewriteSpec defines the desired state of InferenceModelRewrite.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "inference.networking.x-k8s.io",
    version = "v1alpha2",
    kind = "InferenceModelRewrite",
    plural = "inferencemodelrewrites"
)]
#[kube(namespaced)]
#[kube(status = "InferenceStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct InferenceModelRewriteSpec {
    /// PoolRef is a reference to the inference pool.
    #[serde(rename = "poolRef")]
    pub pool_ref: InferencePoolRef,
    pub rules: Vec<InferenceModelRewriteRules>,
}
/// InferenceModelRewriteRule defines the match criteria and corresponding action.
/// For details on how precedence is determined across multiple rules and
/// InferenceModelRewrite resources, see the "Precedence and Conflict Resolution"
/// section in InferenceModelRewriteSpec.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceModelRewriteRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<InferenceModelRewriteRulesMatches>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<InferenceModelRewriteRulesTargets>>,
}
/// Match defines the criteria for matching the LLM requests.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceModelRewriteRulesMatches {
    /// Model specifies the criteria for matching the 'model' field
    /// within the JSON request body.
    pub model: InferenceModelRewriteRulesMatchesModel,
}
/// Model specifies the criteria for matching the 'model' field
/// within the JSON request body.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceModelRewriteRulesMatchesModel {
    /// Type specifies the kind of string matching to use.
    /// Supported value is "Exact". Defaults to "Exact".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InferenceModelRewriteRulesMatchesModelType>,
    /// Value is the model name string to match against.
    pub value: String,
}
/// Model specifies the criteria for matching the 'model' field
/// within the JSON request body.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum InferenceModelRewriteRulesMatchesModelType {
    Exact,
}
/// TargetModel defines a weighted model destination for traffic distribution.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceModelRewriteRulesTargets {
    #[serde(rename = "modelRewrite")]
    pub model_rewrite: String,
    /// (The following comment is copied from the original targetModel)
    /// Weight is used to determine the proportion of traffic that should be
    /// sent to this model when multiple target models are specified.
    ///
    /// Weight defines the proportion of requests forwarded to the specified
    /// model. This is computed as weight/(sum of all weights in this
    /// TargetModels list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If a weight is set for any targetModel, it must be set for all targetModels.
    /// Conversely weights are optional, so long as ALL targetModels do not specify a weight.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
