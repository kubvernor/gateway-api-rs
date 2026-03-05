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
/// Spec defines the desired state of ListenerSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "ListenerSet",
    plural = "listenersets"
)]
#[kube(namespaced)]
#[kube(status = "ListenerSetStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ListenerSetSpec {
    /// Listeners associated with this ListenerSet. Listeners define
    /// logical endpoints that are bound on this referenced parent Gateway's addresses.
    ///
    /// Listeners in a `Gateway` and their attached `ListenerSets` are concatenated
    /// as a list when programming the underlying infrastructure. Each listener
    /// name does not need to be unique across the Gateway and ListenerSets.
    /// See ListenerEntry.Name for more details.
    ///
    /// Implementations MUST treat the parent Gateway as having the merged
    /// list of all listeners from itself and attached ListenerSets using
    /// the following precedence:
    ///
    /// 1. "parent" Gateway
    /// 2. ListenerSet ordered by creation time (oldest first)
    /// 3. ListenerSet ordered alphabetically by "{namespace}/{name}".
    ///
    /// An implementation MAY reject listeners by setting the ListenerEntryStatus
    /// `Accepted` condition to False with the Reason `TooManyListeners`
    ///
    /// If a listener has a conflict, this will be reported in the
    /// Status.ListenerEntryStatus setting the `Conflicted` condition to True.
    ///
    /// Implementations SHOULD be cautious about what information from the
    /// parent or siblings are reported to avoid accidentally leaking
    /// sensitive information that the child would not otherwise have access
    /// to. This can include contents of secrets etc.
    pub listeners: Vec<Listeners>,
    /// ParentRef references the Gateway that the listeners are attached to.
    #[serde(rename = "parentRef")]
    pub parent_ref: Reference,
}
/// Status defines the current state of ListenerSet.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetStatus {
    /// Conditions describe the current conditions of the ListenerSet.
    ///
    /// Implementations MUST express ListenerSet conditions using the
    /// `ListenerSetConditionType` and `ListenerSetConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe ListenerSet state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerSetStatusListeners>>,
}
/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetStatusListeners {
    /// AttachedRoutes represents the total number of Routes that have been
    /// successfully attached to this Listener.
    ///
    /// Successful attachment of a Route to a Listener is based solely on the
    /// combination of the AllowedRoutes field on the corresponding Listener
    /// and the Route's ParentRefs field. A Route is successfully attached to
    /// a Listener when it is selected by the Listener's AllowedRoutes field
    /// AND the Route has a valid ParentRef selecting the whole Gateway
    /// resource or a specific Listener as a parent resource (more detail on
    /// attachment semantics can be found in the documentation on the various
    /// Route kinds ParentRefs fields). Listener status does not impact
    /// successful attachment, i.e. the AttachedRoutes field count MUST be set
    /// for Listeners, even if the Accepted condition of an individual Listener is set
    /// to "False". The AttachedRoutes number represents the number of Routes with
    /// the Accepted condition set to "True" that have been attached to this Listener.
    /// Routes with any other value for the Accepted condition MUST NOT be included
    /// in this count.
    ///
    /// Uses for this field include troubleshooting Route attachment and
    /// measuring blast radius/impact of changes to a Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<Condition>,
    /// Name is the name of the Listener that this status corresponds to.
    pub name: String,
    /// SupportedKinds is the list indicating the Kinds supported by this
    /// listener. This MUST represent the kinds supported by an implementation for
    /// that Listener configuration.
    ///
    /// If kinds are specified in Spec that are not supported, they MUST NOT
    /// appear in this list and an implementation MUST set the "ResolvedRefs"
    /// condition to "False" with the "InvalidRouteKinds" reason. If both valid
    /// and invalid Route kinds are specified, the implementation MUST
    /// reference the valid Route kinds that have been specified.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "supportedKinds"
    )]
    pub supported_kinds: Option<Vec<Kind>>,
}
