// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;
/// Spec defines the desired state of the InferencePool.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "inference.networking.k8s.io",
    version = "v1",
    kind = "InferencePool",
    plural = "inferencepools"
)]
#[kube(namespaced)]
#[kube(status = "InferencePoolStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct InferencePoolSpec {
    /// EndpointPickerRef is a reference to the Endpoint Picker extension and its
    /// associated configuration.
    #[serde(rename = "endpointPickerRef")]
    pub endpoint_picker_ref: InferencePoolEndpointPickerRef,
    /// Selector determines which Pods are members of this inference pool.
    /// It matches Pods by their labels only within the same namespace; cross-namespace
    /// selection is not supported.
    ///
    /// The structure of this LabelSelector is intentionally simple to be compatible
    /// with Kubernetes Service selectors, as some implementations may translate
    /// this configuration into a Service resource.
    pub selector: InferencePoolSelector,
    /// TargetPorts defines a list of ports that are exposed by this InferencePool.
    /// Every port will be treated as a distinctive endpoint by EPP,
    /// addressable as a 'podIP:portNumber' combination.
    #[serde(rename = "targetPorts")]
    pub target_ports: Vec<InferencePoolEndpointPickerRefPort>,
}
/// EndpointPickerRef is a reference to the Endpoint Picker extension and its
/// associated configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolEndpointPickerRef {
    /// FailureMode configures how the parent handles the case when the Endpoint Picker extension
    /// is non-responsive. When unspecified, defaults to "FailClose".
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureMode"
    )]
    pub failure_mode: Option<InferencePoolEndpointPickerRefFailureMode>,
    /// Group is the group of the referent API object. When unspecified, the default value
    /// is "", representing the Core API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent.
    ///
    /// Required if the referent is ambiguous, e.g. service with multiple ports.
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations MUST NOT
    /// support ExternalName Services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent API object.
    pub name: String,
    /// Port is the port of the Endpoint Picker extension service.
    ///
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<InferencePoolEndpointPickerRefPort>,
}
/// EndpointPickerRef is a reference to the Endpoint Picker extension and its
/// associated configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum InferencePoolEndpointPickerRefFailureMode {
    FailOpen,
    FailClose,
}
/// Selector determines which Pods are members of this inference pool.
/// It matches Pods by their labels only within the same namespace; cross-namespace
/// selection is not supported.
///
/// The structure of this LabelSelector is intentionally simple to be compatible
/// with Kubernetes Service selectors, as some implementations may translate
/// this configuration into a Service resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolSelector {
    /// MatchLabels contains a set of required {key,value} pairs.
    /// An object must match every label in this map to be selected.
    /// The matching logic is an AND operation on all entries.
    #[serde(rename = "matchLabels")]
    pub match_labels: BTreeMap<String, String>,
}
/// Status defines the observed state of the InferencePool.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolStatus {
    /// Parents is a list of parent resources, typically Gateways, that are associated with
    /// the InferencePool, and the status of the InferencePool with respect to each parent.
    ///
    /// A controller that manages the InferencePool, must add an entry for each parent it manages
    /// and remove the parent entry when the controller no longer considers the InferencePool to
    /// be associated with that parent.
    ///
    /// A maximum of 32 parents will be represented in this list. When the list is empty,
    /// it indicates that the InferencePool is not associated with any parents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<InferencePoolStatusParents>>,
}
/// ParentStatus defines the observed state of InferencePool from a Parent, i.e. Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolStatusParents {
    /// Conditions is a list of status conditions that provide information about the observed
    /// state of the InferencePool. This field is required to be set by the controller that
    /// manages the InferencePool.
    ///
    /// Supported condition types are:
    ///
    /// * "Accepted"
    /// * "ResolvedRefs"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ControllerName is a domain/path string that indicates the name of the controller that
    /// wrote this status. This corresponds with the GatewayClass controllerName field when the
    /// parentRef references a Gateway kind.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are valid Kubernetes names:
    ///
    ///  <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names>
    ///
    /// Controllers MAY populate this field when writing status. When populating this field, controllers
    /// should ensure that entries to status populated with their ControllerName are cleaned up when they
    /// are no longer necessary.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "controllerName"
    )]
    pub controller_name: Option<String>,
    /// ParentRef is used to identify the parent resource that this status
    /// is associated with. It is used to match the InferencePool with the parent
    /// resource, such as a Gateway.
    #[serde(rename = "parentRef")]
    pub parent_ref: Reference,
}
