// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Kuadrant/dns-operator/kuadrant.io/v1alpha1/dnsrecords.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DNSRecordSpec defines the desired state of DNSRecord
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuadrant.io", version = "v1alpha1", kind = "DNSRecord", plural = "dnsrecords")]
#[kube(namespaced)]
#[kube(status = "DNSRecordStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DNSRecordSpec {
    /// endpoints is a list of endpoints that will be published into the dns provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<DNSRecordEndpoints>>,
    /// HealthCheckSpec configures health checks in the DNS provider.
    /// By default this health check will be applied to each unique DNS A Record for
    /// the listeners assigned to the target gateway
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<DNSRecordHealthCheck>,
    /// ownerID is a unique string used to identify the owner of this record.
    /// If unset or set to an empty string the record UID will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// providerRef is a reference to a provider secret.
    #[serde(rename = "providerRef")]
    pub provider_ref: DNSRecordProviderRef,
    /// rootHost is the single root for all endpoints in a DNSRecord.
    /// it is expected all defined endpoints are children of or equal to this rootHost
    /// Must contain at least two groups of valid URL characters separated by a "."
    #[serde(rename = "rootHost")]
    pub root_host: String,
}

/// Endpoint is a high-level way of a connection between a service and an IP
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordEndpoints {
    /// The hostname of the DNS record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Labels stores labels defined for the Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ProviderSpecific stores provider specific config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerSpecific")]
    pub provider_specific: Option<Vec<DNSRecordEndpointsProviderSpecific>>,
    /// TTL for the record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTTL")]
    pub record_ttl: Option<i64>,
    /// RecordType type of record, e.g. CNAME, A, AAAA, SRV, TXT etc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordType")]
    pub record_type: Option<String>,
    /// Identifier to distinguish multiple records with the same name and type (e.g. Route53 records with routing policies other than 'simple')
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setIdentifier")]
    pub set_identifier: Option<String>,
    /// The targets the DNS record points to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// ProviderSpecificProperty holds the name and value of a configuration which is specific to individual DNS providers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordEndpointsProviderSpecific {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// HealthCheckSpec configures health checks in the DNS provider.
/// By default this health check will be applied to each unique DNS A Record for
/// the listeners assigned to the target gateway
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordHealthCheck {
    /// AdditionalHeadersRef refers to a secret that contains extra headers to send in the probe request, this is primarily useful if an authentication
    /// token is required by the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalHeadersRef")]
    pub additional_headers_ref: Option<DNSRecordHealthCheckAdditionalHeadersRef>,
    /// FailureThreshold is a limit of consecutive failures that must occur for a host to be considered unhealthy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i64>,
    /// Interval defines how frequently this probe should execute
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to append to the host to reach the expected health check.
    /// Must start with "?" or "/", contain only valid URL characters and end with alphanumeric char or "/". For example "/" or "/healthz" are common
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port to connect to the host on. Must be either 80, 443 or 1024-49151
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Protocol to use when connecting to the host, valid values are "HTTP" or "HTTPS"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// AdditionalHeadersRef refers to a secret that contains extra headers to send in the probe request, this is primarily useful if an authentication
/// token is required by the endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordHealthCheckAdditionalHeadersRef {
    pub name: String,
}

/// providerRef is a reference to a provider secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordProviderRef {
    pub name: String,
}

/// DNSRecordStatus defines the observed state of DNSRecord
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatus {
    /// conditions are any conditions associated with the record in the dns provider.
    /// 
    /// 
    /// If publishing the record fails, the "Failed" condition will be set with a
    /// reason and message describing the cause of the failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// DomainOwners is a list of all the owners working against the root domain of this record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainOwners")]
    pub domain_owners: Option<Vec<String>>,
    /// endpoints are the last endpoints that were successfully published to the provider zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<DNSRecordStatusEndpoints>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<DNSRecordStatusHealthCheck>,
    /// observedGeneration is the most recently observed generation of the DNSRecord.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ownerID is a unique string used to identify the owner of this record.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// QueuedAt is a time when DNS record was received for the reconciliation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queuedAt")]
    pub queued_at: Option<String>,
    /// ZoneEndpoints are all the endpoints for the DNSRecordSpec.RootHost that are present in the provider
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relatedEndpoints")]
    pub related_endpoints: Option<Vec<DNSRecordStatusRelatedEndpoints>>,
    /// ValidFor indicates duration since the last reconciliation we consider data in the record to be valid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validFor")]
    pub valid_for: Option<String>,
    /// WriteCounter represent a number of consecutive write attempts on the same generation of the record.
    /// It is being reset to 0 when the generation changes or there are no changes to write.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeCounter")]
    pub write_counter: Option<i64>,
    /// zoneDomainName is the domain name of the zone that the dns record is publishing endpoints
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneDomainName")]
    pub zone_domain_name: Option<String>,
    /// zoneID is the provider specific id to which this dns record is publishing endpoints
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneID")]
    pub zone_id: Option<String>,
}

/// Endpoint is a high-level way of a connection between a service and an IP
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusEndpoints {
    /// The hostname of the DNS record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Labels stores labels defined for the Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ProviderSpecific stores provider specific config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerSpecific")]
    pub provider_specific: Option<Vec<DNSRecordStatusEndpointsProviderSpecific>>,
    /// TTL for the record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTTL")]
    pub record_ttl: Option<i64>,
    /// RecordType type of record, e.g. CNAME, A, AAAA, SRV, TXT etc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordType")]
    pub record_type: Option<String>,
    /// Identifier to distinguish multiple records with the same name and type (e.g. Route53 records with routing policies other than 'simple')
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setIdentifier")]
    pub set_identifier: Option<String>,
    /// The targets the DNS record points to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// ProviderSpecificProperty holds the name and value of a configuration which is specific to individual DNS providers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusEndpointsProviderSpecific {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probes: Option<Vec<DNSRecordStatusHealthCheckProbes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusHealthCheckProbes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    pub host: String,
    pub id: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}

/// Endpoint is a high-level way of a connection between a service and an IP
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusRelatedEndpoints {
    /// The hostname of the DNS record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsName")]
    pub dns_name: Option<String>,
    /// Labels stores labels defined for the Endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ProviderSpecific stores provider specific config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerSpecific")]
    pub provider_specific: Option<Vec<DNSRecordStatusRelatedEndpointsProviderSpecific>>,
    /// TTL for the record
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTTL")]
    pub record_ttl: Option<i64>,
    /// RecordType type of record, e.g. CNAME, A, AAAA, SRV, TXT etc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordType")]
    pub record_type: Option<String>,
    /// Identifier to distinguish multiple records with the same name and type (e.g. Route53 records with routing policies other than 'simple')
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setIdentifier")]
    pub set_identifier: Option<String>,
    /// The targets the DNS record points to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// ProviderSpecificProperty holds the name and value of a configuration which is specific to individual DNS providers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSRecordStatusRelatedEndpointsProviderSpecific {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

