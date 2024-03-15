// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/k8up-io/k8up/k8up.io/v1/checks.yaml --derive=PartialEq
// kopium version: 0.17.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// CheckSpec defines the desired state of Check. It needs to contain the repository
/// information.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "k8up.io", version = "v1", kind = "Check", plural = "checks")]
#[kube(namespaced)]
#[kube(status = "CheckStatus")]
#[kube(schema = "disabled")]
pub struct CheckSpec {
    /// ActiveDeadlineSeconds specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it.
    /// Value must be positive integer if given.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDeadlineSeconds")]
    pub active_deadline_seconds: Option<i64>,
    /// Backend contains the restic repo where the job should backup to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<CheckBackend>,
    /// FailedJobsHistoryLimit amount of failed jobs to keep for later analysis.
    /// KeepJobs is used property is not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedJobsHistoryLimit")]
    pub failed_jobs_history_limit: Option<i64>,
    /// KeepJobs amount of jobs to keep for later analysis.
    /// 
    /// 
    /// Deprecated: Use FailedJobsHistoryLimit and SuccessfulJobsHistoryLimit respectively.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepJobs")]
    pub keep_jobs: Option<i64>,
    /// PodSecurityContext describes the security context with which this action shall be executed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSecurityContext")]
    pub pod_security_context: Option<CheckPodSecurityContext>,
    /// PromURL sets a prometheus push URL where the backup container send metrics to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "promURL")]
    pub prom_url: Option<String>,
    /// Resources describes the compute resource requirements (cpu, memory, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CheckResources>,
    /// SuccessfulJobsHistoryLimit amount of successful jobs to keep for later analysis.
    /// KeepJobs is used property is not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulJobsHistoryLimit")]
    pub successful_jobs_history_limit: Option<i64>,
}

/// Backend contains the restic repo where the job should backup to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackend {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<CheckBackendAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b2: Option<CheckBackendB2>,
    /// EnvFrom adds all environment variables from a an external source to the Restic job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envFrom")]
    pub env_from: Option<Vec<CheckBackendEnvFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<CheckBackendGcs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<CheckBackendLocal>,
    /// RepoPasswordSecretRef references a secret key to look up the restic repository password
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repoPasswordSecretRef")]
    pub repo_password_secret_ref: Option<CheckBackendRepoPasswordSecretRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest: Option<CheckBackendRest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<CheckBackendS3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift: Option<CheckBackendSwift>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendAzure {
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountKeySecretRef")]
    pub account_key_secret_ref: Option<CheckBackendAzureAccountKeySecretRef>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountNameSecretRef")]
    pub account_name_secret_ref: Option<CheckBackendAzureAccountNameSecretRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendAzureAccountKeySecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendAzureAccountNameSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendB2 {
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountIDSecretRef")]
    pub account_id_secret_ref: Option<CheckBackendB2AccountIdSecretRef>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountKeySecretRef")]
    pub account_key_secret_ref: Option<CheckBackendB2AccountKeySecretRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendB2AccountIdSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendB2AccountKeySecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendEnvFrom {
    /// The ConfigMap to select from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapRef")]
    pub config_map_ref: Option<CheckBackendEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<CheckBackendEnvFromSecretRef>,
}

/// The ConfigMap to select from
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendEnvFromConfigMapRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The Secret to select from
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendEnvFromSecretRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendGcs {
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessTokenSecretRef")]
    pub access_token_secret_ref: Option<CheckBackendGcsAccessTokenSecretRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectIDSecretRef")]
    pub project_id_secret_ref: Option<CheckBackendGcsProjectIdSecretRef>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendGcsAccessTokenSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendGcsProjectIdSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendLocal {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
}

/// RepoPasswordSecretRef references a secret key to look up the restic repository password
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendRepoPasswordSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendRest {
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordSecretReg")]
    pub password_secret_reg: Option<CheckBackendRestPasswordSecretReg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userSecretRef")]
    pub user_secret_ref: Option<CheckBackendRestUserSecretRef>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendRestPasswordSecretReg {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendRestUserSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendS3 {
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessKeyIDSecretRef")]
    pub access_key_id_secret_ref: Option<CheckBackendS3AccessKeyIdSecretRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretAccessKeySecretRef")]
    pub secret_access_key_secret_ref: Option<CheckBackendS3SecretAccessKeySecretRef>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendS3AccessKeyIdSecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendS3SecretAccessKeySecretRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckBackendSwift {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// PodSecurityContext describes the security context with which this action shall be executed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckPodSecurityContext {
    /// A special supplemental group that applies to all containers in a pod.
    /// Some volume types allow the Kubelet to change the ownership of that volume
    /// to be owned by the pod:
    /// 
    /// 
    /// 1. The owning GID will be the FSGroup
    /// 2. The setgid bit is set (new files created in the volume will be owned by FSGroup)
    /// 3. The permission bits are OR'd with rw-rw----
    /// 
    /// 
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroup")]
    pub fs_group: Option<i64>,
    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume
    /// before being exposed inside Pod. This field will only apply to
    /// volume types which support fsGroup based ownership(and permissions).
    /// It will have no effect on ephemeral volume types such as: secret, configmaps
    /// and emptydir.
    /// Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroupChangePolicy")]
    pub fs_group_change_policy: Option<String>,
    /// The GID to run the entrypoint of the container process.
    /// Uses runtime default if unset.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence
    /// for that container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsGroup")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user.
    /// If true, the Kubelet will validate the image at runtime to ensure that it
    /// does not run as UID 0 (root) and fail to start the container if it does.
    /// If unset or false, no such validation will be performed.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process.
    /// Defaults to user specified in image metadata if unspecified.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence
    /// for that container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUser")]
    pub run_as_user: Option<i64>,
    /// The SELinux context to be applied to all containers.
    /// If unspecified, the container runtime will allocate a random SELinux context for each
    /// container.  May also be set in SecurityContext.  If set in
    /// both SecurityContext and PodSecurityContext, the value specified in SecurityContext
    /// takes precedence for that container.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seLinuxOptions")]
    pub se_linux_options: Option<CheckPodSecurityContextSeLinuxOptions>,
    /// The seccomp options to use by the containers in this pod.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seccompProfile")]
    pub seccomp_profile: Option<CheckPodSecurityContextSeccompProfile>,
    /// A list of groups applied to the first process run in each container, in addition
    /// to the container's primary GID, the fsGroup (if specified), and group memberships
    /// defined in the container image for the uid of the container process. If unspecified,
    /// no additional groups are added to any container. Note that group memberships
    /// defined in the container image for the uid of the container process are still effective,
    /// even if they are not included in this list.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supplementalGroups")]
    pub supplemental_groups: Option<Vec<i64>>,
    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported
    /// sysctls (by the container runtime) might fail to launch.
    /// Note that this field cannot be set when spec.os.name is windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Vec<CheckPodSecurityContextSysctls>>,
    /// The Windows specific settings applied to all containers.
    /// If unspecified, the options within a container's SecurityContext will be used.
    /// If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    /// Note that this field cannot be set when spec.os.name is linux.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "windowsOptions")]
    pub windows_options: Option<CheckPodSecurityContextWindowsOptions>,
}

/// The SELinux context to be applied to all containers.
/// If unspecified, the container runtime will allocate a random SELinux context for each
/// container.  May also be set in SecurityContext.  If set in
/// both SecurityContext and PodSecurityContext, the value specified in SecurityContext
/// takes precedence for that container.
/// Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckPodSecurityContextSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The seccomp options to use by the containers in this pod.
/// Note that this field cannot be set when spec.os.name is windows.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckPodSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used.
    /// The profile must be preconfigured on the node to work.
    /// Must be a descending path, relative to the kubelet's configured seccomp profile location.
    /// Must be set if type is "Localhost". Must NOT be set for any other type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied.
    /// Valid options are:
    /// 
    /// 
    /// Localhost - a profile defined in a file on the node should be used.
    /// RuntimeDefault - the container runtime default profile should be used.
    /// Unconfined - no profile should be applied.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Sysctl defines a kernel parameter to be set
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckPodSecurityContextSysctls {
    /// Name of a property to set
    pub name: String,
    /// Value of a property to set
    pub value: String,
}

/// The Windows specific settings applied to all containers.
/// If unspecified, the options within a container's SecurityContext will be used.
/// If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
/// Note that this field cannot be set when spec.os.name is linux.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckPodSecurityContextWindowsOptions {
    /// GMSACredentialSpec is where the GMSA admission webhook
    /// (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
    /// GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpec")]
    pub gmsa_credential_spec: Option<String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gmsaCredentialSpecName")]
    pub gmsa_credential_spec_name: Option<String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container.
    /// All of a Pod's containers must have the same effective HostProcess value
    /// (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
    /// In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostProcess")]
    pub host_process: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process.
    /// Defaults to the user specified in image metadata if unspecified.
    /// May also be set in PodSecurityContext. If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUserName")]
    pub run_as_user_name: Option<String>,
}

/// Resources describes the compute resource requirements (cpu, memory, etc.)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<CheckResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// Status defines the observed state of a generic K8up job. It is used for the
/// operator to determine what to do.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckStatus {
    /// Conditions provide a standard mechanism for higher-level status reporting from a controller.
    /// They are an extension mechanism which allows tools and other controllers to collect summary information about
    /// resources without needing to understand resource-specific status details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
}

