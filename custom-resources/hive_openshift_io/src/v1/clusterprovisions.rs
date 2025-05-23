// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/openshift/hive/hive.openshift.io/v1/clusterprovisions.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterProvisionSpec defines the results of provisioning a cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "ClusterProvision", plural = "clusterprovisions")]
#[kube(namespaced)]
#[kube(status = "ClusterProvisionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterProvisionSpec {
    /// AdminKubeconfigSecretRef references the secret containing the admin kubeconfig for this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminKubeconfigSecretRef")]
    pub admin_kubeconfig_secret_ref: Option<ClusterProvisionAdminKubeconfigSecretRef>,
    /// AdminPasswordSecretRef references the secret containing the admin username/password which can be used to login to this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPasswordSecretRef")]
    pub admin_password_secret_ref: Option<ClusterProvisionAdminPasswordSecretRef>,
    /// Attempt is which attempt number of the cluster deployment that this ClusterProvision is
    pub attempt: i64,
    /// ClusterDeploymentRef references the cluster deployment provisioned.
    #[serde(rename = "clusterDeploymentRef")]
    pub cluster_deployment_ref: ClusterProvisionClusterDeploymentRef,
    /// ClusterID is a globally unique identifier for this cluster generated during installation. Used for reporting metrics among other places.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterID")]
    pub cluster_id: Option<String>,
    /// InfraID is an identifier for this cluster generated during installation and used for tagging/naming resources in cloud providers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infraID")]
    pub infra_id: Option<String>,
    /// InstallLog is the log from the installer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installLog")]
    pub install_log: Option<String>,
    /// Metadata is the metadata.json generated by the installer, providing metadata information about the cluster created.
    /// NOTE: This is not used because it didn't work (it was always empty). We think because the thing it's storing
    /// (ClusterMetadata from installer) is not a runtime.Object, so can't be put in a RawExtension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    /// MetadataJSON is a JSON representation of the ClusterMetadata produced by the installer. We don't use a
    /// runtime.RawExtension because ClusterMetadata isn't a runtime.Object. We don't use ClusterMetadata itself
    /// because we don't want our API consumers to need to pull in the installer code and its dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataJSON")]
    pub metadata_json: Option<String>,
    /// PrevClusterID is the cluster ID of the previous failed provision attempt.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prevClusterID")]
    pub prev_cluster_id: Option<String>,
    /// PrevInfraID is the infra ID of the previous failed provision attempt.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prevInfraID")]
    pub prev_infra_id: Option<String>,
    /// PrevProvisionName is the name of the previous failed provision attempt.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prevProvisionName")]
    pub prev_provision_name: Option<String>,
    /// Stage is the stage of provisioning that the cluster deployment has reached.
    pub stage: String,
}

/// AdminKubeconfigSecretRef references the secret containing the admin kubeconfig for this cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProvisionAdminKubeconfigSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AdminPasswordSecretRef references the secret containing the admin username/password which can be used to login to this cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProvisionAdminPasswordSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ClusterDeploymentRef references the cluster deployment provisioned.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProvisionClusterDeploymentRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ClusterProvisionStatus defines the observed state of ClusterProvision.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProvisionStatus {
    /// Conditions includes more detailed status for the cluster provision
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// JobRef is the reference to the job performing the provision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobRef")]
    pub job_ref: Option<ClusterProvisionStatusJobRef>,
}

/// JobRef is the reference to the job performing the provision.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProvisionStatusJobRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

