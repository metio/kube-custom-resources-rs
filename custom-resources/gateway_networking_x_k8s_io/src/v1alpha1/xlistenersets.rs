// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/gateway-api/gateway.networking.x-k8s.io/v1alpha1/xlistenersets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec defines the desired state of ListenerSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "gateway.networking.x-k8s.io", version = "v1alpha1", kind = "XListenerSet", plural = "xlistenersets")]
#[kube(namespaced)]
#[kube(status = "XListenerSetStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct XListenerSetSpec {
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
    /// 3. ListenerSet ordered alphabetically by “{namespace}/{name}”.
    /// 
    /// An implementation MAY reject listeners by setting the ListenerEntryStatus
    /// `Accepted`` condition to False with the Reason `TooManyListeners`
    /// 
    /// If a listener has a conflict, this will be reported in the
    /// Status.ListenerEntryStatus setting the `Conflicted` condition to True.
    /// 
    /// Implementations SHOULD be cautious about what information from the
    /// parent or siblings are reported to avoid accidentally leaking
    /// sensitive information that the child would not otherwise have access
    /// to. This can include contents of secrets etc.
    pub listeners: Vec<XListenerSetListeners>,
    /// ParentRef references the Gateway that the listeners are attached to.
    #[serde(rename = "parentRef")]
    pub parent_ref: XListenerSetParentRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListeners {
    /// AllowedRoutes defines the types of routes that MAY be attached to a
    /// Listener and the trusted namespaces where those Route resources MAY be
    /// present.
    /// 
    /// Although a client request may match multiple route rules, only one rule
    /// may ultimately receive the request. Matching precedence MUST be
    /// determined in order of the following criteria:
    /// 
    /// * The most specific match as defined by the Route type.
    /// * The oldest Route based on creation timestamp. For example, a Route with
    ///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
    ///   a Route with a creation timestamp of "2020-09-08 01:02:04".
    /// * If everything else is equivalent, the Route appearing first in
    ///   alphabetical order (namespace/name) should be given precedence. For
    ///   example, foo/bar is given precedence over foo/baz.
    /// 
    /// All valid rules within a Route attached to this Listener should be
    /// implemented. Invalid Route rules can be ignored (sometimes that will mean
    /// the full Route). If a Route rule transitions from valid to invalid,
    /// support for that Route rule should be dropped to ensure consistency. For
    /// example, even if a filter specified by a Route rule is invalid, the rest
    /// of the rules within that Route should still be supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<XListenerSetListenersAllowedRoutes>,
    /// Hostname specifies the virtual hostname to match for protocol types that
    /// define this concept. When unspecified, all hostnames are matched. This
    /// field is ignored for protocols that don't require hostname based
    /// matching.
    /// 
    /// Implementations MUST apply Hostname matching appropriately for each of
    /// the following protocols:
    /// 
    /// * TLS: The Listener Hostname MUST match the SNI.
    /// * HTTP: The Listener Hostname MUST match the Host header of the request.
    /// * HTTPS: The Listener Hostname SHOULD match at both the TLS and HTTP
    ///   protocol layers as described above. If an implementation does not
    ///   ensure that both the SNI and Host header match the Listener hostname,
    ///   it MUST clearly document that.
    /// 
    /// For HTTPRoute and TLSRoute resources, there is an interaction with the
    /// `spec.hostnames` array. When both listener and route specify hostnames,
    /// there MUST be an intersection between the values for a Route to be
    /// accepted. For more information, refer to the Route specific Hostnames
    /// documentation.
    /// 
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Name is the name of the Listener. This name MUST be unique within a
    /// ListenerSet.
    /// 
    /// Name is not required to be unique across a Gateway and ListenerSets.
    /// Routes can attach to a Listener by having a ListenerSet as a parentRef
    /// and setting the SectionName
    pub name: String,
    /// Port is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    pub port: i32,
    /// Protocol specifies the network protocol this listener expects to receive.
    pub protocol: String,
    /// TLS is the TLS configuration for the Listener. This field is required if
    /// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
    /// if the Protocol field is "HTTP", "TCP", or "UDP".
    /// 
    /// The association of SNIs to Certificate defined in GatewayTLSConfig is
    /// defined based on the Hostname field for this listener.
    /// 
    /// The GatewayClass MUST use the longest matching SNI out of all
    /// available certificates for any TLS handshake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<XListenerSetListenersTls>,
}

/// AllowedRoutes defines the types of routes that MAY be attached to a
/// Listener and the trusted namespaces where those Route resources MAY be
/// present.
/// 
/// Although a client request may match multiple route rules, only one rule
/// may ultimately receive the request. Matching precedence MUST be
/// determined in order of the following criteria:
/// 
/// * The most specific match as defined by the Route type.
/// * The oldest Route based on creation timestamp. For example, a Route with
///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
///   a Route with a creation timestamp of "2020-09-08 01:02:04".
/// * If everything else is equivalent, the Route appearing first in
///   alphabetical order (namespace/name) should be given precedence. For
///   example, foo/bar is given precedence over foo/baz.
/// 
/// All valid rules within a Route attached to this Listener should be
/// implemented. Invalid Route rules can be ignored (sometimes that will mean
/// the full Route). If a Route rule transitions from valid to invalid,
/// support for that Route rule should be dropped to ensure consistency. For
/// example, even if a filter specified by a Route rule is invalid, the rest
/// of the rules within that Route should still be supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersAllowedRoutes {
    /// Kinds specifies the groups and kinds of Routes that are allowed to bind
    /// to this Gateway Listener. When unspecified or empty, the kinds of Routes
    /// selected are determined using the Listener protocol.
    /// 
    /// A RouteGroupKind MUST correspond to kinds of Routes that are compatible
    /// with the application protocol specified in the Listener's Protocol field.
    /// If an implementation does not support or recognize this resource type, it
    /// MUST set the "ResolvedRefs" condition to False for this Listener with the
    /// "InvalidRouteKinds" reason.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<XListenerSetListenersAllowedRoutesKinds>>,
    /// Namespaces indicates namespaces from which Routes may be attached to this
    /// Listener. This is restricted to the namespace of this Gateway by default.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<XListenerSetListenersAllowedRoutesNamespaces>,
}

/// RouteGroupKind indicates the group and kind of a Route resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersAllowedRoutesKinds {
    /// Group is the group of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the Route.
    pub kind: String,
}

/// Namespaces indicates namespaces from which Routes may be attached to this
/// Listener. This is restricted to the namespace of this Gateway by default.
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersAllowedRoutesNamespaces {
    /// From indicates where Routes will be selected for this Gateway. Possible
    /// values are:
    /// 
    /// * All: Routes in all namespaces may be used by this Gateway.
    /// * Selector: Routes in namespaces selected by the selector may be used by
    ///   this Gateway.
    /// * Same: Only Routes in the same namespace may be used by this Gateway.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<XListenerSetListenersAllowedRoutesNamespacesFrom>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only Routes in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<XListenerSetListenersAllowedRoutesNamespacesSelector>,
}

/// Namespaces indicates namespaces from which Routes may be attached to this
/// Listener. This is restricted to the namespace of this Gateway by default.
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum XListenerSetListenersAllowedRoutesNamespacesFrom {
    All,
    Selector,
    Same,
}

/// Selector must be specified when From is set to "Selector". In that case,
/// only Routes in Namespaces matching this Selector will be selected by this
/// Gateway. This field is ignored for other values of "From".
/// 
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersAllowedRoutesNamespacesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<XListenerSetListenersAllowedRoutesNamespacesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersAllowedRoutesNamespacesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TLS is the TLS configuration for the Listener. This field is required if
/// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
/// if the Protocol field is "HTTP", "TCP", or "UDP".
/// 
/// The association of SNIs to Certificate defined in GatewayTLSConfig is
/// defined based on the Hostname field for this listener.
/// 
/// The GatewayClass MUST use the longest matching SNI out of all
/// available certificates for any TLS handshake.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersTls {
    /// CertificateRefs contains a series of references to Kubernetes objects that
    /// contains TLS certificates and private keys. These certificates are used to
    /// establish a TLS handshake for requests that match the hostname of the
    /// associated listener.
    /// 
    /// A single CertificateRef to a Kubernetes Secret has "Core" support.
    /// Implementations MAY choose to support attaching multiple certificates to
    /// a Listener, but this behavior is implementation-specific.
    /// 
    /// References to a resource in different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    /// 
    /// This field is required to have at least one element when the mode is set
    /// to "Terminate" (default) and is optional otherwise.
    /// 
    /// CertificateRefs can reference to standard Kubernetes resources, i.e.
    /// Secret, or implementation-specific custom resources.
    /// 
    /// Support: Core - A single reference to a Kubernetes Secret of type kubernetes.io/tls
    /// 
    /// Support: Implementation-specific (More than one reference or other resource types)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateRefs")]
    pub certificate_refs: Option<Vec<XListenerSetListenersTlsCertificateRefs>>,
    /// FrontendValidation holds configuration information for validating the frontend (client).
    /// Setting this field will require clients to send a client certificate
    /// required for validation during the TLS handshake. In browsers this may result in a dialog appearing
    /// that requests a user to specify the client certificate.
    /// The maximum depth of a certificate chain accepted in verification is Implementation specific.
    /// 
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frontendValidation")]
    pub frontend_validation: Option<XListenerSetListenersTlsFrontendValidation>,
    /// Mode defines the TLS behavior for the TLS session initiated by the client.
    /// There are two possible modes:
    /// 
    /// - Terminate: The TLS session between the downstream client and the
    ///   Gateway is terminated at the Gateway. This mode requires certificates
    ///   to be specified in some way, such as populating the certificateRefs
    ///   field.
    /// - Passthrough: The TLS session is NOT terminated by the Gateway. This
    ///   implies that the Gateway can't decipher the TLS stream except for
    ///   the ClientHello message of the TLS protocol. The certificateRefs field
    ///   is ignored in this mode.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<XListenerSetListenersTlsMode>,
    /// Options are a list of key/value pairs to enable extended TLS
    /// configuration for each implementation. For example, configuring the
    /// minimum TLS version or supported cipher suites.
    /// 
    /// A set of common keys MAY be defined by the API in the future. To avoid
    /// any ambiguity, implementation-specific definitions MUST use
    /// domain-prefixed names, such as `example.com/my-custom-option`.
    /// Un-prefixed names are reserved for key names defined by Gateway API.
    /// 
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
}

/// SecretObjectReference identifies an API object including its namespace,
/// defaulting to Secret.
/// 
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
/// 
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersTlsCertificateRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "Secret".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    /// 
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// FrontendValidation holds configuration information for validating the frontend (client).
/// Setting this field will require clients to send a client certificate
/// required for validation during the TLS handshake. In browsers this may result in a dialog appearing
/// that requests a user to specify the client certificate.
/// The maximum depth of a certificate chain accepted in verification is Implementation specific.
/// 
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersTlsFrontendValidation {
    /// CACertificateRefs contains one or more references to
    /// Kubernetes objects that contain TLS certificates of
    /// the Certificate Authorities that can be used
    /// as a trust anchor to validate the certificates presented by the client.
    /// 
    /// A single CA certificate reference to a Kubernetes ConfigMap
    /// has "Core" support.
    /// Implementations MAY choose to support attaching multiple CA certificates to
    /// a Listener, but this behavior is implementation-specific.
    /// 
    /// Support: Core - A single reference to a Kubernetes ConfigMap
    /// with the CA certificate in a key named `ca.crt`.
    /// 
    /// Support: Implementation-specific (More than one reference, or other kinds
    /// of resources).
    /// 
    /// References to a resource in a different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateRefs")]
    pub ca_certificate_refs: Option<Vec<XListenerSetListenersTlsFrontendValidationCaCertificateRefs>>,
}

/// ObjectReference identifies an API object including its namespace.
/// 
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
/// 
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetListenersTlsFrontendValidationCaCertificateRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When set to the empty string, core API group is inferred.
    pub group: String,
    /// Kind is kind of the referent. For example "ConfigMap" or "Service".
    pub kind: String,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    /// 
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// TLS is the TLS configuration for the Listener. This field is required if
/// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
/// if the Protocol field is "HTTP", "TCP", or "UDP".
/// 
/// The association of SNIs to Certificate defined in GatewayTLSConfig is
/// defined based on the Hostname field for this listener.
/// 
/// The GatewayClass MUST use the longest matching SNI out of all
/// available certificates for any TLS handshake.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum XListenerSetListenersTlsMode {
    Terminate,
    Passthrough,
}

/// ParentRef references the Gateway that the listeners are attached to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetParentRef {
    /// Group is the group of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "Gateway".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referent.  If not present,
    /// the namespace of the referent is assumed to be the same as
    /// the namespace of the referring object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Status defines the current state of ListenerSet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetStatus {
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
    pub listeners: Option<Vec<XListenerSetStatusListeners>>,
}

/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetStatusListeners {
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
    /// for Listeners with condition Accepted: false and MUST count successfully
    /// attached Routes that may themselves have Accepted: false conditions.
    /// 
    /// Uses for this field include troubleshooting Route attachment and
    /// measuring blast radius/impact of changes to a Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<Condition>,
    /// Name is the name of the Listener that this status corresponds to.
    pub name: String,
    /// Port is the network port the listener is configured to listen on.
    pub port: i32,
    /// SupportedKinds is the list indicating the Kinds supported by this
    /// listener. This MUST represent the kinds an implementation supports for
    /// that Listener configuration.
    /// 
    /// If kinds are specified in Spec that are not supported, they MUST NOT
    /// appear in this list and an implementation MUST set the "ResolvedRefs"
    /// condition to "False" with the "InvalidRouteKinds" reason. If both valid
    /// and invalid Route kinds are specified, the implementation MUST
    /// reference the valid Route kinds that have been specified.
    #[serde(rename = "supportedKinds")]
    pub supported_kinds: Vec<XListenerSetStatusListenersSupportedKinds>,
}

/// RouteGroupKind indicates the group and kind of a Route resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct XListenerSetStatusListenersSupportedKinds {
    /// Group is the group of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the Route.
    pub kind: String,
}

