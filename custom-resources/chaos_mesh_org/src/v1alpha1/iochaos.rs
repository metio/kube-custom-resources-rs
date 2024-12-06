// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/iochaos.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// IOChaosSpec defines the desired state of IOChaos
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "IOChaos", plural = "iochaos")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct IOChaosSpec {
    /// Action defines the specific pod chaos action. Supported action: latency / fault / attrOverride / mistake
    pub action: IOChaosAction,
    /// Attr defines the overrided attribution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr: Option<IOChaosAttr>,
    /// ContainerNames indicates list of the name of affected container. If not set, the first container will be injected
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerNames")]
    pub container_names: Option<Vec<String>>,
    /// Delay defines the value of I/O chaos action delay. A delay string is a possibly signed sequence of decimal numbers, each with optional fraction and a unit suffix, such as "300ms". Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Duration represents the duration of the chaos action. It is required when the action is `PodFailureAction`. A duration string is a possibly signed sequence of decimal numbers, each with optional fraction and a unit suffix, such as "300ms", "-1.5h" or "2h45m". Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Errno defines the error code that returned by I/O action. refer to: https://www-numi.fnal.gov/offline_software/srt_public_context/WebDocs/Errors/unix_system_errors.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errno: Option<i32>,
    /// Methods defines the I/O methods for injecting I/O chaos action. default: all I/O methods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<String>>,
    /// Mistake defines what types of incorrectness are injected to IO operations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mistake: Option<IOChaosMistake>,
    /// Mode defines the mode to run chaos action. Supported mode: one / all / fixed / fixed-percent / random-max-percent
    pub mode: IOChaosMode,
    /// Path defines the path of files for injecting I/O chaos action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Percent defines the percentage of injection errors and provides a number from 0-100. default: 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// Selector is used to select pods that are used to inject chaos action.
    pub selector: IOChaosSelector,
    /// Value is required when the mode is set to `FixedMode` / `FixedPercentMode` / `RandomMaxPercentMode`. If `FixedMode`, provide an integer of pods to do chaos action. If `FixedPercentMode`, provide a number from 0-100 to specify the percent of pods the server can do chaos action. IF `RandomMaxPercentMode`,  provide a number from 0-100 to specify the max percent of pods to do chaos action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// VolumePath represents the mount path of injected volume
    #[serde(rename = "volumePath")]
    pub volume_path: String,
}

/// IOChaosSpec defines the desired state of IOChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IOChaosAction {
    #[serde(rename = "latency")]
    Latency,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "attrOverride")]
    AttrOverride,
    #[serde(rename = "mistake")]
    Mistake,
}

/// Attr defines the overrided attribution
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosAttr {
    /// Timespec represents a time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atime: Option<IOChaosAttrAtime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<i64>,
    /// Timespec represents a time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctime: Option<IOChaosAttrCtime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ino: Option<i64>,
    /// FileType represents type of file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Timespec represents a time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtime: Option<IOChaosAttrMtime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nlink: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdev: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
}

/// Timespec represents a time
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosAttrAtime {
    pub nsec: i64,
    pub sec: i64,
}

/// Timespec represents a time
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosAttrCtime {
    pub nsec: i64,
    pub sec: i64,
}

/// Timespec represents a time
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosAttrMtime {
    pub nsec: i64,
    pub sec: i64,
}

/// Mistake defines what types of incorrectness are injected to IO operations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosMistake {
    /// Filling determines what is filled in the mistake data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filling: Option<IOChaosMistakeFilling>,
    /// Max length of each wrong data segment in bytes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxLength")]
    pub max_length: Option<i64>,
    /// There will be [1, MaxOccurrences] segments of wrong data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxOccurrences")]
    pub max_occurrences: Option<i64>,
}

/// Mistake defines what types of incorrectness are injected to IO operations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IOChaosMistakeFilling {
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "random")]
    Random,
}

/// IOChaosSpec defines the desired state of IOChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IOChaosMode {
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
pub struct IOChaosSelector {
    /// Map of string keys and values that can be used to select objects. A selector based on annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelectors")]
    pub annotation_selectors: Option<BTreeMap<String, String>>,
    /// a slice of label selector expressions that can be used to select objects. A list of selectors based on set-based label expressions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expressionSelectors")]
    pub expression_selectors: Option<Vec<IOChaosSelectorExpressionSelectors>>,
    /// Map of string keys and values that can be used to select objects. A selector based on fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelectors")]
    pub field_selectors: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to select objects. A selector based on labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelectors")]
    pub label_selectors: Option<BTreeMap<String, String>>,
    /// Namespaces is a set of namespace to which objects belong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Map of string keys and values that can be used to select nodes. Selector which must match a node's labels, and objects must belong to these selected nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelectors")]
    pub node_selectors: Option<BTreeMap<String, String>>,
    /// Nodes is a set of node name and objects must belong to these nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// PodPhaseSelectors is a set of condition of a pod at the current time. supported value: Pending / Running / Succeeded / Failed / Unknown
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podPhaseSelectors")]
    pub pod_phase_selectors: Option<Vec<String>>,
    /// Pods is a map of string keys and a set values that used to select pods. The key defines the namespace which pods belong, and the each values is a set of pod names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosSelectorExpressionSelectors {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// IOChaosStatus defines the observed state of IOChaos
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosStatus {
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<IOChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: IOChaosStatusExperiment,
    /// Instances always specifies podiochaos generation or empty
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<BTreeMap<String, i64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<IOChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<IOChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IOChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<IOChaosStatusExperimentContainerRecordsEvents>>,
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
pub struct IOChaosStatusExperimentContainerRecordsEvents {
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
pub enum IOChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

