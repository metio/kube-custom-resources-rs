// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename ./crd-catalog/devfile/registry-operator/registry.devfile.io/v1alpha1/devfileregistries.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// DevfileRegistrySpec defines the desired state of DevfileRegistry
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "registry.devfile.io", version = "v1alpha1", kind = "DevfileRegistry", plural = "devfileregistries")]
#[kube(namespaced)]
#[kube(status = "DevfileRegistryStatus")]
#[kube(schema = "disabled")]
pub struct DevfileRegistrySpec {
    /// Sets the devfile index container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileIndex")]
    pub devfile_index: Option<DevfileRegistryDevfileIndex>,
    /// Sets the container image containing devfile stacks to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileIndexImage")]
    pub devfile_index_image: Option<String>,
    /// Sets the registry server deployment to run under headless mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headless: Option<bool>,
    /// DevfileRegistrySpecK8sOnly defines the desired state of the kubernetes-only fields of the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k8s: Option<DevfileRegistryK8s>,
    /// Sets the OCI registry container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ociRegistry")]
    pub oci_registry: Option<DevfileRegistryOciRegistry>,
    /// Overrides the container image used for the OCI registry. Recommended to leave blank and default to the image specified by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ociRegistryImage")]
    pub oci_registry_image: Option<String>,
    /// Sets the registry viewer container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewer")]
    pub registry_viewer: Option<DevfileRegistryRegistryViewer>,
    /// Overrides the container image used for the registry viewer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewerImage")]
    pub registry_viewer_image: Option<String>,
    /// DevfileRegistrySpecStorage defines the desired state of the storage for the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<DevfileRegistryStorage>,
    /// Telemetry defines the desired state for telemetry in the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<DevfileRegistryTelemetry>,
    /// DevfileRegistrySpecTLS defines the desired state for TLS in the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<DevfileRegistryTls>,
}

/// Sets the devfile index container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryDevfileIndex {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
}

/// DevfileRegistrySpecK8sOnly defines the desired state of the kubernetes-only fields of the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryK8s {
    /// Ingress domain for a Kubernetes cluster. This MUST be explicitly specified on Kubernetes. There are no defaults
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressDomain")]
    pub ingress_domain: Option<String>,
}

/// Sets the OCI registry container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryOciRegistry {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
}

/// Sets the registry viewer container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryRegistryViewer {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
}

/// DevfileRegistrySpecStorage defines the desired state of the storage for the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryStorage {
    /// Instructs the operator to deploy the DevfileRegistry with persistent storage Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Configures the size of the devfile registry's persistent volume, if enabled. Defaults to 1Gi.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryVolumeSize")]
    pub registry_volume_size: Option<String>,
}

/// Telemetry defines the desired state for telemetry in the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryTelemetry {
    /// Specify a telemetry key to allow devfile specific data to be sent to a client's own Segment analytics source. If the write key is specified then telemetry will be enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The registry name (can be any string) that is used as identifier for devfile telemetry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryName")]
    pub registry_name: Option<String>,
    /// Specify a telemetry write key for the registry viewer component to allow data to be sent to a client's own Segment analytics source. If the write key is specified then telemetry for the registry viewer component will be enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewerWriteKey")]
    pub registry_viewer_write_key: Option<String>,
}

/// DevfileRegistrySpecTLS defines the desired state for TLS in the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryTls {
    /// Instructs the operator to deploy the DevfileRegistry with TLS enabled. Enabled by default. Disabling is only recommended for development or test.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Name of an optional, pre-existing TLS secret to use for TLS termination on ingress/route resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// DevfileRegistryStatus defines the observed state of DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryStatus {
    /// Conditions shows the state devfile registries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DevfileRegistryStatusConditions>>,
    /// URL is the exposed URL for the Devfile Registry, and is set in the status after the registry has become available.
    pub url: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DevfileRegistryStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: DevfileRegistryStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DevfileRegistryStatusConditionsStatus {
    True,
    False,
    Unknown,
}

