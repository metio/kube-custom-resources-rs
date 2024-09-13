// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/releases.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ReleaseSpec defines the desired state of Release.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "Release", plural = "releases")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ReleaseSpec {
    pub version: String,
}

/// ReleaseStatus defines the observed state of Release.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaseStatus {
}

