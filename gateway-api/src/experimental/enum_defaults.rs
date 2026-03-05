// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
pub mod prelude {

    pub use super::super::backendtlspolicies::*;
    pub use super::super::gatewayclasses::*;
    pub use super::super::gateways::*;
    pub use super::super::grpcroutes::*;
    pub use super::super::httproutes::*;
    pub use super::super::listenersets::*;
    pub use super::super::referencegrants::*;
    pub use super::super::tcproutes::*;
    pub use super::super::tlsroutes::*;
    pub use super::super::udproutes::*;

    pub use super::super::common::*;
}
use prelude::*;
impl Default for AllowedRoutesNamespacesFrom {
    fn default() -> Self {
        AllowedRoutesNamespacesFrom::Same
    }
}

impl Default for BackendTlsPolicyValidationSubjectAltNamesType {
    fn default() -> Self {
        BackendTlsPolicyValidationSubjectAltNamesType::Hostname
    }
}

impl Default for CookieConfigLifetimeType {
    fn default() -> Self {
        CookieConfigLifetimeType::Session
    }
}

impl Default for ExternalAuthProtocol {
    fn default() -> Self {
        ExternalAuthProtocol::Http
    }
}

impl Default for GRPCFilterType {
    fn default() -> Self {
        GRPCFilterType::RequestHeaderModifier
    }
}

impl Default for GatewayAllowedListenersNamespacesFrom {
    fn default() -> Self {
        GatewayAllowedListenersNamespacesFrom::Same
    }
}

impl Default for HTTPFilterType {
    fn default() -> Self {
        HTTPFilterType::RequestHeaderModifier
    }
}

impl Default for HTTPMethodMatch {
    fn default() -> Self {
        HTTPMethodMatch::Get
    }
}

impl Default for HeaderMatchType {
    fn default() -> Self {
        HeaderMatchType::Exact
    }
}

impl Default for HttpRouteRulesMatchesPathType {
    fn default() -> Self {
        HttpRouteRulesMatchesPathType::Exact
    }
}

impl Default for RedirectStatusCode {
    fn default() -> Self {
        RedirectStatusCode::r#_301
    }
}

impl Default for RequestOperationType {
    fn default() -> Self {
        RequestOperationType::ReplaceFullPath
    }
}

impl Default for RequestRedirectScheme {
    fn default() -> Self {
        RequestRedirectScheme::Https
    }
}

impl Default for SessionPersistenceType {
    fn default() -> Self {
        SessionPersistenceType::Cookie
    }
}

impl Default for TlsMode {
    fn default() -> Self {
        TlsMode::Terminate
    }
}

impl Default for TlsValidationMode {
    fn default() -> Self {
        TlsValidationMode::AllowValidOnly
    }
}

use crate::experimental::backendtlspolicies::BackendTlsPolicyValidationSubjectAltNamesType;
