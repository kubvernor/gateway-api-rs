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
/// Status defines the observed state of the InferencePoolImport.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolImportStatus {
    /// Controllers is a list of controllers that are responsible for managing the InferencePoolImport.
    pub controllers: Vec<InferencePoolImportStatusControllers>,
}
/// ImportController defines a controller that is responsible for managing the InferencePoolImport.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolImportStatusControllers {
    /// Conditions track the state of the InferencePoolImport.
    ///
    /// Known condition types are:
    ///
    ///  * "Accepted"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ExportingClusters is a list of clusters that exported the InferencePool(s) that back the
    /// InferencePoolImport. Required when the controller is responsible for CRUD'ing the InferencePoolImport
    /// from the exported InferencePool(s).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exportingClusters"
    )]
    pub exporting_clusters: Option<Vec<InferencePoolImportStatusControllersExportingClusters>>,
    /// Name is a domain/path string that indicates the name of the controller that manages the
    /// InferencePoolImport. Name corresponds to the GatewayClass controllerName field when the
    /// controller will manage parents of type "Gateway". Otherwise, the name is implementation-specific.
    ///
    /// Example: "example.net/import-controller".
    ///
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are valid Kubernetes
    /// names (<https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names).>
    ///
    /// A controller MUST populate this field when writing status and ensure that entries to status
    /// populated with their controller name are removed when they are no longer necessary.
    pub name: String,
    /// Parents is a list of parent resources, typically Gateways, that are associated with the
    /// InferencePoolImport, and the status of the InferencePoolImport with respect to each parent.
    ///
    /// Ancestor would be a more accurate name, but Parent is consistent with InferencePool terminology.
    ///
    /// Required when the controller manages the InferencePoolImport as an HTTPRoute backendRef. The controller
    /// must add an entry for each parent it manages and remove the parent entry when the controller no longer
    /// considers the InferencePoolImport to be associated with that parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<InferencePoolImportStatusControllersParents>>,
}
/// ExportingCluster defines a cluster that exported the InferencePool that backs this InferencePoolImport.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolImportStatusControllersExportingClusters {
    /// Name of the exporting cluster (must be unique within the list).
    pub name: String,
}
/// ParentStatus defines the observed state of InferencePool from a Parent, i.e. Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolImportStatusControllersParents {
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
    pub parent_ref: InferencePoolImportStatusControllersParentsParentRef,
}
