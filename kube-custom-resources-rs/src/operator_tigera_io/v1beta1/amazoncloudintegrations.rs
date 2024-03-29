// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1beta1/amazoncloudintegrations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// AmazonCloudIntegrationSpec defines the desired state of AmazonCloudIntegration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1beta1", kind = "AmazonCloudIntegration", plural = "amazoncloudintegrations")]
#[kube(schema = "disabled")]
pub struct AmazonCloudIntegrationSpec {
}

/// AmazonCloudIntegrationStatus defines the observed state of AmazonCloudIntegration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AmazonCloudIntegrationStatus {
}

