// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/everest-operator/everest.percona.com/v1alpha1/monitoringconfigs.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// MonitoringConfigSpec defines the desired state of MonitoringConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "everest.percona.com", version = "v1alpha1", kind = "MonitoringConfig", plural = "monitoringconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct MonitoringConfigSpec {
    /// AllowedNamespaces is the list of namespaces where the operator will copy secrets provided in the CredentialsSecretsName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedNamespaces")]
    pub allowed_namespaces: Option<Vec<String>>,
    /// CredentialsSecretName is the name of the secret with credentials.
    #[serde(rename = "credentialsSecretName")]
    pub credentials_secret_name: String,
    /// PMM is configuration for the PMM monitoring type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmm: Option<MonitoringConfigPmm>,
    /// Type is type of monitoring.
    #[serde(rename = "type")]
    pub r#type: MonitoringConfigType,
}

/// PMM is configuration for the PMM monitoring type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MonitoringConfigPmm {
    /// Image is a Docker image name to use for deploying PMM client. Defaults to using the latest version.
    pub image: String,
    /// URL is url to the monitoring instance.
    pub url: String,
}

/// MonitoringConfigSpec defines the desired state of MonitoringConfig.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MonitoringConfigType {
    #[serde(rename = "pmm")]
    Pmm,
}

/// MonitoringConfigStatus defines the observed state of MonitoringConfig.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MonitoringConfigStatus {
}

