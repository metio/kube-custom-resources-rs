// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/konveyor/operator/tackle.konveyor.io/v1alpha1/tasks.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec defines the desired state the resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tackle.konveyor.io", version = "v1alpha1", kind = "Task", plural = "tasks")]
#[kube(namespaced)]
#[kube(status = "TaskStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TaskSpec {
    /// Data object passed to the addon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, serde_json::Value>>,
    /// Dependencies defines a list of task names on which this task depends.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    /// Priority defines the task priority (0-n).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

/// Status defines the observed state the resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TaskStatus {
    /// The most recent generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

