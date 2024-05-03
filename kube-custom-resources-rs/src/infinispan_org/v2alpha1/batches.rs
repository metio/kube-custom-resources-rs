// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/infinispan/infinispan-operator/infinispan.org/v2alpha1/batches.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BatchSpec defines the desired state of Batch
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infinispan.org", version = "v2alpha1", kind = "Batch", plural = "batches")]
#[kube(namespaced)]
#[kube(status = "BatchStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BatchSpec {
    /// Infinispan cluster name
    pub cluster: String,
    /// Batch string to be executed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Name of the ConfigMap containing the batch and resource files to be executed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMap")]
    pub config_map: Option<String>,
}

/// BatchStatus defines the observed state of Batch
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BatchStatus {
    /// The UUID of the Infinispan instance that the Batch is associated with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterUID")]
    pub cluster_uid: Option<String>,
    /// Current phase of the batch operation
    pub phase: String,
    /// The reason for any batch related failures
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

