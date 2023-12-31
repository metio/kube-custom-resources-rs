// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/redhat-developer/service-binding-operator/servicebinding.io/v1alpha3/servicebindings.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ServiceBindingSpec defines the desired state of ServiceBinding
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "servicebinding.io", version = "v1alpha3", kind = "ServiceBinding", plural = "servicebindings")]
#[kube(namespaced)]
#[kube(status = "ServiceBindingStatus")]
#[kube(schema = "disabled")]
pub struct ServiceBindingSpec {
    /// Env is the collection of mappings from Secret entries to environment variables
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ServiceBindingEnv>>,
    /// Name is the name of the service as projected into the workload container.  Defaults to .metadata.name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Provider is the provider of the service as projected into the workload container
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// Service is a reference to an object that fulfills the ProvisionedService duck type
    pub service: ServiceBindingService,
    /// Type is the type of the service as projected into the workload container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Workload is a reference to an object
    pub workload: ServiceBindingWorkload,
}

/// EnvMapping defines a mapping from the value of a Secret entry to an environment variable
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingEnv {
    /// Key is the key in the Secret that will be exposed
    pub key: String,
    /// Name is the name of the environment variable
    pub name: String,
}

/// Service is a reference to an object that fulfills the ProvisionedService duck type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingService {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// Workload is a reference to an object
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingWorkload {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Containers describes which containers in a Pod should be bound to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<String>>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selector is a query that selects the workload or workloads to bind the service to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<ServiceBindingWorkloadSelector>,
}

/// Selector is a query that selects the workload or workloads to bind the service to
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingWorkloadSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ServiceBindingWorkloadSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingWorkloadSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ServiceBindingStatus defines the observed state of ServiceBinding
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingStatus {
    /// Binding exposes the projected secret for this ServiceBinding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<ServiceBindingStatusBinding>,
    /// Conditions are the conditions of this ServiceBinding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ServiceBindingStatusConditions>>,
    /// ObservedGeneration is the 'Generation' of the ServiceBinding that was last processed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Binding exposes the projected secret for this ServiceBinding
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingStatusBinding {
    /// Name of the referent secret. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  	type FooStatus struct{ 	    // Represents the observations of a foo's current state. 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" 	    // +patchMergeKey=type 	    // +patchStrategy=merge 	    // +listType=map 	    // +listMapKey=type 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  	    // other fields 	}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceBindingStatusConditions {
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
    pub status: ServiceBindingStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  	type FooStatus struct{ 	    // Represents the observations of a foo's current state. 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" 	    // +patchMergeKey=type 	    // +patchStrategy=merge 	    // +listType=map 	    // +listMapKey=type 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  	    // other fields 	}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceBindingStatusConditionsStatus {
    True,
    False,
    Unknown,
}

