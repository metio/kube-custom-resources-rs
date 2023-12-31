// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/eks-controller/services.k8s.aws/v1alpha1/fieldexports.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// FieldExportSpec defines the desired state of the FieldExport.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "services.k8s.aws", version = "v1alpha1", kind = "FieldExport", plural = "fieldexports")]
#[kube(namespaced)]
#[kube(status = "FieldExportStatus")]
#[kube(schema = "disabled")]
pub struct FieldExportSpec {
    /// ResourceFieldSelector provides the values necessary to identify an individual field on an individual K8s resource.
    pub from: FieldExportFrom,
    /// FieldExportTarget provides the values necessary to identify the output path for a field export.
    pub to: FieldExportTo,
}

/// ResourceFieldSelector provides the values necessary to identify an individual field on an individual K8s resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldExportFrom {
    pub path: String,
    /// NamespacedResource provides all the values necessary to identify an ACK resource of a given type (within the same namespace as the custom resource containing this type).
    pub resource: FieldExportFromResource,
}

/// NamespacedResource provides all the values necessary to identify an ACK resource of a given type (within the same namespace as the custom resource containing this type).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldExportFromResource {
    pub group: String,
    pub kind: String,
    pub name: String,
}

/// FieldExportTarget provides the values necessary to identify the output path for a field export.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldExportTo {
    /// Key overrides the default value (`<namespace>.<FieldExport-resource-name>`) for the FieldExport target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// FieldExportOutputType represents all types that can be produced by a field export operation
    pub kind: FieldExportToKind,
    pub name: String,
    /// Namespace is marked as optional, so we cannot compose `NamespacedName`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// FieldExportTarget provides the values necessary to identify the output path for a field export.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FieldExportToKind {
    #[serde(rename = "configmap")]
    Configmap,
    #[serde(rename = "secret")]
    Secret,
}

/// FieldExportStatus defines the observed status of the FieldExport.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldExportStatus {
    /// A collection of `ackv1alpha1.Condition` objects that describe the various recoverable states of the field CR
    pub conditions: Vec<FieldExportStatusConditions>,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FieldExportStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

