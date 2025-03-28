// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/nodeupgrades.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// NodeUpgradeSpec defines the desired state of NodeUpgrade.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "NodeUpgrade", plural = "nodeupgrades")]
#[kube(namespaced)]
#[kube(status = "NodeUpgradeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeUpgradeSpec {
    /// EtcdVersion refers to the version of ETCD to upgrade to.
    /// This field is optional and only gets used for control plane nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "etcdVersion")]
    pub etcd_version: Option<String>,
    /// FirstNodeToBeUpgraded signifies that the Node is the first node to be upgraded.
    /// This flag is only valid for control plane nodes and ignored for worker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firstNodeToBeUpgraded")]
    pub first_node_to_be_upgraded: Option<bool>,
    /// KubernetesVersion refers to the Kubernetes version to upgrade the node to.
    #[serde(rename = "kubernetesVersion")]
    pub kubernetes_version: String,
    /// Machine is a reference to the CAPI Machine that needs to be upgraded.
    pub machine: ObjectReference,
}

/// Machine is a reference to the CAPI Machine that needs to be upgraded.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeUpgradeMachine {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// NodeUpgradeStatus defines the observed state of NodeUpgrade.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeUpgradeStatus {
    /// Completed denotes that the upgrader has completed running all the operations
    /// and the node is successfully upgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Conditions defines current state of the NodeUpgrade,
    /// including the state of init containers, that facilitate the upgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

