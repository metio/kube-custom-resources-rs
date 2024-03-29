// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeshop/testkube-operator/executor.testkube.io/v1/webhooks.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// WebhookSpec defines the desired state of Webhook
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "executor.testkube.io", version = "v1", kind = "Webhook", plural = "webhooks")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct WebhookSpec {
    /// Events declare list if events on which webhook should be called
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// webhook headers (golang template supported)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    /// will load the generated payload for notification inside the object
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payloadObjectField")]
    pub payload_object_field: Option<String>,
    /// golang based template for notification payload
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payloadTemplate")]
    pub payload_template: Option<String>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payloadTemplateReference")]
    pub payload_template_reference: Option<String>,
    /// Labels to filter for tests and test suites
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Uri is address where webhook should be made (golang template supported)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// WebhookStatus defines the observed state of Webhook
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WebhookStatus {
}

