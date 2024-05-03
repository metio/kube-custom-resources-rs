// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/metacontroller/metacontroller/metacontroller.k8s.io/v1alpha1/decoratorcontrollers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "metacontroller.k8s.io", version = "v1alpha1", kind = "DecoratorController", plural = "decoratorcontrollers")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DecoratorControllerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<DecoratorControllerAttachments>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<DecoratorControllerHooks>,
    pub resources: Vec<DecoratorControllerResources>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriodSeconds")]
    pub resync_period_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerAttachments {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<DecoratorControllerAttachmentsUpdateStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerAttachmentsUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<DecoratorControllerAttachmentsUpdateStrategyMethod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerAttachmentsUpdateStrategyMethod {
    OnDelete,
    Recreate,
    InPlace,
    RollingRecreate,
    RollingInPlace,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customize: Option<DecoratorControllerHooksCustomize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize: Option<DecoratorControllerHooksFinalize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<DecoratorControllerHooksSync>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksCustomize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<DecoratorControllerHooksCustomizeVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<DecoratorControllerHooksCustomizeWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksCustomizeVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksCustomizeWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<DecoratorControllerHooksCustomizeWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<DecoratorControllerHooksCustomizeWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<DecoratorControllerHooksCustomizeWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksCustomizeWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksCustomizeWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksCustomizeWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksFinalize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<DecoratorControllerHooksFinalizeVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<DecoratorControllerHooksFinalizeWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksFinalizeVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksFinalizeWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<DecoratorControllerHooksFinalizeWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<DecoratorControllerHooksFinalizeWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<DecoratorControllerHooksFinalizeWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksFinalizeWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksFinalizeWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksFinalizeWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksSync {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<DecoratorControllerHooksSyncVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<DecoratorControllerHooksSyncWebhook>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksSyncVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksSyncWebhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<DecoratorControllerHooksSyncWebhookEtag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Sets the json unmarshall mode. One of the 'loose' or 'strict'. In 'strict' mode additional checks are performed to detect unknown and duplicated fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseUnMarshallMode")]
    pub response_un_marshall_mode: Option<DecoratorControllerHooksSyncWebhookResponseUnMarshallMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<DecoratorControllerHooksSyncWebhookService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksSyncWebhookEtag {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheCleanupSeconds")]
    pub cache_cleanup_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheTimeoutSeconds")]
    pub cache_timeout_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DecoratorControllerHooksSyncWebhookResponseUnMarshallMode {
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "strict")]
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerHooksSyncWebhookService {
    pub name: String,
    pub namespace: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerResources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelector")]
    pub annotation_selector: Option<DecoratorControllerResourcesAnnotationSelector>,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<DecoratorControllerResourcesLabelSelector>,
    pub resource: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerResourcesAnnotationSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchAnnotations")]
    pub match_annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DecoratorControllerResourcesAnnotationSelectorMatchExpressions>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerResourcesAnnotationSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerResourcesLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DecoratorControllerResourcesLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerResourcesLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DecoratorControllerStatus {
}

