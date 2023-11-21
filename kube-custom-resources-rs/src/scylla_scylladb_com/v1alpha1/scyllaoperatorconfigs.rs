// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/scylladb/scylla-operator/scylla.scylladb.com/v1alpha1/scyllaoperatorconfigs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// spec defines the desired state of the operator.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scylla.scylladb.com", version = "v1alpha1", kind = "ScyllaOperatorConfig", plural = "scyllaoperatorconfigs")]
#[kube(status = "ScyllaOperatorConfigStatus")]
#[kube(schema = "disabled")]
pub struct ScyllaOperatorConfigSpec {
    /// scyllaUtilsImage is a Scylla image used for running scylla utilities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scyllaUtilsImage")]
    pub scylla_utils_image: Option<String>,
}

/// status defines the observed state of the operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScyllaOperatorConfigStatus {
}
