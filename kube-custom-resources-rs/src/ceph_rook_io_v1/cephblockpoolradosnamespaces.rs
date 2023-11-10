// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephblockpoolradosnamespaces.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec represents the specification of a Ceph BlockPool Rados Namespace
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBlockPoolRadosNamespace", plural = "cephblockpoolradosnamespaces")]
#[kube(namespaced)]
#[kube(status = "CephBlockPoolRadosNamespaceStatus")]
#[kube(schema = "disabled")]
pub struct CephBlockPoolRadosNamespaceSpec {
    /// BlockPoolName is the name of Ceph BlockPool. Typically it's the name of the CephBlockPool CR.
    #[serde(rename = "blockPoolName")]
    pub block_pool_name: String,
}

/// Status represents the status of a CephBlockPool Rados Namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolRadosNamespaceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<BTreeMap<String, String>>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

