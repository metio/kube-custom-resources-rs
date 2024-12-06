// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/security-profiles-operator/security-profiles-operator.x-k8s.io/v1alpha1/securityprofilenodestatuses.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security-profiles-operator.x-k8s.io", version = "v1alpha1", kind = "SecurityProfileNodeStatus", plural = "securityprofilenodestatuses")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SecurityProfileNodeStatusSpec {
}

