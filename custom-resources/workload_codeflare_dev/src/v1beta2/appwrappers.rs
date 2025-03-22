// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/project-codeflare/codeflare-operator/workload.codeflare.dev/v1beta2/appwrappers.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// AppWrapperSpec defines the desired state of the AppWrapper
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "workload.codeflare.dev", version = "v1beta2", kind = "AppWrapper", plural = "appwrappers")]
#[kube(namespaced)]
#[kube(status = "AppWrapperStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AppWrapperSpec {
    /// Components lists the components contained in the AppWrapper
    pub components: Vec<AppWrapperComponents>,
    /// ManagedBy is used to indicate the controller or entity that manages the AppWrapper.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedBy")]
    pub managed_by: Option<String>,
    /// Suspend suspends the AppWrapper when set to true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

/// AppWrapperComponent describes a single wrapped Kubernetes resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperComponents {
    /// Annotations is an unstructured key value map that may be used to store and retrieve
    /// arbitrary metadata about the Component to customize its treatment by the AppWrapper controller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// PodSetInfos assigned to the Component's PodSets by Kueue
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSetInfos")]
    pub pod_set_infos: Option<Vec<AppWrapperComponentsPodSetInfos>>,
    /// DeclaredPodSets for the Component (optional for known PodCreating GVKs)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSets")]
    pub pod_sets: Option<Vec<AppWrapperComponentsPodSets>>,
    /// Template defines the Kubernetes resource for the Component
    pub template: BTreeMap<String, serde_json::Value>,
}

/// AppWrapperPodSetInfo contains the data that Kueue wants to inject into an admitted PodSpecTemplate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperComponentsPodSetInfos {
    /// Annotations to be added to the PodSpecTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels to be added to the PodSepcTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// NodeSelectors to be added to the PodSpecTemplate
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// SchedulingGates to be added to the PodSpecTemplate
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulingGates")]
    pub scheduling_gates: Option<Vec<AppWrapperComponentsPodSetInfosSchedulingGates>>,
    /// Tolerations to be added to the PodSpecTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<AppWrapperComponentsPodSetInfosTolerations>>,
}

/// PodSchedulingGate is associated to a Pod to guard its scheduling.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperComponentsPodSetInfosSchedulingGates {
    /// Name of the scheduling gate.
    /// Each scheduling gate must have a unique name field.
    pub name: String,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperComponentsPodSetInfosTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// AppWrapperPodSet describes an homogeneous set of pods
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperComponentsPodSets {
    /// Path is the path Component.Template to the PodTemplateSpec for this PodSet
    pub path: String,
    /// Replicas is the number of pods in this PodSet
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// AppWrapperStatus defines the observed state of the appwrapper
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperStatus {
    /// ComponentStatus parallels the Components array in the Spec and tracks the actually deployed resources
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "componentStatus")]
    pub component_status: Option<Vec<AppWrapperStatusComponentStatus>>,
    /// Conditions hold the latest available observations of the AppWrapper current state.
    /// 
    /// The type of the condition could be:
    /// 
    /// - QuotaReserved: The AppWrapper was admitted by Kueue and has quota allocated to it
    /// - ResourcesDeployed: The contained resources are deployed (or being deployed) on the cluster
    /// - PodsReady: All pods of the contained resources are in the Ready or Succeeded state
    /// - Unhealthy: One or more of the contained resources is unhealthy
    /// - DeletingResources: The contained resources are in the process of being deleted from the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Phase of the AppWrapper object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Retries counts the number of times the AppWrapper has entered the Resetting Phase
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resettingCount")]
    pub resetting_count: Option<i32>,
}

/// AppWrapperComponentStatus tracks the status of a single managed Component
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperStatusComponentStatus {
    /// APIVersion is the APIVersion of the Component
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Conditions hold the latest available observations of the Component's current state.
    /// 
    /// The type of the condition could be:
    /// 
    /// - ResourcesDeployed: The component is deployed on the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Kind is the Kind of the Component
    pub kind: String,
    /// Name is the name of the Component
    pub name: String,
    /// PodSets is the validated PodSets for the Component (either from AppWrapperComponent.DeclaredPodSets or inferred by the controller)
    #[serde(rename = "podSets")]
    pub pod_sets: Vec<AppWrapperStatusComponentStatusPodSets>,
}

/// AppWrapperPodSet describes an homogeneous set of pods
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppWrapperStatusComponentStatusPodSets {
    /// Path is the path Component.Template to the PodTemplateSpec for this PodSet
    pub path: String,
    /// Replicas is the number of pods in this PodSet
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

