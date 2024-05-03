// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta1/receivers.yaml --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ReceiverSpec defines the desired state of Receiver
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta1", kind = "Receiver", plural = "receivers")]
#[kube(namespaced)]
#[kube(status = "ReceiverStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ReceiverSpec {
    /// A list of events to handle,
    /// e.g. 'push' for GitHub or 'Push Hook' for GitLab.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// A list of resources to be notified about changes.
    pub resources: Vec<ReceiverResources>,
    /// Secret reference containing the token used
    /// to validate the payload authenticity
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ReceiverSecretRef>,
    /// This flag tells the controller to suspend subsequent events handling.
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Type of webhook sender, used to determine
    /// the validation procedure and payload deserialization.
    #[serde(rename = "type")]
    pub r#type: ReceiverType,
}

/// CrossNamespaceObjectReference contains enough information to let you locate the
/// typed referenced object at cluster level
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverResources {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ReceiverResourcesKind>,
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Name of the referent
    pub name: String,
    /// Namespace of the referent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// CrossNamespaceObjectReference contains enough information to let you locate the
/// typed referenced object at cluster level
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReceiverResourcesKind {
    Bucket,
    GitRepository,
    Kustomization,
    HelmRelease,
    HelmChart,
    HelmRepository,
    ImageRepository,
    ImagePolicy,
    ImageUpdateAutomation,
    #[serde(rename = "OCIRepository")]
    OciRepository,
}

/// Secret reference containing the token used
/// to validate the payload authenticity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ReceiverSpec defines the desired state of Receiver
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReceiverType {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "generic-hmac")]
    GenericHmac,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "harbor")]
    Harbor,
    #[serde(rename = "dockerhub")]
    Dockerhub,
    #[serde(rename = "quay")]
    Quay,
    #[serde(rename = "gcr")]
    Gcr,
    #[serde(rename = "nexus")]
    Nexus,
    #[serde(rename = "acr")]
    Acr,
}

/// ReceiverStatus defines the observed state of Receiver
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Generated webhook URL in the format
    /// of '/hook/sha256sum(token+name+namespace)'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

