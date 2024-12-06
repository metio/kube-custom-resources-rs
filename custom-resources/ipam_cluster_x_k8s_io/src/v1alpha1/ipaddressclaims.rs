// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api/ipam.cluster.x-k8s.io/v1alpha1/ipaddressclaims.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// IPAddressClaimSpec is the desired state of an IPAddressClaim.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ipam.cluster.x-k8s.io", version = "v1alpha1", kind = "IPAddressClaim", plural = "ipaddressclaims")]
#[kube(namespaced)]
#[kube(status = "IPAddressClaimStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IPAddressClaimSpec {
    /// poolRef is a reference to the pool from which an IP address should be created.
    #[serde(rename = "poolRef")]
    pub pool_ref: IPAddressClaimPoolRef,
}

/// poolRef is a reference to the pool from which an IP address should be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimPoolRef {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// IPAddressClaimStatus is the observed status of a IPAddressClaim.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatus {
    /// addressRef is a reference to the address that was created for this claim.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressRef")]
    pub address_ref: Option<IPAddressClaimStatusAddressRef>,
    /// conditions summarises the current state of the IPAddressClaim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

/// addressRef is a reference to the address that was created for this claim.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusAddressRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

