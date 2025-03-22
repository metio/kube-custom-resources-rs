// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/modules.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "Module", plural = "modules")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ModuleSpec {
    /// AmbassadorID declares which Ambassador instances should pay
    /// attention to this resource. If no value is provided, the default is:
    /// 
    /// 	ambassador_id:
    /// 	- "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    pub config: BTreeMap<String, serde_json::Value>,
}

