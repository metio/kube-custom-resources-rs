// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/stresschaos.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec defines the behavior of a time chaos experiment
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "StressChaos", plural = "stresschaos")]
#[kube(namespaced)]
#[kube(status = "StressChaosStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct StressChaosSpec {
    /// ContainerNames indicates list of the name of affected container.
    /// If not set, the first container will be injected
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerNames")]
    pub container_names: Option<Vec<String>>,
    /// Duration represents the duration of the chaos action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Mode defines the mode to run chaos action.
    /// Supported mode: one / all / fixed / fixed-percent / random-max-percent
    pub mode: StressChaosMode,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// Selector is used to select pods that are used to inject chaos action.
    pub selector: StressChaosSelector,
    /// StressngStressors defines plenty of stressors just like `Stressors` except that it's an experimental
    /// feature and more powerful. You can define stressors in `stress-ng` (see also `man stress-ng`) dialect,
    /// however not all of the supported stressors are well tested. It maybe retired in later releases. You
    /// should always use `Stressors` to define the stressors and use this only when you want more stressors
    /// unsupported by `Stressors`. When both `StressngStressors` and `Stressors` are defined, `StressngStressors`
    /// wins.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stressngStressors")]
    pub stressng_stressors: Option<String>,
    /// Stressors defines plenty of stressors supported to stress system components out.
    /// You can use one or more of them to make up various kinds of stresses. At least
    /// one of the stressors should be specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stressors: Option<StressChaosStressors>,
    /// Value is required when the mode is set to `FixedMode` / `FixedPercentMode` / `RandomMaxPercentMode`.
    /// If `FixedMode`, provide an integer of pods to do chaos action.
    /// If `FixedPercentMode`, provide a number from 0-100 to specify the percent of pods the server can do chaos action.
    /// IF `RandomMaxPercentMode`,  provide a number from 0-100 to specify the max percent of pods to do chaos action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Spec defines the behavior of a time chaos experiment
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum StressChaosMode {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "fixed-percent")]
    FixedPercent,
    #[serde(rename = "random-max-percent")]
    RandomMaxPercent,
}

/// Selector is used to select pods that are used to inject chaos action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosSelector {
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelectors")]
    pub annotation_selectors: Option<BTreeMap<String, String>>,
    /// a slice of label selector expressions that can be used to select objects.
    /// A list of selectors based on set-based label expressions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expressionSelectors")]
    pub expression_selectors: Option<Vec<StressChaosSelectorExpressionSelectors>>,
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelectors")]
    pub field_selectors: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelectors")]
    pub label_selectors: Option<BTreeMap<String, String>>,
    /// Namespaces is a set of namespace to which objects belong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Map of string keys and values that can be used to select nodes.
    /// Selector which must match a node's labels,
    /// and objects must belong to these selected nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelectors")]
    pub node_selectors: Option<BTreeMap<String, String>>,
    /// Nodes is a set of node name and objects must belong to these nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// PodPhaseSelectors is a set of condition of a pod at the current time.
    /// supported value: Pending / Running / Succeeded / Failed / Unknown
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podPhaseSelectors")]
    pub pod_phase_selectors: Option<Vec<String>>,
    /// Pods is a map of string keys and a set values that used to select pods.
    /// The key defines the namespace which pods belong,
    /// and the each values is a set of pod names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosSelectorExpressionSelectors {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Stressors defines plenty of stressors supported to stress system components out.
/// You can use one or more of them to make up various kinds of stresses. At least
/// one of the stressors should be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStressors {
    /// CPUStressor stresses CPU out
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<StressChaosStressorsCpu>,
    /// MemoryStressor stresses virtual memory out
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<StressChaosStressorsMemory>,
}

/// CPUStressor stresses CPU out
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStressorsCpu {
    /// Load specifies P percent loading per CPU worker. 0 is effectively a sleep (no load) and 100
    /// is full loading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<i64>,
    /// extend stress-ng options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Workers specifies N workers to apply the stressor.
    /// Maximum 8192 workers can run by stress-ng
    pub workers: i64,
}

/// MemoryStressor stresses virtual memory out
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStressorsMemory {
    /// OOMScoreAdj sets the oom_score_adj of the stress process. See `man 5 proc` to know more
    /// about this option.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    /// extend stress-ng options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Size specifies N bytes consumed per vm worker, default is the total available memory.
    /// One can specify the size as % of total available memory or in units of B, KB/KiB,
    /// MB/MiB, GB/GiB, TB/TiB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Workers specifies N workers to apply the stressor.
    /// Maximum 8192 workers can run by stress-ng
    pub workers: i64,
}

/// Most recently observed status of the time chaos experiment
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStatus {
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<StressChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: StressChaosStatusExperiment,
    /// Instances always specifies stressing instances
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<BTreeMap<String, StressChaosStatusInstances>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<StressChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<StressChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<StressChaosStatusExperimentContainerRecordsEvents>>,
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
pub struct StressChaosStatusExperimentContainerRecordsEvents {
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
pub enum StressChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

/// Instances always specifies stressing instances
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StressChaosStatusInstances {
    /// MemoryStartTime specifies when the memStress starts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryStartTime")]
    pub memory_start_time: Option<String>,
    /// MemoryUID is the memStress identifier
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryUid")]
    pub memory_uid: Option<String>,
    /// StartTime specifies when the stress-ng starts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    /// UID is the stress-ng identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

