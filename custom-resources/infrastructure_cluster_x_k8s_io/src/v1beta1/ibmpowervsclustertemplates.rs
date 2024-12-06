// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta1/ibmpowervsclustertemplates.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// IBMPowerVSClusterTemplateSpec defines the desired state of IBMPowerVSClusterTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "IBMPowerVSClusterTemplate", plural = "ibmpowervsclustertemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMPowerVSClusterTemplateSpec {
    /// IBMPowerVSClusterTemplateResource describes the data needed to create an IBMPowerVSCluster from a template.
    pub template: IBMPowerVSClusterTemplateTemplate,
}

/// IBMPowerVSClusterTemplateResource describes the data needed to create an IBMPowerVSCluster from a template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterTemplateTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IBMPowerVSClusterTemplateTemplateMetadata>,
    /// IBMPowerVSClusterSpec defines the desired state of IBMPowerVSCluster.
    pub spec: IBMPowerVSClusterTemplateTemplateSpec,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterTemplateTemplateMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// IBMPowerVSClusterSpec defines the desired state of IBMPowerVSCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterTemplateTemplateSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<IBMPowerVSClusterTemplateTemplateSpecControlPlaneEndpoint>,
    /// Network is the reference to the Network to use for this cluster.
    pub network: IBMPowerVSClusterTemplateTemplateSpecNetwork,
    /// ServiceInstanceID is the id of the power cloud instance where the vsi instance will get deployed.
    #[serde(rename = "serviceInstanceID")]
    pub service_instance_id: String,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterTemplateTemplateSpecControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// Network is the reference to the Network to use for this cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSClusterTemplateTemplateSpecNetwork {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

