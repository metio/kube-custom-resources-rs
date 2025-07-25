// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api/ipam.cluster.x-k8s.io/v1beta2/ipaddressclaims.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// spec is the desired state of IPAddressClaim.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ipam.cluster.x-k8s.io", version = "v1beta2", kind = "IPAddressClaim", plural = "ipaddressclaims")]
#[kube(namespaced)]
#[kube(status = "IPAddressClaimStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IPAddressClaimSpec {
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// poolRef is a reference to the pool from which an IP address should be created.
    #[serde(rename = "poolRef")]
    pub pool_ref: IPAddressClaimPoolRef,
}

/// poolRef is a reference to the pool from which an IP address should be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimPoolRef {
    /// apiGroup of the IPPool.
    /// apiGroup must be fully qualified domain name.
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    /// kind of the IPPool.
    /// kind must consist of alphanumeric characters or '-', start with an alphabetic character, and end with an alphanumeric character.
    pub kind: String,
    /// name of the IPPool.
    /// name must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character.
    pub name: String,
}

/// status is the observed state of IPAddressClaim.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatus {
    /// addressRef is a reference to the address that was created for this claim.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressRef")]
    pub address_ref: Option<IPAddressClaimStatusAddressRef>,
    /// conditions represents the observations of a IPAddressClaim's current state.
    /// Known condition types are Ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// deprecated groups all the status fields that are deprecated and will be removed when all the nested field are removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<IPAddressClaimStatusDeprecated>,
}

/// addressRef is a reference to the address that was created for this claim.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusAddressRef {
    /// name of the IPAddress.
    /// name must consist of lower case alphanumeric characters, '-' or '.', and must start and end with an alphanumeric character.
    pub name: String,
}

/// deprecated groups all the status fields that are deprecated and will be removed when all the nested field are removed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusDeprecated {
    /// v1beta1 groups all the status fields that are deprecated and will be removed when support for v1beta1 will be dropped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v1beta1: Option<IPAddressClaimStatusDeprecatedV1beta1>,
}

/// v1beta1 groups all the status fields that are deprecated and will be removed when support for v1beta1 will be dropped.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusDeprecatedV1beta1 {
    /// conditions summarises the current state of the IPAddressClaim
    /// 
    /// Deprecated: This field is deprecated and is going to be removed when support for v1beta1 will be dropped. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

