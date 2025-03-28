// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/kubernetesserviceresolvers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// KubernetesServiceResolver tells Ambassador to use Kubernetes Service
/// resources to resolve services. It actually has no spec other than the
/// AmbassadorID.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "KubernetesServiceResolver", plural = "kubernetesserviceresolvers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KubernetesServiceResolverSpec {
    /// AmbassadorID declares which Ambassador instances should pay
    /// attention to this resource. If no value is provided, the default is:
    /// 
    /// 	ambassador_id:
    /// 	- "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
}

