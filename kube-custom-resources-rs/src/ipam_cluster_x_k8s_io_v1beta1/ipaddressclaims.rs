// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/ipam.cluster.x-k8s.io/v1beta1/ipaddressclaims.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// IPAddressClaimSpec is the desired state of an IPAddressClaim.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ipam.cluster.x-k8s.io", version = "v1beta1", kind = "IPAddressClaim", plural = "ipaddressclaims")]
#[kube(namespaced)]
#[kube(status = "IPAddressClaimStatus")]
#[kube(schema = "disabled")]
pub struct IPAddressClaimSpec {
    /// PoolRef is a reference to the pool from which an IP address should be created.
    #[serde(rename = "poolRef")]
    pub pool_ref: IPAddressClaimPoolRef,
}

/// PoolRef is a reference to the pool from which an IP address should be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimPoolRef {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
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
    /// AddressRef is a reference to the address that was created for this claim.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressRef")]
    pub address_ref: Option<IPAddressClaimStatusAddressRef>,
    /// Conditions summarises the current state of the IPAddressClaim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<IPAddressClaimStatusConditions>>,
}

/// AddressRef is a reference to the address that was created for this claim.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusAddressRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Condition defines an observation of a Cluster API resource operational state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IPAddressClaimStatusConditions {
    /// Last time the condition transitioned from one status to another. This should be when the underlying condition changed. If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// A human readable message indicating details about the transition. This field may be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition in CamelCase. The specific API may choose whether or not this field is considered a guaranteed API. This field may not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Severity provides an explicit classification of Reason code, so the users or machines can immediately understand the current situation and act accordingly. The Severity field MUST be set only when Status=False.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of condition in CamelCase or in foo.example.com/CamelCase. Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important.
    #[serde(rename = "type")]
    pub r#type: String,
}

