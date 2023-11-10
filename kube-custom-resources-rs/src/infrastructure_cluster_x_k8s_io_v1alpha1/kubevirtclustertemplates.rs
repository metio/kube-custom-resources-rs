// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-kubevirt/infrastructure.cluster.x-k8s.io/v1alpha1/kubevirtclustertemplates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// KubevirtClusterTemplateSpec defines the desired state of KubevirtClusterTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha1", kind = "KubevirtClusterTemplate", plural = "kubevirtclustertemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct KubevirtClusterTemplateSpec {
    /// KubevirtClusterTemplateResource describes the data needed to create a KubevirtCluster from a template.
    pub template: KubevirtClusterTemplateTemplate,
}

/// KubevirtClusterTemplateResource describes the data needed to create a KubevirtCluster from a template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplate {
    /// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create. This is a copy of customizable fields from metav1.ObjectMeta. 
    ///  ObjectMeta is embedded in `Machine.Spec`, `MachineDeployment.Template` and `MachineSet.Template`, which are not top-level Kubernetes objects. Given that metav1.ObjectMeta has lots of special cases and read-only fields which end up in the generated CRD validation, having it as a subset simplifies the API and some issues that can impact user experience. 
    ///  During the [upgrade to controller-tools@v2](https://github.com/kubernetes-sigs/cluster-api/pull/1054) for v1alpha2, we noticed a failure would occur running Cluster API test suite against the new CRDs, specifically `spec.metadata.creationTimestamp in body must be of type string: "null"`. The investigation showed that `controller-tools@v2` behaves differently than its previous version when handling types from [metav1](k8s.io/apimachinery/pkg/apis/meta/v1) package. 
    ///  In more details, we found that embedded (non-top level) types that embedded `metav1.ObjectMeta` had validation properties, including for `creationTimestamp` (metav1.Time). The `metav1.Time` type specifies a custom json marshaller that, when IsZero() is true, returns `null` which breaks validation because the field isn't marked as nullable. 
    ///  In future versions, controller-tools@v2 might allow overriding the type and validation for embedded types. When that happens, this hack should be revisited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<KubevirtClusterTemplateTemplateMetadata>,
    /// KubevirtClusterSpec defines the desired state of KubevirtCluster.
    pub spec: KubevirtClusterTemplateTemplateSpec,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create. This is a copy of customizable fields from metav1.ObjectMeta. 
///  ObjectMeta is embedded in `Machine.Spec`, `MachineDeployment.Template` and `MachineSet.Template`, which are not top-level Kubernetes objects. Given that metav1.ObjectMeta has lots of special cases and read-only fields which end up in the generated CRD validation, having it as a subset simplifies the API and some issues that can impact user experience. 
///  During the [upgrade to controller-tools@v2](https://github.com/kubernetes-sigs/cluster-api/pull/1054) for v1alpha2, we noticed a failure would occur running Cluster API test suite against the new CRDs, specifically `spec.metadata.creationTimestamp in body must be of type string: "null"`. The investigation showed that `controller-tools@v2` behaves differently than its previous version when handling types from [metav1](k8s.io/apimachinery/pkg/apis/meta/v1) package. 
///  In more details, we found that embedded (non-top level) types that embedded `metav1.ObjectMeta` had validation properties, including for `creationTimestamp` (metav1.Time). The `metav1.Time` type specifies a custom json marshaller that, when IsZero() is true, returns `null` which breaks validation because the field isn't marked as nullable. 
///  In future versions, controller-tools@v2 might allow overriding the type and validation for embedded types. When that happens, this hack should be revisited.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// KubevirtClusterSpec defines the desired state of KubevirtCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<KubevirtClusterTemplateTemplateSpecControlPlaneEndpoint>,
    /// ControlPlaneServiceTemplate can be used to modify service that fronts the control plane nodes to handle the api-server traffic (port 6443). This field is optional, by default control plane nodes will use a service of type ClusterIP, which will make workload cluster only accessible within the same cluster. Note, this does not aim to expose the entire Service spec to users, but only provides capability to modify the service metadata and the service type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneServiceTemplate")]
    pub control_plane_service_template: Option<KubevirtClusterTemplateTemplateSpecControlPlaneServiceTemplate>,
    /// InfraClusterSecretRef is a reference to a secret with a kubeconfig for external cluster used for infra.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infraClusterSecretRef")]
    pub infra_cluster_secret_ref: Option<KubevirtClusterTemplateTemplateSpecInfraClusterSecretRef>,
    /// SSHKeys is a reference to a local struct for SSH keys persistence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKeys")]
    pub ssh_keys: Option<KubevirtClusterTemplateTemplateSpecSshKeys>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecControlPlaneEndpoint {
    /// Host is the hostname on which the API server is serving.
    pub host: String,
    /// Port is the port on which the API server is serving.
    pub port: i64,
}

/// ControlPlaneServiceTemplate can be used to modify service that fronts the control plane nodes to handle the api-server traffic (port 6443). This field is optional, by default control plane nodes will use a service of type ClusterIP, which will make workload cluster only accessible within the same cluster. Note, this does not aim to expose the entire Service spec to users, but only provides capability to modify the service metadata and the service type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecControlPlaneServiceTemplate {
    /// Service metadata allows to set labels, annotations and namespace for the service. When infraClusterSecretRef is used, ControlPlaneService take the kubeconfig namespace by default if metadata.namespace is not specified. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    /// Service specification allows to override some fields in the service spec. Note, it does not aim cover all fields of the service spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<KubevirtClusterTemplateTemplateSpecControlPlaneServiceTemplateSpec>,
}

/// Service specification allows to override some fields in the service spec. Note, it does not aim cover all fields of the service spec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecControlPlaneServiceTemplateSpec {
    /// Type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// InfraClusterSecretRef is a reference to a secret with a kubeconfig for external cluster used for infra.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecInfraClusterSecretRef {
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

/// SSHKeys is a reference to a local struct for SSH keys persistence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecSshKeys {
    /// ConfigRef is a reference to a resource containing the keys. The reference is optional to allow users/operators to specify Bootstrap.DataSecretName without the need of a controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<KubevirtClusterTemplateTemplateSpecSshKeysConfigRef>,
    /// DataSecretName is the name of the secret that stores ssh keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSecretName")]
    pub data_secret_name: Option<String>,
}

/// ConfigRef is a reference to a resource containing the keys. The reference is optional to allow users/operators to specify Bootstrap.DataSecretName without the need of a controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterTemplateTemplateSpecSshKeysConfigRef {
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

