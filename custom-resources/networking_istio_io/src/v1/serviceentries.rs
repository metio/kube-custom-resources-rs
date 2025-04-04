// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/istio/istio/networking.istio.io/v1/serviceentries.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1", kind = "ServiceEntry", plural = "serviceentries")]
#[kube(namespaced)]
#[kube(status = "ServiceEntryStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ServiceEntrySpec {
    /// The virtual IP addresses associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// One or more endpoints associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ServiceEntryEndpoints>>,
    /// A list of namespaces to which this service is exported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    /// The hosts associated with the ServiceEntry.
    pub hosts: Vec<String>,
    /// Specify whether the service should be considered external to the mesh or part of the mesh.
    /// 
    /// Valid Options: MESH_EXTERNAL, MESH_INTERNAL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ServiceEntryLocation>,
    /// The ports associated with the external service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<ServiceEntryPorts>>,
    /// Service resolution mode for the hosts.
    /// 
    /// Valid Options: NONE, STATIC, DNS, DNS_ROUND_ROBIN
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ServiceEntryResolution>,
    /// If specified, the proxy will verify that the server certificate's subject alternate name matches one of the specified values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    /// Applicable only for MESH_INTERNAL services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<ServiceEntryWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryEndpoints {
    /// Address associated with the network endpoint without the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// One or more labels associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The locality associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Network enables Istio to group endpoints resident in the same L3 domain/network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Set of ports associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<BTreeMap<String, i64>>,
    /// The service account associated with the workload if a sidecar is present in the workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// The load balancing weight associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceEntryLocation {
    #[serde(rename = "MESH_EXTERNAL")]
    MeshExternal,
    #[serde(rename = "MESH_INTERNAL")]
    MeshInternal,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryPorts {
    /// Label assigned to the port.
    pub name: String,
    /// A valid non-negative integer port number.
    pub number: i64,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The port number on the endpoint where the traffic will be received.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Configuration affecting service registry. See more details at: https://istio.io/docs/reference/config/networking/service-entry.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceEntryResolution {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "STATIC")]
    Static,
    #[serde(rename = "DNS")]
    Dns,
    #[serde(rename = "DNS_ROUND_ROBIN")]
    DnsRoundRobin,
}

/// Applicable only for MESH_INTERNAL services.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryWorkloadSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which the configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resource Generation to which the Reconciled Condition refers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<ServiceEntryStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<ServiceEntryStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ServiceEntryStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceEntryStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceEntryStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

