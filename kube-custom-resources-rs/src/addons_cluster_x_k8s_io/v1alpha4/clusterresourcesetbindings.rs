// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/addons.cluster.x-k8s.io/v1alpha4/clusterresourcesetbindings.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// spec is the desired state of ClusterResourceSetBinding.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "addons.cluster.x-k8s.io", version = "v1alpha4", kind = "ClusterResourceSetBinding", plural = "clusterresourcesetbindings")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterResourceSetBindingSpec {
    /// bindings is a list of ClusterResourceSets and their resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<ClusterResourceSetBindingBindings>>,
}

/// ResourceSetBinding keeps info on all of the resources in a ClusterResourceSet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterResourceSetBindingBindings {
    /// clusterResourceSetName is the name of the ClusterResourceSet that is applied to the owner cluster of the binding.
    #[serde(rename = "clusterResourceSetName")]
    pub cluster_resource_set_name: String,
    /// resources is a list of resources that the ClusterResourceSet has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ClusterResourceSetBindingBindingsResources>>,
}

/// ResourceBinding shows the status of a resource that belongs to a ClusterResourceSet matched by the owner cluster of the ClusterResourceSetBinding object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterResourceSetBindingBindingsResources {
    /// applied is to track if a resource is applied to the cluster or not.
    pub applied: bool,
    /// hash is the hash of a resource's data. This can be used to decide if a resource is changed.
    /// For "ApplyOnce" ClusterResourceSet.spec.strategy, this is no-op as that strategy does not act on change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// kind of the resource. Supported kinds are: Secrets and ConfigMaps.
    pub kind: ClusterResourceSetBindingBindingsResourcesKind,
    /// lastAppliedTime identifies when this resource was last applied to the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAppliedTime")]
    pub last_applied_time: Option<String>,
    /// name of the resource that is in the same namespace with ClusterResourceSet object.
    pub name: String,
}

/// ResourceBinding shows the status of a resource that belongs to a ClusterResourceSet matched by the owner cluster of the ClusterResourceSetBinding object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterResourceSetBindingBindingsResourcesKind {
    Secret,
    ConfigMap,
}

