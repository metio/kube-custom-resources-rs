// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/eryalito/kubensync-operator/automation.kubensync.com/v1alpha1/managedresources.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ManagedResourceSpec defines the desired state of ManagedResource
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "automation.kubensync.com", version = "v1alpha1", kind = "ManagedResource", plural = "managedresources")]
#[kube(status = "ManagedResourceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ManagedResourceSpec {
    /// AvoidResourceUpdate defines if the created resources should be updated if they already exists. Default value is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "avoidResourceUpdate")]
    pub avoid_resource_update: Option<bool>,
    /// ManagedResourceSpecNamespaceSelector defines the selector used to specify which namespaces are affected
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ManagedResourceNamespaceSelector>,
    /// ManagedResourceSpecTemplate defines the resources to be created when a namespace matches the selector
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ManagedResourceTemplate>,
}

/// ManagedResourceSpecNamespaceSelector defines the selector used to specify which namespaces are affected
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceNamespaceSelector {
    /// Labels that the namespace must have to be selected
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<ManagedResourceNamespaceSelectorLabelSelector>,
    /// Regex that the namespace name must match to be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// Labels that the namespace must have to be selected
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceNamespaceSelectorLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ManagedResourceNamespaceSelectorLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceNamespaceSelectorLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ManagedResourceSpecTemplate defines the resources to be created when a namespace matches the selector
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceTemplate {
    /// Data defines a set of refences to secrets or configmaps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ManagedResourceTemplateData>>,
    /// Literal defines a go template to be renderized for each namespace matching the selector
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
}

/// Describes extra data that will be loaded into the go template as inputs. They all will be inside `.Data` parent and all Secret/ConfigMap keys will be loaded. The format inside the template would look as follows `.Data.${Name}.${Key}`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceTemplateData {
    /// Name of the key where the contents will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines the reference to the resource that should be imported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ref")]
    pub r#ref: Option<ManagedResourceTemplateDataRef>,
    /// Defines the kind of resource the ref is pointing to. Could be `Secret` or `ConfigMap`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Defines the reference to the resource that should be imported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceTemplateDataRef {
    /// Name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// ManagedResourceStatus defines the observed state of ManagedResource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdResources")]
    pub created_resources: Option<Vec<ManagedResourceStatusCreatedResources>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedResourceStatusCreatedResources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerNamespace")]
    pub trigger_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

