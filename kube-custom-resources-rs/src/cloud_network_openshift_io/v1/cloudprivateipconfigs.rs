// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/cloud.network.openshift.io/v1/cloudprivateipconfigs.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// spec is the definition of the desired private IP request.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "cloud.network.openshift.io", version = "v1", kind = "CloudPrivateIPConfig", plural = "cloudprivateipconfigs")]
#[kube(status = "CloudPrivateIPConfigStatus")]
#[kube(schema = "disabled")]
pub struct CloudPrivateIPConfigSpec {
    /// node is the node name, as specified by the Kubernetes field: node.metadata.name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

/// status is the observed status of the desired private IP request. Read-only.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CloudPrivateIPConfigStatus {
    /// condition is the assignment condition of the private IP and its status
    pub conditions: Vec<Condition>,
    /// node is the node name, as specified by the Kubernetes field: node.metadata.name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

