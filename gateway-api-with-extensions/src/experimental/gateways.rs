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
/// Spec defines the desired state of Gateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "Gateway",
    plural = "gateways"
)]
#[kube(namespaced)]
#[kube(status = "GatewayStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct GatewaySpec {
    /// Addresses requested for this Gateway. This is optional and behavior can
    /// depend on the implementation. If a value is set in the spec and the
    /// requested address is invalid or unavailable, the implementation MUST
    /// indicate this in an associated entry in GatewayStatus.Conditions.
    ///
    /// The Addresses field represents a request for the address(es) on the
    /// "outside of the Gateway", that traffic bound for this Gateway will use.
    /// This could be the IP address or hostname of an external load balancer or
    /// other networking infrastructure, or some other address that traffic will
    /// be sent to.
    ///
    /// If no Addresses are specified, the implementation MAY schedule the
    /// Gateway in an implementation-specific manner, assigning an appropriate
    /// set of Addresses.
    ///
    /// The implementation MUST bind all Listeners to every GatewayAddress that
    /// it assigns to the Gateway and add a corresponding entry in
    /// GatewayStatus.Addresses.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayAddresses>>,
    /// AllowedListeners defines which ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedListeners"
    )]
    pub allowed_listeners: Option<GatewayAllowedListeners>,
    /// DefaultScope, when set, configures the Gateway as a default Gateway,
    /// meaning it will dynamically and implicitly have Routes (e.g. HTTPRoute)
    /// attached to it, according to the scope configured here.
    ///
    /// If unset (the default) or set to None, the Gateway will not act as a
    /// default Gateway; if set, the Gateway will claim any Route with a
    /// matching scope set in its UseDefaultGateway field, subject to the usual
    /// rules about which routes the Gateway can attach to.
    ///
    /// Think carefully before using this functionality! While the normal rules
    /// about which Route can apply are still enforced, it is simply easier for
    /// the wrong Route to be accidentally attached to this Gateway in this
    /// configuration. If the Gateway operator is not also the operator in
    /// control of the scope (e.g. namespace) with tight controls and checks on
    /// what kind of workloads and Routes get added in that scope, we strongly
    /// recommend not using this just because it seems convenient, and instead
    /// stick to direct Route attachment.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "defaultScope"
    )]
    pub default_scope: Option<DefaultGateway>,
    /// GatewayClassName used for this Gateway. This is the name of a
    /// GatewayClass resource.
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: String,
    /// Infrastructure defines infrastructure level attributes about this Gateway instance.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<GatewayInfrastructure>,
    /// Listeners associated with this Gateway. Listeners define
    /// logical endpoints that are bound on this Gateway's addresses.
    /// At least one Listener MUST be specified.
    ///
    /// ## Distinct Listeners
    ///
    /// Each Listener in a set of Listeners (for example, in a single Gateway)
    /// MUST be _distinct_, in that a traffic flow MUST be able to be assigned to
    /// exactly one listener. (This section uses "set of Listeners" rather than
    /// "Listeners in a single Gateway" because implementations MAY merge configuration
    /// from multiple Gateways onto a single data plane, and these rules _also_
    /// apply in that case).
    ///
    /// Practically, this means that each listener in a set MUST have a unique
    /// combination of Port, Protocol, and, if supported by the protocol, Hostname.
    ///
    /// Some combinations of port, protocol, and TLS settings are considered
    /// Core support and MUST be supported by implementations based on the objects
    /// they support:
    ///
    /// HTTPRoute
    ///
    /// 1. HTTPRoute, Port: 80, Protocol: HTTP
    /// 2. HTTPRoute, Port: 443, Protocol: HTTPS, TLS Mode: Terminate, TLS keypair provided
    ///
    /// TLSRoute
    ///
    /// 1. TLSRoute, Port: 443, Protocol: TLS, TLS Mode: Passthrough
    ///
    /// "Distinct" Listeners have the following property:
    ///
    /// **The implementation can match inbound requests to a single distinct
    /// Listener**.
    ///
    /// When multiple Listeners share values for fields (for
    /// example, two Listeners with the same Port value), the implementation
    /// can match requests to only one of the Listeners using other
    /// Listener fields.
    ///
    /// When multiple listeners have the same value for the Protocol field, then
    /// each of the Listeners with matching Protocol values MUST have different
    /// values for other fields.
    ///
    /// The set of fields that MUST be different for a Listener differs per protocol.
    /// The following rules define the rules for what fields MUST be considered for
    /// Listeners to be distinct with each protocol currently defined in the
    /// Gateway API spec.
    ///
    /// The set of listeners that all share a protocol value MUST have _different_
    /// values for _at least one_ of these fields to be distinct:
    ///
    /// * **HTTP, HTTPS, TLS**: Port, Hostname
    /// * **TCP, UDP**: Port
    ///
    /// One **very** important rule to call out involves what happens when an
    /// implementation:
    ///
    /// * Supports TCP protocol Listeners, as well as HTTP, HTTPS, or TLS protocol
    ///   Listeners, and
    /// * sees HTTP, HTTPS, or TLS protocols with the same `port` as one with TCP
    ///   Protocol.
    ///
    /// In this case all the Listeners that share a port with the
    /// TCP Listener are not distinct and so MUST NOT be accepted.
    ///
    /// If an implementation does not support TCP Protocol Listeners, then the
    /// previous rule does not apply, and the TCP Listeners SHOULD NOT be
    /// accepted.
    ///
    /// Note that the `tls` field is not used for determining if a listener is distinct, because
    /// Listeners that _only_ differ on TLS config will still conflict in all cases.
    ///
    /// ### Listeners that are distinct only by Hostname
    ///
    /// When the Listeners are distinct based only on Hostname, inbound request
    /// hostnames MUST match from the most specific to least specific Hostname
    /// values to choose the correct Listener and its associated set of Routes.
    ///
    /// Exact matches MUST be processed before wildcard matches, and wildcard
    /// matches MUST be processed before fallback (empty Hostname value)
    /// matches. For example, `"foo.example.com"` takes precedence over
    /// `"*.example.com"`, and `"*.example.com"` takes precedence over `""`.
    ///
    /// Additionally, if there are multiple wildcard entries, more specific
    /// wildcard entries must be processed before less specific wildcard entries.
    /// For example, `"*.foo.example.com"` takes precedence over `"*.example.com"`.
    ///
    /// The precise definition here is that the higher the number of dots in the
    /// hostname to the right of the wildcard character, the higher the precedence.
    ///
    /// The wildcard character will match any number of characters _and dots_ to
    /// the left, however, so `"*.example.com"` will match both
    /// `"foo.bar.example.com"` _and_ `"bar.example.com"`.
    ///
    /// ## Handling indistinct Listeners
    ///
    /// If a set of Listeners contains Listeners that are not distinct, then those
    /// Listeners are _Conflicted_, and the implementation MUST set the "Conflicted"
    /// condition in the Listener Status to "True".
    ///
    /// The words "indistinct" and "conflicted" are considered equivalent for the
    /// purpose of this documentation.
    ///
    /// Implementations MAY choose to accept a Gateway with some Conflicted
    /// Listeners only if they only accept the partial Listener set that contains
    /// no Conflicted Listeners.
    ///
    /// Specifically, an implementation MAY accept a partial Listener set subject to
    /// the following rules:
    ///
    /// * The implementation MUST NOT pick one conflicting Listener as the winner.
    ///   ALL indistinct Listeners must not be accepted for processing.
    /// * At least one distinct Listener MUST be present, or else the Gateway effectively
    ///   contains _no_ Listeners, and must be rejected from processing as a whole.
    ///
    /// The implementation MUST set a "ListenersNotValid" condition on the
    /// Gateway Status when the Gateway contains Conflicted Listeners whether or
    /// not they accept the Gateway. That Condition SHOULD clearly
    /// indicate in the Message which Listeners are conflicted, and which are
    /// Accepted. Additionally, the Listener status for those listeners SHOULD
    /// indicate which Listeners are conflicted and not Accepted.
    ///
    /// ## General Listener behavior
    ///
    /// Note that, for all distinct Listeners, requests SHOULD match at most one Listener.
    /// For example, if Listeners are defined for "foo.example.com" and "*.example.com", a
    /// request to "foo.example.com" SHOULD only be routed using routes attached
    /// to the "foo.example.com" Listener (and not the "*.example.com" Listener).
    ///
    /// This concept is known as "Listener Isolation", and it is an Extended feature
    /// of Gateway API. Implementations that do not support Listener Isolation MUST
    /// clearly document this, and MUST NOT claim support for the
    /// `GatewayHTTPListenerIsolation` feature.
    ///
    /// Implementations that _do_ support Listener Isolation SHOULD claim support
    /// for the Extended `GatewayHTTPListenerIsolation` feature and pass the associated
    /// conformance tests.
    ///
    /// ## Compatible Listeners
    ///
    /// A Gateway's Listeners are considered _compatible_ if:
    ///
    /// 1. They are distinct.
    /// 2. The implementation can serve them in compliance with the Addresses
    ///    requirement that all Listeners are available on all assigned
    ///    addresses.
    ///
    /// Compatible combinations in Extended support are expected to vary across
    /// implementations. A combination that is compatible for one implementation
    /// may not be compatible for another.
    ///
    /// For example, an implementation that cannot serve both TCP and UDP listeners
    /// on the same address, or cannot mix HTTPS and generic TLS listens on the same port
    /// would not consider those cases compatible, even though they are distinct.
    ///
    /// Implementations MAY merge separate Gateways onto a single set of
    /// Addresses if all Listeners across all Gateways are compatible.
    ///
    /// In a future release the MinItems=1 requirement MAY be dropped.
    ///
    /// Support: Core
    pub listeners: Vec<Listeners>,
    /// TLS specifies frontend and backend tls configuration for entire gateway.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayTls>,
}
/// GatewaySpecAddress describes an address that can be bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayAddresses {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// When a value is unspecified, an implementation SHOULD automatically
    /// assign an address matching the requested type if possible.
    ///
    /// If an implementation does not support an empty value, they MUST set the
    /// "Programmed" condition in status to False with a reason of "AddressNotAssigned".
    ///
    /// Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// AllowedListeners defines which ListenerSets can be attached to this Gateway.
/// The default value is to allow no ListenerSets.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayAllowedListeners {
    /// Namespaces defines which namespaces ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<GatewayAllowedListenersNamespaces>,
}
/// Namespaces defines which namespaces ListenerSets can be attached to this Gateway.
/// The default value is to allow no ListenerSets.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayAllowedListenersNamespaces {
    /// From indicates where ListenerSets can attach to this Gateway. Possible
    /// values are:
    ///
    /// * Same: Only ListenerSets in the same namespace may be attached to this Gateway.
    /// * Selector: ListenerSets in namespaces selected by the selector may be attached to this Gateway.
    /// * All: ListenerSets in all namespaces may be attached to this Gateway.
    /// * None: Only listeners defined in the Gateway's spec are allowed
    ///
    /// The default value None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<GatewayAllowedListenersNamespacesFrom>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only ListenerSets in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<NamespaceSelector>,
}
/// Namespaces defines which namespaces ListenerSets can be attached to this Gateway.
/// The default value is to allow no ListenerSets.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GatewayAllowedListenersNamespacesFrom {
    All,
    Selector,
    Same,
    None,
}
/// Infrastructure defines infrastructure level attributes about this Gateway instance.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayInfrastructure {
    /// Annotations that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.annotations` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "annotations" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific annotations as they see fit.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.labels` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "labels" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific labels as they see fit.
    ///
    /// If an implementation maps these labels to Pods, or any other resource that would need to be recreated when labels
    /// change, it SHOULD clearly warn about this behavior in documentation.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ParametersRef is a reference to a resource that contains the configuration
    /// parameters corresponding to the Gateway. This is optional if the
    /// controller does not require any additional configuration.
    ///
    /// This follows the same semantics as GatewayClass's `parametersRef`, but on a per-Gateway basis
    ///
    /// The Gateway's GatewayClass may provide its own `parametersRef`. When both are specified,
    /// the merging behavior is implementation specific.
    /// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
    ///
    /// If the referent cannot be found, refers to an unsupported kind, or when
    /// the data within that resource is malformed, the Gateway SHOULD be
    /// rejected with the "Accepted" status condition set to "False" and an
    /// "InvalidParameters" reason.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parametersRef"
    )]
    pub parameters_ref: Option<ExtensionParametersReference>,
}
/// TLS specifies frontend and backend tls configuration for entire gateway.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayTls {
    /// Backend describes TLS configuration for gateway when connecting
    /// to backends.
    ///
    /// Note that this contains only details for the Gateway as a TLS client,
    /// and does _not_ imply behavior about how to choose which backend should
    /// get a TLS connection. That is determined by the presence of a BackendTLSPolicy.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<GatewayTlsBackend>,
    /// Frontend describes TLS config when client connects to Gateway.
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<GatewayTlsFrontend>,
}
/// Backend describes TLS configuration for gateway when connecting
/// to backends.
///
/// Note that this contains only details for the Gateway as a TLS client,
/// and does _not_ imply behavior about how to choose which backend should
/// get a TLS connection. That is determined by the presence of a BackendTLSPolicy.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayTlsBackend {
    /// ClientCertificateRef references an object that contains a client certificate
    /// and its associated private key. It can reference standard Kubernetes resources,
    /// i.e., Secret, or implementation-specific custom resources.
    ///
    /// A ClientCertificateRef is considered invalid if:
    ///
    /// * It refers to a resource that cannot be resolved (e.g., the referenced resource
    ///   does not exist) or is misconfigured (e.g., a Secret does not contain the keys
    ///   named `tls.crt` and `tls.key`). In this case, the `ResolvedRefs` condition
    ///   on the Gateway MUST be set to False with the Reason `InvalidClientCertificateRef`
    ///   and the Message of the Condition MUST indicate why the reference is invalid.
    ///
    /// * It refers to a resource in another namespace UNLESS there is a ReferenceGrant
    ///   in the target namespace that allows the certificate to be attached.
    ///   If a ReferenceGrant does not allow this reference, the `ResolvedRefs` condition
    ///   on the Gateway MUST be set to False with the Reason `RefNotPermitted`.
    ///
    /// Implementations MAY choose to perform further validation of the certificate
    /// content (e.g., checking expiry or enforcing specific formats). In such cases,
    /// an implementation-specific Reason and Message MUST be set.
    ///
    /// Support: Core - Reference to a Kubernetes TLS Secret (with the type `kubernetes.io/tls`).
    /// Support: Implementation-specific - Other resource kinds or Secrets with a
    /// different type (e.g., `Opaque`).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientCertificateRef"
    )]
    pub client_certificate_ref: Option<Reference>,
}
/// Frontend describes TLS config when client connects to Gateway.
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayTlsFrontend {
    /// Default specifies the default client certificate validation configuration
    /// for all Listeners handling HTTPS traffic, unless a per-port configuration
    /// is defined.
    ///
    /// support: Core
    pub default: FrontendTls,
    /// PerPort specifies tls configuration assigned per port.
    /// Per port configuration is optional. Once set this configuration overrides
    /// the default configuration for all Listeners handling HTTPS traffic
    /// that match this port.
    /// Each override port requires a unique TLS configuration.
    ///
    /// support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perPort")]
    pub per_port: Option<Vec<GatewayTlsFrontendPerPort>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayTlsFrontendPerPort {
    /// The Port indicates the Port Number to which the TLS configuration will be
    /// applied. This configuration will be applied to all Listeners handling HTTPS
    /// traffic that match this port.
    ///
    /// Support: Core
    pub port: i32,
    /// TLS store the configuration that will be applied to all Listeners handling
    /// HTTPS traffic and matching given port.
    ///
    /// Support: Core
    pub tls: FrontendTls,
}
/// Status defines the current state of Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatus {
    /// Addresses lists the network addresses that have been bound to the
    /// Gateway.
    ///
    /// This list may differ from the addresses provided in the spec under some
    /// conditions:
    ///
    ///   * no addresses are specified, all addresses are dynamically assigned
    ///   * a combination of specified and dynamic addresses are assigned
    ///   * a specified address was unusable (e.g. already in use)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayStatusAddresses>>,
    /// AttachedListenerSets represents the total number of ListenerSets that have been
    /// successfully attached to this Gateway.
    ///
    /// A ListenerSet is successfully attached to a Gateway when all the following conditions are met:
    /// - The ListenerSet is selected by the Gateway's AllowedListeners field
    /// - The ListenerSet has a valid ParentRef selecting the Gateway
    /// - The ListenerSet's status has the condition "Accepted: true"
    ///
    /// Uses for this field include troubleshooting AttachedListenerSets attachment and
    /// measuring blast radius/impact of changes to a Gateway.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "attachedListenerSets"
    )]
    pub attached_listener_sets: Option<i32>,
    /// Conditions describe the current conditions of the Gateway.
    ///
    /// Implementations should prefer to express Gateway conditions
    /// using the `GatewayConditionType` and `GatewayConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe Gateway state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    /// * "Ready"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GatewayStatusListeners>>,
}
/// GatewayStatusAddress describes a network address that is bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatusAddresses {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Value of the address. The validity of the values will depend
    /// on the type and support by the controller.
    ///
    /// Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    pub value: String,
}
/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatusListeners {
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
    /// Route kinds ParentRefs fields). Listener or Route status does not impact
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
