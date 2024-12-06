// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1beta1/vsphereclustertemplates.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// VSphereClusterTemplateSpec defines the desired state of VSphereClusterTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "VSphereClusterTemplate", plural = "vsphereclustertemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereClusterTemplateSpec {
    /// VSphereClusterTemplateResource describes the data for creating a VSphereCluster from a template.
    pub template: VSphereClusterTemplateTemplate,
}

/// VSphereClusterTemplateResource describes the data for creating a VSphereCluster from a template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplate {
    /// VSphereClusterSpec defines the desired state of VSphereCluster.
    pub spec: VSphereClusterTemplateTemplateSpec,
}

/// VSphereClusterSpec defines the desired state of VSphereCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpec {
    /// ClusterModules hosts information regarding the anti-affinity vSphere constructs
    /// for each of the objects responsible for creation of VM objects belonging to the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterModules")]
    pub cluster_modules: Option<Vec<VSphereClusterTemplateTemplateSpecClusterModules>>,
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<VSphereClusterTemplateTemplateSpecControlPlaneEndpoint>,
    /// DisableClusterModule is used to explicitly turn off the ClusterModule feature.
    /// This should work along side NodeAntiAffinity feature flag.
    /// If the NodeAntiAffinity feature flag is turned off, this will be disregarded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableClusterModule")]
    pub disable_cluster_module: Option<bool>,
    /// FailureDomainSelector is the label selector to use for failure domain selection
    /// for the control plane nodes of the cluster.
    /// If not set (`nil`), selecting failure domains will be disabled.
    /// An empty value (`{}`) selects all existing failure domains.
    /// A valid selector will select all failure domains which match the selector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomainSelector")]
    pub failure_domain_selector: Option<VSphereClusterTemplateTemplateSpecFailureDomainSelector>,
    /// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
    /// the identity to use when reconciling the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityRef")]
    pub identity_ref: Option<VSphereClusterTemplateTemplateSpecIdentityRef>,
    /// Server is the address of the vSphere endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Thumbprint is the colon-separated SHA-1 checksum of the given vCenter server's host certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

/// ClusterModule holds the anti affinity construct `ClusterModule` identifier
/// in use by the VMs owned by the object referred by the TargetObjectName field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecClusterModules {
    /// ControlPlane indicates whether the referred object is responsible for control plane nodes.
    /// Currently, only the KubeadmControlPlane objects have this flag set to true.
    /// Only a single object in the slice can have this value set to true.
    #[serde(rename = "controlPlane")]
    pub control_plane: bool,
    /// ModuleUUID is the unique identifier of the `ClusterModule` used by the object.
    #[serde(rename = "moduleUUID")]
    pub module_uuid: String,
    /// TargetObjectName points to the object that uses the Cluster Module information to enforce
    /// anti-affinity amongst its descendant VM objects.
    #[serde(rename = "targetObjectName")]
    pub target_object_name: String,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecControlPlaneEndpoint {
    /// The hostname on which the API server is serving.
    pub host: String,
    /// The port on which the API server is serving.
    pub port: i32,
}

/// FailureDomainSelector is the label selector to use for failure domain selection
/// for the control plane nodes of the cluster.
/// If not set (`nil`), selecting failure domains will be disabled.
/// An empty value (`{}`) selects all existing failure domains.
/// A valid selector will select all failure domains which match the selector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecFailureDomainSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VSphereClusterTemplateTemplateSpecFailureDomainSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecFailureDomainSelectorMatchExpressions {
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

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
/// the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VSphereClusterTemplateTemplateSpecIdentityRef {
    /// Kind of the identity. Can either be VSphereClusterIdentity or Secret
    pub kind: VSphereClusterTemplateTemplateSpecIdentityRefKind,
    /// Name of the identity.
    pub name: String,
}

/// IdentityRef is a reference to either a Secret or VSphereClusterIdentity that contains
/// the identity to use when reconciling the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereClusterTemplateTemplateSpecIdentityRefKind {
    VSphereClusterIdentity,
    Secret,
}

