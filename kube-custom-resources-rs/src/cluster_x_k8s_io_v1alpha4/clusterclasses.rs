// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1alpha4/clusterclasses.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ClusterClassSpec describes the desired state of the ClusterClass.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1alpha4", kind = "ClusterClass", plural = "clusterclasses")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ClusterClassSpec {
    /// ControlPlane is a reference to a local struct that holds the details for provisioning the Control Plane for the Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<ClusterClassControlPlane>,
    /// Infrastructure is a reference to a provider-specific template that holds the details for provisioning infrastructure specific cluster for the underlying provider. The underlying provider is responsible for the implementation of the template to an infrastructure cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<ClusterClassInfrastructure>,
    /// Workers describes the worker nodes for the cluster. It is a collection of node types which can be used to create the worker nodes of the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workers: Option<ClusterClassWorkers>,
}

/// ControlPlane is a reference to a local struct that holds the details for provisioning the Control Plane for the Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassControlPlane {
    /// MachineTemplate defines the metadata and infrastructure information for control plane machines. 
    ///  This field is supported if and only if the control plane provider template referenced above is Machine based and supports setting replicas.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineInfrastructure")]
    pub machine_infrastructure: Option<ClusterClassControlPlaneMachineInfrastructure>,
    /// Metadata is the metadata applied to the machines of the ControlPlane. At runtime this metadata is merged with the corresponding metadata from the topology. 
    ///  This field is supported if and only if the control plane provider template referenced is Machine based.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterClassControlPlaneMetadata>,
    /// Ref is a required reference to a custom resource offered by a provider.
    #[serde(rename = "ref")]
    pub r#ref: ClusterClassControlPlaneRef,
}

/// MachineTemplate defines the metadata and infrastructure information for control plane machines. 
///  This field is supported if and only if the control plane provider template referenced above is Machine based and supports setting replicas.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassControlPlaneMachineInfrastructure {
    /// Ref is a required reference to a custom resource offered by a provider.
    #[serde(rename = "ref")]
    pub r#ref: ClusterClassControlPlaneMachineInfrastructureRef,
}

/// Ref is a required reference to a custom resource offered by a provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassControlPlaneMachineInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Metadata is the metadata applied to the machines of the ControlPlane. At runtime this metadata is merged with the corresponding metadata from the topology. 
///  This field is supported if and only if the control plane provider template referenced is Machine based.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassControlPlaneMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Ref is a required reference to a custom resource offered by a provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassControlPlaneRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Infrastructure is a reference to a provider-specific template that holds the details for provisioning infrastructure specific cluster for the underlying provider. The underlying provider is responsible for the implementation of the template to an infrastructure cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassInfrastructure {
    /// Ref is a required reference to a custom resource offered by a provider.
    #[serde(rename = "ref")]
    pub r#ref: ClusterClassInfrastructureRef,
}

/// Ref is a required reference to a custom resource offered by a provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Workers describes the worker nodes for the cluster. It is a collection of node types which can be used to create the worker nodes of the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkers {
    /// MachineDeployments is a list of machine deployment classes that can be used to create a set of worker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineDeployments")]
    pub machine_deployments: Option<Vec<ClusterClassWorkersMachineDeployments>>,
}

/// MachineDeploymentClass serves as a template to define a set of worker nodes of the cluster provisioned using the `ClusterClass`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeployments {
    /// Class denotes a type of worker node present in the cluster, this name MUST be unique within a ClusterClass and can be referenced in the Cluster to create a managed MachineDeployment.
    pub class: String,
    /// Template is a local struct containing a collection of templates for creation of MachineDeployment objects representing a set of worker nodes.
    pub template: ClusterClassWorkersMachineDeploymentsTemplate,
}

/// Template is a local struct containing a collection of templates for creation of MachineDeployment objects representing a set of worker nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplate {
    /// Bootstrap contains the bootstrap template reference to be used for the creation of worker Machines.
    pub bootstrap: ClusterClassWorkersMachineDeploymentsTemplateBootstrap,
    /// Infrastructure contains the infrastructure template reference to be used for the creation of worker Machines.
    pub infrastructure: ClusterClassWorkersMachineDeploymentsTemplateInfrastructure,
    /// Metadata is the metadata applied to the machines of the MachineDeployment. At runtime this metadata is merged with the corresponding metadata from the topology.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClusterClassWorkersMachineDeploymentsTemplateMetadata>,
}

/// Bootstrap contains the bootstrap template reference to be used for the creation of worker Machines.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplateBootstrap {
    /// Ref is a required reference to a custom resource offered by a provider.
    #[serde(rename = "ref")]
    pub r#ref: ClusterClassWorkersMachineDeploymentsTemplateBootstrapRef,
}

/// Ref is a required reference to a custom resource offered by a provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplateBootstrapRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Infrastructure contains the infrastructure template reference to be used for the creation of worker Machines.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplateInfrastructure {
    /// Ref is a required reference to a custom resource offered by a provider.
    #[serde(rename = "ref")]
    pub r#ref: ClusterClassWorkersMachineDeploymentsTemplateInfrastructureRef,
}

/// Ref is a required reference to a custom resource offered by a provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplateInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Metadata is the metadata applied to the machines of the MachineDeployment. At runtime this metadata is merged with the corresponding metadata from the topology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClassWorkersMachineDeploymentsTemplateMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

