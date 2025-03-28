// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/otterize/helm-charts/k8s.otterize.com/v1alpha2/protectedservices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ProtectedServiceSpec defines the desired state of ProtectedService
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.otterize.com", version = "v1alpha2", kind = "ProtectedService", plural = "protectedservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ProtectedServiceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ProtectedServiceStatus defines the observed state of ProtectedService
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProtectedServiceStatus {
}

