// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/mattermost/mattermost-operator/mattermost.com/v1alpha1/mattermostrestoredbs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// MattermostRestoreDBSpec defines the desired state of MattermostRestoreDB
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "mattermost.com", version = "v1alpha1", kind = "MattermostRestoreDB", plural = "mattermostrestoredbs")]
#[kube(namespaced)]
#[kube(status = "MattermostRestoreDBStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MattermostRestoreDBSpec {
    /// InitBucketURL defines where the DB backup file is located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initBucketURL")]
    pub init_bucket_url: Option<String>,
    /// MattermostClusterName defines the ClusterInstallation name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mattermostClusterName")]
    pub mattermost_cluster_name: Option<String>,
    /// MattermostDBName defines the database name.
    /// Need to set if different from `mattermost`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mattermostDBName")]
    pub mattermost_db_name: Option<String>,
    /// MattermostDBPassword defines the user password to access the database.
    /// Need to set if the user is different from the one created by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mattermostDBPassword")]
    pub mattermost_db_password: Option<String>,
    /// MattermostDBUser defines the user to access the database.
    /// Need to set if the user is different from `mmuser`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mattermostDBUser")]
    pub mattermost_db_user: Option<String>,
    /// RestoreSecret defines the secret that holds the credentials to
    /// MySQL Operator be able to download the DB backup file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreSecret")]
    pub restore_secret: Option<String>,
}

/// MattermostRestoreDBStatus defines the observed state of MattermostRestoreDB
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MattermostRestoreDBStatus {
    /// The original number of database replicas. will be used to restore after applying the db restore process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "originalDBReplicas")]
    pub original_db_replicas: Option<i32>,
    /// Represents the state of the Mattermost restore Database.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

