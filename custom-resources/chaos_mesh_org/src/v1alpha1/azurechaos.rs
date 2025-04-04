// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/azurechaos.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AzureChaosSpec is the content of the specification for an AzureChaos
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "AzureChaos", plural = "azurechaos")]
#[kube(namespaced)]
#[kube(status = "AzureChaosStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct AzureChaosSpec {
    /// Action defines the specific azure chaos action.
    /// Supported action: vm-stop / vm-restart / disk-detach
    /// Default action: vm-stop
    pub action: AzureChaosAction,
    /// DiskName indicates the name of the disk.
    /// Needed in disk-detach.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskName")]
    pub disk_name: Option<String>,
    /// Duration represents the duration of the chaos action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// LUN indicates the Logical Unit Number of the data disk.
    /// Needed in disk-detach.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i64>,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// ResourceGroupName defines the name of ResourceGroup
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// SecretName defines the name of kubernetes secret. It is used for Azure credentials.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// SubscriptionID defines the id of Azure subscription.
    #[serde(rename = "subscriptionID")]
    pub subscription_id: String,
    /// VMName defines the name of Virtual Machine
    #[serde(rename = "vmName")]
    pub vm_name: String,
}

/// AzureChaosSpec is the content of the specification for an AzureChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AzureChaosAction {
    #[serde(rename = "vm-stop")]
    VmStop,
    #[serde(rename = "vm-restart")]
    VmRestart,
    #[serde(rename = "disk-detach")]
    DiskDetach,
}

/// AzureChaosStatus represents the status of an AzureChaos
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureChaosStatus {
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AzureChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: AzureChaosStatusExperiment,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<AzureChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<AzureChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<AzureChaosStatusExperimentContainerRecordsEvents>>,
    pub id: String,
    /// InjectedCount is a counter to record the sum of successful injections
    #[serde(rename = "injectedCount")]
    pub injected_count: i64,
    pub phase: String,
    /// RecoveredCount is a counter to record the sum of successful recoveries
    #[serde(rename = "recoveredCount")]
    pub recovered_count: i64,
    #[serde(rename = "selectorKey")]
    pub selector_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AzureChaosStatusExperimentContainerRecordsEvents {
    /// Message is the detail message, e.g. the reason why we failed to inject the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Operation represents the operation we are doing, when we crate this event
    pub operation: String,
    /// Timestamp is time when we create this event
    pub timestamp: String,
    /// Type means the stage of this event
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AzureChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

