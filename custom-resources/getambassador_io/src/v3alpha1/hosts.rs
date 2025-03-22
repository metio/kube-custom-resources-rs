// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/hosts.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// HostSpec defines the desired state of Host
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "Host", plural = "hosts")]
#[kube(namespaced)]
#[kube(status = "HostStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HostSpec {
    /// Specifies whether/who to talk ACME with to automatically manage the $tlsSecret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acmeProvider")]
    pub acme_provider: Option<HostAcmeProvider>,
    /// Common to all Ambassador objects (and optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    /// Hostname by which the Ambassador can be reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Selector for Mappings we'll associate with this Host. At the moment, Selector and
    /// MappingSelector are synonyms, but that will change soon.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mappingSelector")]
    pub mapping_selector: Option<HostMappingSelector>,
    /// Configuration for the Preview URL feature of Service Preview. Defaults to preview URLs not enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "previewUrl")]
    pub preview_url: Option<HostPreviewUrl>,
    /// Request policy definition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestPolicy")]
    pub request_policy: Option<HostRequestPolicy>,
    /// DEPRECATED: Selector by which we can find further configuration. Use MappingSelector instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<HostSelector>,
    /// TLS configuration.  It is not valid to specify both
    /// `tlsContext` and `tls`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<HostTls>,
    /// Name of the TLSContext the Host resource is linked with.
    /// It is not valid to specify both `tlsContext` and `tls`.
    /// 
    /// Note that this is a native-Kubernetes-style core.v1.LocalObjectReference, not
    /// an Ambassador-style `{name}.{namespace}` string.  Because we're opinionated, it
    /// does not support referencing a Secret in another namespace (because most native
    /// Kubernetes resources don't support that), but if we ever abandon that opinion
    /// and decide to support non-local references it, it would be by adding a
    /// `namespace:` field by changing it from a core.v1.LocalObjectReference to a
    /// core.v1.SecretReference, not by adopting the `{name}.{namespace}` notation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsContext")]
    pub tls_context: Option<HostTlsContext>,
    /// Name of the Kubernetes secret into which to save generated
    /// certificates.  If ACME is enabled (see $acmeProvider), then the
    /// default is $hostname; otherwise the default is "".  If the value
    /// is "", then we do not do TLS for this Host.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<HostTlsSecret>,
}

/// Specifies whether/who to talk ACME with to automatically manage the $tlsSecret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostAcmeProvider {
    /// Specifies who to talk ACME with to get certs. Defaults to Let's
    /// Encrypt; if "none" (case-insensitive), do not try to do ACME for
    /// this Host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Specifies the Kubernetes Secret to use to store the private key of the ACME
    /// account (essentially, where to store the auto-generated password for the
    /// auto-created ACME account).  You should not normally need to set this--the
    /// default value is based on a combination of the ACME authority being registered
    /// wit and the email address associated with the account.
    /// 
    /// Note that this is a native-Kubernetes-style core.v1.LocalObjectReference, not
    /// an Ambassador-style `{name}.{namespace}` string.  Because we're opinionated, it
    /// does not support referencing a Secret in another namespace (because most native
    /// Kubernetes resources don't support that), but if we ever abandon that opinion
    /// and decide to support non-local references it, it would be by adding a
    /// `namespace:` field by changing it from a core.v1.LocalObjectReference to a
    /// core.v1.SecretReference, not by adopting the `{name}.{namespace}` notation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKeySecret")]
    pub private_key_secret: Option<HostAcmeProviderPrivateKeySecret>,
    /// This is normally set automatically
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration: Option<String>,
}

/// Specifies the Kubernetes Secret to use to store the private key of the ACME
/// account (essentially, where to store the auto-generated password for the
/// auto-created ACME account).  You should not normally need to set this--the
/// default value is based on a combination of the ACME authority being registered
/// wit and the email address associated with the account.
/// 
/// Note that this is a native-Kubernetes-style core.v1.LocalObjectReference, not
/// an Ambassador-style `{name}.{namespace}` string.  Because we're opinionated, it
/// does not support referencing a Secret in another namespace (because most native
/// Kubernetes resources don't support that), but if we ever abandon that opinion
/// and decide to support non-local references it, it would be by adding a
/// `namespace:` field by changing it from a core.v1.LocalObjectReference to a
/// core.v1.SecretReference, not by adopting the `{name}.{namespace}` notation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostAcmeProviderPrivateKeySecret {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Selector for Mappings we'll associate with this Host. At the moment, Selector and
/// MappingSelector are synonyms, but that will change soon.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostMappingSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<HostMappingSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostMappingSelectorMatchExpressions {
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

/// Configuration for the Preview URL feature of Service Preview. Defaults to preview URLs not enabled.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostPreviewUrl {
    /// Is the Preview URL feature enabled?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// What type of Preview URL is allowed?
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HostPreviewUrlType>,
}

/// Configuration for the Preview URL feature of Service Preview. Defaults to preview URLs not enabled.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostPreviewUrlType {
    Path,
}

/// Request policy definition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostRequestPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<HostRequestPolicyInsecure>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostRequestPolicyInsecure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<HostRequestPolicyInsecureAction>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalPort")]
    pub additional_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostRequestPolicyInsecureAction {
    Redirect,
    Reject,
    Route,
}

/// DEPRECATED: Selector by which we can find further configuration. Use MappingSelector instead.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<HostSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostSelectorMatchExpressions {
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

/// TLS configuration.  It is not valid to specify both
/// `tlsContext` and `tls`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostTls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpn_protocols: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert_chain_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_chain_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crl_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecdh_curves: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tls_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_cleartext_from: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
}

/// Name of the TLSContext the Host resource is linked with.
/// It is not valid to specify both `tlsContext` and `tls`.
/// 
/// Note that this is a native-Kubernetes-style core.v1.LocalObjectReference, not
/// an Ambassador-style `{name}.{namespace}` string.  Because we're opinionated, it
/// does not support referencing a Secret in another namespace (because most native
/// Kubernetes resources don't support that), but if we ever abandon that opinion
/// and decide to support non-local references it, it would be by adding a
/// `namespace:` field by changing it from a core.v1.LocalObjectReference to a
/// core.v1.SecretReference, not by adopting the `{name}.{namespace}` notation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostTlsContext {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Name of the Kubernetes secret into which to save generated
/// certificates.  If ACME is enabled (see $acmeProvider), then the
/// default is $hostname; otherwise the default is "".  If the value
/// is "", then we do not do TLS for this Host.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostTlsSecret {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// HostStatus defines the observed state of Host
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorBackoff")]
    pub error_backoff: Option<String>,
    /// errorReason, errorTimestamp, and errorBackoff are valid when state==Error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorReason")]
    pub error_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorTimestamp")]
    pub error_timestamp: Option<String>,
    /// phaseCompleted and phasePending are valid when state==Pending or
    /// state==Error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "phaseCompleted")]
    pub phase_completed: Option<HostStatusPhaseCompleted>,
    /// phaseCompleted and phasePending are valid when state==Pending or
    /// state==Error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "phasePending")]
    pub phase_pending: Option<HostStatusPhasePending>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<HostStatusState>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsCertificateSource")]
    pub tls_certificate_source: Option<HostStatusTlsCertificateSource>,
}

/// HostStatus defines the observed state of Host
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostStatusPhaseCompleted {
    #[serde(rename = "NA")]
    Na,
    DefaultsFilled,
    #[serde(rename = "ACMEUserPrivateKeyCreated")]
    AcmeUserPrivateKeyCreated,
    #[serde(rename = "ACMEUserRegistered")]
    AcmeUserRegistered,
    #[serde(rename = "ACMECertificateChallenge")]
    AcmeCertificateChallenge,
}

/// HostStatus defines the observed state of Host
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostStatusPhasePending {
    #[serde(rename = "NA")]
    Na,
    DefaultsFilled,
    #[serde(rename = "ACMEUserPrivateKeyCreated")]
    AcmeUserPrivateKeyCreated,
    #[serde(rename = "ACMEUserRegistered")]
    AcmeUserRegistered,
    #[serde(rename = "ACMECertificateChallenge")]
    AcmeCertificateChallenge,
}

/// HostStatus defines the observed state of Host
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostStatusState {
    Initial,
    Pending,
    Ready,
    Error,
}

/// HostStatus defines the observed state of Host
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HostStatusTlsCertificateSource {
    Unknown,
    None,
    Other,
    #[serde(rename = "ACME")]
    Acme,
}

