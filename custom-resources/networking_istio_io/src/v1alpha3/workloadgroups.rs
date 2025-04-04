// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/istio/istio/networking.istio.io/v1alpha3/workloadgroups.yaml
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

/// Describes a collection of workload instances. See more details at: https://istio.io/docs/reference/config/networking/workload-group.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "WorkloadGroup", plural = "workloadgroups")]
#[kube(namespaced)]
#[kube(status = "WorkloadGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WorkloadGroupSpec {
    /// Metadata that will be used for all corresponding `WorkloadEntries`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WorkloadGroupMetadata>,
    /// `ReadinessProbe` describes the configuration the user must provide for healthchecking on their workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<WorkloadGroupProbe>,
    /// Template to be used for the generation of `WorkloadEntry` resources that belong to this `WorkloadGroup`.
    pub template: WorkloadGroupTemplate,
}

/// Metadata that will be used for all corresponding `WorkloadEntries`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// `ReadinessProbe` describes the configuration the user must provide for healthchecking on their workload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupProbe {
    /// Health is determined by how the command that is executed exited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<WorkloadGroupProbeExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i32>,
    /// `httpGet` is performed to a given endpoint and the status/able to connect determines health.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpGet")]
    pub http_get: Option<WorkloadGroupProbeHttpGet>,
    /// Number of seconds after the container has started before readiness probes are initiated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialDelaySeconds")]
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "periodSeconds")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successThreshold")]
    pub success_threshold: Option<i32>,
    /// Health is determined by if the proxy is able to connect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpSocket")]
    pub tcp_socket: Option<WorkloadGroupProbeTcpSocket>,
    /// Number of seconds after which the probe times out.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

/// Health is determined by how the command that is executed exited.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupProbeExec {
    /// Command to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

/// `httpGet` is performed to a given endpoint and the status/able to connect determines health.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupProbeHttpGet {
    /// Host name to connect to, defaults to the pod IP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Headers the proxy will pass on to make the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpHeaders")]
    pub http_headers: Option<Vec<WorkloadGroupProbeHttpGetHttpHeaders>>,
    /// Path to access on the HTTP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port on which the endpoint lives.
    pub port: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupProbeHttpGetHttpHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Health is determined by if the proxy is able to connect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupProbeTcpSocket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub port: i64,
}

/// Template to be used for the generation of `WorkloadEntry` resources that belong to this `WorkloadGroup`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupTemplate {
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
pub struct WorkloadGroupStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resource Generation to which the Reconciled Condition refers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<WorkloadGroupStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkloadGroupStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<WorkloadGroupStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WorkloadGroupStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WorkloadGroupStatusValidationMessagesLevel {
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
pub struct WorkloadGroupStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

