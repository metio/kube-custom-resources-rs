// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/kubernetes-ingress/externaldns.nginx.org/v1/dnsendpoints.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DNSEndpointSpec holds information about endpoints.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "externaldns.nginx.org", version = "v1", kind = "DNSEndpoint", plural = "dnsendpoints")]
#[kube(namespaced)]
#[kube(status = "DNSEndpointStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DNSEndpointSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<DNSEndpointEndpoints>>,
}

/// Endpoint describes DNS Endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSEndpointEndpoints {
    /// The hostname for the DNS record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Labels stores labels defined for the Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ProviderSpecific stores provider specific config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerSpecific")]
    pub provider_specific: Option<Vec<DNSEndpointEndpointsProviderSpecific>>,
    /// TTL for the record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTTL")]
    pub record_ttl: Option<i64>,
    /// RecordType type of record, e.g. CNAME, A, SRV, TXT, MX
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordType")]
    pub record_type: Option<String>,
    /// The targets the DNS service points to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// ProviderSpecificProperty represents provider specific config property.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSEndpointEndpointsProviderSpecific {
    /// Name of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value of the property
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DNSEndpointStatus represents generation observed by the external dns controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSEndpointStatus {
    /// The generation observed by by the external-dns controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

