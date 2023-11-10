// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta1/helmrepositories.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// HelmRepositorySpec defines the reference to a Helm repository.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta1", kind = "HelmRepository", plural = "helmrepositories")]
#[kube(namespaced)]
#[kube(status = "HelmRepositoryStatus")]
#[kube(schema = "disabled")]
pub struct HelmRepositorySpec {
    /// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<HelmRepositoryAccessFrom>,
    /// The interval at which to check the upstream for updates.
    pub interval: String,
    /// PassCredentials allows the credentials from the SecretRef to be passed on to a host that does not match the host as defined in URL. This may be required if the host of the advertised chart URLs in the index differ from the defined URL. Enabling this should be done with caution, as it can potentially result in credentials getting stolen in a MITM-attack.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passCredentials")]
    pub pass_credentials: Option<bool>,
    /// The name of the secret containing authentication credentials for the Helm repository. For HTTP/S basic auth the secret must contain username and password fields. For TLS the secret must contain a certFile and keyFile, and/or caFile fields.
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositoryAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies. Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<HelmRepositoryAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies. An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositoryAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// The name of the secret containing authentication credentials for the Helm repository. For HTTP/S basic auth the secret must contain username and password fields. For TLS the secret must contain a certFile and keyFile, and/or caFile fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositorySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// HelmRepositoryStatus defines the observed state of the HelmRepository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositoryStatus {
    /// Artifact represents the output of the last successful repository sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmRepositoryStatusArtifact>,
    /// Conditions holds the conditions for the HelmRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<HelmRepositoryStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositoryStatusArtifact {
    /// Checksum is the SHA256 checksum of the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of this artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    /// Path is the relative file path of this artifact.
    pub path: String,
    /// Revision is a human readable identifier traceable in the origin source system. It can be a Git commit SHA, Git tag, a Helm index timestamp, a Helm chart version, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// URL is the HTTP address of this artifact.
    pub url: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmRepositoryStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: HelmRepositoryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmRepositoryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

