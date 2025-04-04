// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/pulp/pulp-operator/repo-manager.pulpproject.org/v1beta2/pulprestores.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// PulpRestoreSpec defines the desired state of PulpRestore
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "repo-manager.pulpproject.org", version = "v1beta2", kind = "PulpRestore", plural = "pulprestores")]
#[kube(namespaced)]
#[kube(status = "PulpRestoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PulpRestoreSpec {
    /// Backup directory name, set as a status found on the backup object (backupDirectory)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_dir: Option<String>,
    /// Name of the backup custom resource
    pub backup_name: String,
    /// Name of the PVC to be restored from, set as a status found on the backup object (backupClaim)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_pvc: Option<String>,
    /// Name of the deployment to be restored to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Name of the deployment type. Can be one of {galaxy,pulp}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<PulpRestoreDeploymentType>,
    /// KeepBackupReplicasCount allows to define if the restore controller should restore the components with the
    /// same number of replicas from backup or restore only a single replica each.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_replicas: Option<bool>,
}

/// PulpRestoreSpec defines the desired state of PulpRestore
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PulpRestoreDeploymentType {
    #[serde(rename = "galaxy")]
    Galaxy,
    #[serde(rename = "pulp")]
    Pulp,
}

/// PulpRestoreStatus defines the observed state of PulpRestore
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PulpRestoreStatus {
    pub conditions: Vec<Condition>,
    pub postgres_secret: String,
}

