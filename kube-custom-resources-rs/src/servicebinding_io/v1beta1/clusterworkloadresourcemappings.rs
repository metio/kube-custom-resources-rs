// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/redhat-developer/service-binding-operator/servicebinding.io/v1beta1/clusterworkloadresourcemappings.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ClusterWorkloadResourceMappingSpec defines the desired state of ClusterWorkloadResourceMapping
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "servicebinding.io", version = "v1beta1", kind = "ClusterWorkloadResourceMapping", plural = "clusterworkloadresourcemappings")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterWorkloadResourceMappingSpec {
    /// Versions is the collection of versions for a given resource, with mappings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ClusterWorkloadResourceMappingVersions>>,
}

/// ClusterWorkloadResourceMappingTemplate defines the mapping for a specific version of an workload resource to a logical PodTemplateSpec-like structure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkloadResourceMappingVersions {
    /// Annotations is a Restricted JSONPath that references the annotations map within the workload resource. These annotations must end up in the resulting Pod, and are generally not the workload resource's annotations. Defaults to `.spec.template.metadata.annotations`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<String>,
    /// Containers is the collection of mappings to container-like fragments of the workload resource. Defaults to mappings appropriate for a PodSpecable resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ClusterWorkloadResourceMappingVersionsContainers>>,
    /// Version is the version of the workload resource that this mapping is for.
    pub version: String,
    /// Volumes is a Restricted JSONPath that references the slice of volumes within the workload resource. Defaults to `.spec.template.spec.volumes`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<String>,
}

/// ClusterWorkloadResourceMappingContainer defines the mapping for a specific fragment of an workload resource to a Container-like structure. 
///  Each mapping defines exactly one path that may match multiple container-like fragments within the workload resource. For each object matching the path the name, env and volumeMounts expressions are resolved to find those structures.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkloadResourceMappingVersionsContainers {
    /// Env is a Restricted JSONPath that references the slice of environment variables for the container with the container-like workload resource fragment. The referenced location is created if it does not exist. Defaults to `.envs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<String>,
    /// Name is a Restricted JSONPath that references the name of the container with the container-like workload resource fragment. If not defined, container name filtering is ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path is the JSONPath within the workload resource that matches an existing fragment that is container-like.
    pub path: String,
    /// VolumeMounts is a Restricted JSONPath that references the slice of volume mounts for the container with the container-like workload resource fragment. The referenced location is created if it does not exist. Defaults to `.volumeMounts`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeMounts")]
    pub volume_mounts: Option<String>,
}

