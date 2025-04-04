// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/volcano-sh/volcano/nodeinfo.volcano.sh/v1alpha1/numatopologies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Specification of the numa information of the worker node
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "nodeinfo.volcano.sh", version = "v1alpha1", kind = "Numatopology", plural = "numatopologies")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NumatopologySpec {
    /// Specifies the cpu topology info
    /// Key is cpu id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuDetail")]
    pub cpu_detail: Option<BTreeMap<String, NumatopologyCpuDetail>>,
    /// Specifies the numa info for the resource
    /// Key is resource name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numares: Option<BTreeMap<String, NumatopologyNumares>>,
    /// Specifies the policy of the manager
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<BTreeMap<String, String>>,
    /// Specifies the reserved resource of the node
    /// Key is resource name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resReserved")]
    pub res_reserved: Option<BTreeMap<String, String>>,
}

/// Specifies the cpu topology info
/// Key is cpu id
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NumatopologyCpuDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socket: Option<i64>,
}

/// Specifies the numa info for the resource
/// Key is resource name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NumatopologyNumares {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

