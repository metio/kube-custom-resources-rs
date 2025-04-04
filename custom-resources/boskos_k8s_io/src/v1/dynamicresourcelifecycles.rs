// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/boskos/boskos.k8s.io/v1/dynamicresourcelifecycles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "boskos.k8s.io", version = "v1", kind = "DRLCObject", plural = "dynamicresourcelifecycles")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DRLCObjectSpec {
    /// Config information about how to create the object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<DRLCObjectConfig>,
    /// Lifespan of a resource, time after which the resource should be reset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifespan: Option<i64>,
    /// Maxiumum number of resources expected. This maximum may be temporarily exceeded while resources are in the process of being deleted, though this is only expected when MaxCount is lowered.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "max-count")]
    pub max_count: Option<i32>,
    /// Minimum number of resources to be used as a buffer. Resources in the process of being deleted and cleaned up are included in this count.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "min-count")]
    pub min_count: Option<i32>,
    /// Define the resource needs to create the object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Config information about how to create the object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DRLCObjectConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The dynamic resource type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

