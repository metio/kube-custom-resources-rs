// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/cilium/cilium/cilium.io/v2alpha1/ciliumpodippools.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cilium.io", version = "v2alpha1", kind = "CiliumPodIPPool", plural = "ciliumpodippools")]
#[kube(schema = "disabled")]
pub struct CiliumPodIPPoolSpec {
    /// IPv4 specifies the IPv4 CIDRs and mask sizes of the pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<CiliumPodIPPoolIpv4>,
    /// IPv6 specifies the IPv6 CIDRs and mask sizes of the pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<CiliumPodIPPoolIpv6>,
}

/// IPv4 specifies the IPv4 CIDRs and mask sizes of the pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumPodIPPoolIpv4 {
    /// CIDRs is a list of IPv4 CIDRs that are part of the pool.
    pub cidrs: Vec<String>,
    /// MaskSize is the mask size of the pool.
    #[serde(rename = "maskSize")]
    pub mask_size: i64,
}

/// IPv6 specifies the IPv6 CIDRs and mask sizes of the pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CiliumPodIPPoolIpv6 {
    /// CIDRs is a list of IPv6 CIDRs that are part of the pool.
    pub cidrs: Vec<String>,
    /// MaskSize is the mask size of the pool.
    #[serde(rename = "maskSize")]
    pub mask_size: i64,
}

