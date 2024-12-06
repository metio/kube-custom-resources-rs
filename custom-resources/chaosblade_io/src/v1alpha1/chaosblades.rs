// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaosblade-io/chaosblade-operator/chaosblade.io/v1alpha1/chaosblades.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ChaosBladeSpec defines the desired state of ChaosBlade
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chaosblade.io", version = "v1alpha1", kind = "ChaosBlade", plural = "chaosblades")]
#[kube(namespaced)]
#[kube(status = "ChaosBladeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ChaosBladeSpec {
    /// INSERT ADDITIONAL SPEC FIELDS - desired state of cluster Important: Run "operator-sdk generate k8s" to regenerate code after modifying this file Add custom validation using kubebuilder tags: https://book-v1.book.kubebuilder.io/beyond_basics/generating_crd.html
    pub experiments: Vec<ChaosBladeExperiments>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChaosBladeExperiments {
    /// Action is the experiment scenario of the target, such as delay, load
    pub action: String,
    /// Desc is the experiment description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Matchers is the experiment rules
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matchers: Option<Vec<ChaosBladeExperimentsMatchers>>,
    /// Scope is the area of the experiments, currently support node, pod and container
    pub scope: String,
    /// Target is the experiment target, such as cpu, network
    pub target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChaosBladeExperimentsMatchers {
    /// Name is the name of flag
    pub name: String,
    /// TODO: Temporarily defined as an array for all flags Value is the value of flag
    pub value: Vec<String>,
}

/// ChaosBladeStatus defines the observed state of ChaosBlade
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChaosBladeStatus {
    /// Important: Run "operator-sdk generate k8s" to regenerate code after modifying this file Add custom validation using kubebuilder tags: https://book-v1.book.kubebuilder.io/beyond_basics/generating_crd.html
    #[serde(rename = "expStatuses")]
    pub exp_statuses: Vec<ChaosBladeStatusExpStatuses>,
    /// Phase indicates the state of the experiment   Initial -> Running -> Updating -> Destroying -> Destroyed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChaosBladeStatusExpStatuses {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ResStatuses is the details of the experiment
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resStatuses")]
    pub res_statuses: Option<Vec<ChaosBladeStatusExpStatusesResStatuses>>,
    /// experiment scope for cache
    pub scope: String,
    /// State is used to describe the experiment result
    pub state: String,
    /// Success is used to judge the experiment result
    pub success: bool,
    pub target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ChaosBladeStatusExpStatusesResStatuses {
    /// experiment error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// experiment uid in chaosblade
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource identifier, rules as following: container: Namespace/NodeName/PodName/ContainerName pod： Namespace/NodeName/PodName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Kind
    pub kind: String,
    /// experiment state
    pub state: String,
    /// success
    pub success: bool,
}

