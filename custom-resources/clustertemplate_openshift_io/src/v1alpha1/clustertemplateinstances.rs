// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/stolostron/cluster-templates-operator/clustertemplate.openshift.io/v1alpha1/clustertemplateinstances.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "clustertemplate.openshift.io", version = "v1alpha1", kind = "ClusterTemplateInstance", plural = "clustertemplateinstances")]
#[kube(namespaced)]
#[kube(status = "ClusterTemplateInstanceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterTemplateInstanceSpec {
    /// A reference to ClusterTemplate which will be used for installing and setting up the cluster
    #[serde(rename = "clusterTemplateRef")]
    pub cluster_template_ref: String,
    /// A reference to a secret which contains kubeconfig of the cluster. If specified day1 operation won't be executed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeconfigSecretRef")]
    pub kubeconfig_secret_ref: Option<String>,
    /// Helm parameters to be passed to cluster installation or setup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ClusterTemplateInstanceParameters>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceParameters {
    /// Name of the application set to which parameter is applied
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSetup")]
    pub cluster_setup: Option<String>,
    /// Name of the Helm parameter
    pub name: String,
    /// Value of the Helm parameter
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatus {
    /// A reference for secret which contains username and password under keys "username" and "password"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPassword")]
    pub admin_password: Option<ClusterTemplateInstanceStatusAdminPassword>,
    /// API server URL of the new cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiServerURL")]
    pub api_server_url: Option<String>,
    /// Status of each cluster setup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSetup")]
    pub cluster_setup: Option<Vec<ClusterTemplateInstanceStatusClusterSetup>>,
    /// Secrets create by cluster setup which provide credentials for applications created by cluster setup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSetupSecrets")]
    pub cluster_setup_secrets: Option<Vec<ClusterTemplateInstanceStatusClusterSetupSecrets>>,
    /// Resource conditions
    pub conditions: Vec<Condition>,
    /// Console URL of the new cluster. The value is taken from ManagedCluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consoleURL")]
    pub console_url: Option<String>,
    /// Time of first attempt of login to a new cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firstLoginAttempt")]
    pub first_login_attempt: Option<String>,
    /// A reference for secret which contains kubeconfig under key "kubeconfig"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubeconfig: Option<ClusterTemplateInstanceStatusKubeconfig>,
    /// A reference to ManagedCluster resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedCluster")]
    pub managed_cluster: Option<ClusterTemplateInstanceStatusManagedCluster>,
    /// Additional message for Phase
    pub message: String,
    /// Represents instance installaton & setup phase
    pub phase: String,
}

/// A reference for secret which contains username and password under keys "username" and "password"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatusAdminPassword {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatusClusterSetup {
    /// Description of the cluster setup status
    pub message: String,
    /// Name of the cluster setup
    pub name: String,
    /// Status of the cluster setup
    pub status: String,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatusClusterSetupSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A reference for secret which contains kubeconfig under key "kubeconfig"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatusKubeconfig {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A reference to ManagedCluster resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateInstanceStatusManagedCluster {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

