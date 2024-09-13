// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/istio/istio/networking.istio.io/v1alpha3/workloadentries.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Configuration affecting VMs onboarded into the mesh. See more details at: https://istio.io/docs/reference/config/networking/workload-entry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "WorkloadEntry", plural = "workloadentries")]
#[kube(namespaced)]
#[kube(status = "WorkloadEntryStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WorkloadEntrySpec {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadEntryStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resource Generation to which the Reconciled Condition refers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<WorkloadEntryStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadEntryStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<WorkloadEntryStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WorkloadEntryStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WorkloadEntryStatusValidationMessagesLevel {
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
pub struct WorkloadEntryStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

