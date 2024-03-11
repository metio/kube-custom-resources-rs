// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/aws-application-networking-k8s/application-networking.k8s.aws/v1alpha1/serviceexports.yaml --derive=PartialEq
// kopium version: 0.17.1


use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// status describes the current state of an exported service. Service configuration comes from the Service that had the same name and namespace as this ServiceExport. Populated by the multi-cluster service implementation's controller.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServiceExportStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

