// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1alpha3/vsphereclusteridentities.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha3", kind = "VSphereClusterIdentity", plural = "vsphereclusteridentities")]
#[kube(status = "VSphereClusterIdentityStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereClusterIdentitySpec {
    /// AllowedNamespaces is used to identify which namespaces are allowed to use this account.
    /// Namespaces can be selected with a label selector.
    /// If this object is nil, no namespaces will be allowed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedNamespaces")]
    pub allowed_namespaces: Option<VSphereClusterIdentityAllowedNamespaces>,
    /// SecretName references a Secret inside the controller namespace with the credentials to use
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// AllowedNamespaces is used to identify which namespaces are allowed to use this account.
/// Namespaces can be selected with a label selector.
/// If this object is nil, no namespaces will be allowed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterIdentityAllowedNamespaces {
    /// Selector is a standard Kubernetes LabelSelector. A label query over a set of resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<VSphereClusterIdentityAllowedNamespacesSelector>,
}

/// Selector is a standard Kubernetes LabelSelector. A label query over a set of resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterIdentityAllowedNamespacesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VSphereClusterIdentityAllowedNamespacesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterIdentityAllowedNamespacesSelectorMatchExpressions {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterIdentityStatus {
    /// Conditions defines current service state of the VSphereCluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

