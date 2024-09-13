// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta1/helmrepositories.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// HelmRepositorySpec defines the reference to a Helm repository.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta1", kind = "HelmRepository", plural = "helmrepositories")]
#[kube(namespaced)]
#[kube(status = "HelmRepositoryStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HelmRepositorySpec {
    /// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<HelmRepositoryAccessFrom>,
    /// The interval at which to check the upstream for updates.
    pub interval: String,
    /// PassCredentials allows the credentials from the SecretRef to be passed on to
    /// a host that does not match the host as defined in URL.
    /// This may be required if the host of the advertised chart URLs in the index
    /// differ from the defined URL.
    /// Enabling this should be done with caution, as it can potentially result in
    /// credentials getting stolen in a MITM-attack.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passCredentials")]
    pub pass_credentials: Option<bool>,
    /// The name of the secret containing authentication credentials for the Helm
    /// repository.
    /// For HTTP/S basic auth the secret must contain username and
    /// password fields.
    /// For TLS the secret must contain a certFile and keyFile, and/or
    /// caFile fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<HelmRepositorySecretRef>,
    /// This flag tells the controller to suspend the reconciliation of this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// The timeout of index downloading, defaults to 60s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// The Helm repository URL, a valid URL contains at least a protocol and host.
    pub url: String,
}

/// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmRepositoryAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies.
    /// Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<HelmRepositoryAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies.
/// An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmRepositoryAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// The name of the secret containing authentication credentials for the Helm
/// repository.
/// For HTTP/S basic auth the secret must contain username and
/// password fields.
/// For TLS the secret must contain a certFile and keyFile, and/or
/// caFile fields.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// HelmRepositoryStatus defines the observed state of the HelmRepository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmRepositoryStatus {
    /// Artifact represents the output of the last successful repository sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmRepositoryStatusArtifact>,
    /// Conditions holds the conditions for the HelmRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the download link for the last index fetched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful repository sync.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmRepositoryStatusArtifact {
    /// Checksum is the SHA256 checksum of the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of this
    /// artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Path is the relative file path of this artifact.
    pub path: String,
    /// Revision is a human readable identifier traceable in the origin source
    /// system. It can be a Git commit SHA, Git tag, a Helm index timestamp, a Helm
    /// chart version, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// URL is the HTTP address of this artifact.
    pub url: String,
}

