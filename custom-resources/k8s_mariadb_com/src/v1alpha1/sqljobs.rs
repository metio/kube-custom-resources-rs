// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/mariadb-operator/mariadb-operator/k8s.mariadb.com/v1alpha1/sqljobs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SqlJobSpec defines the desired state of SqlJob
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.mariadb.com", version = "v1alpha1", kind = "SqlJob", plural = "sqljobs")]
#[kube(namespaced)]
#[kube(status = "SqlJobStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SqlJobSpec {
    /// Affinity to be used in the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<SqlJobAffinity>,
    /// Args to be used in the Container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// BackoffLimit defines the maximum number of attempts to successfully execute a SqlJob.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffLimit")]
    pub backoff_limit: Option<i32>,
    /// Username to be used when executing the SqlJob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// DependsOn defines dependencies with other SqlJob objectecs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dependsOn")]
    pub depends_on: Option<Vec<SqlJobDependsOn>>,
    /// FailedJobsHistoryLimit defines the maximum number of failed Jobs to be displayed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedJobsHistoryLimit")]
    pub failed_jobs_history_limit: Option<i32>,
    /// ImagePullSecrets is the list of pull Secrets to be used to pull the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<SqlJobImagePullSecrets>>,
    /// InheritMetadata defines the metadata to be inherited by children resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inheritMetadata")]
    pub inherit_metadata: Option<SqlJobInheritMetadata>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: SqlJobMariaDbRef,
    /// NodeSelector to be used in the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// UserPasswordSecretKeyRef is a reference to the impersonated user's password to be used when executing the SqlJob.
    #[serde(rename = "passwordSecretKeyRef")]
    pub password_secret_key_ref: SqlJobPasswordSecretKeyRef,
    /// PodMetadata defines extra metadata for the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podMetadata")]
    pub pod_metadata: Option<SqlJobPodMetadata>,
    /// SecurityContext holds pod-level security attributes and common container settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSecurityContext")]
    pub pod_security_context: Option<SqlJobPodSecurityContext>,
    /// PriorityClassName to be used in the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityClassName")]
    pub priority_class_name: Option<String>,
    /// Resouces describes the compute resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<SqlJobResources>,
    /// RestartPolicy to be added to the SqlJob Pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restartPolicy")]
    pub restart_policy: Option<SqlJobRestartPolicy>,
    /// Schedule defines when the SqlJob will be executed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<SqlJobSchedule>,
    /// SecurityContext holds security configuration that will be applied to a container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityContext")]
    pub security_context: Option<SqlJobSecurityContext>,
    /// ServiceAccountName is the name of the ServiceAccount to be used by the Pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// Sql is the script to be executed by the SqlJob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    /// SqlConfigMapKeyRef is a reference to a ConfigMap containing the Sql script.
    /// It is defaulted to a ConfigMap with the contents of the Sql field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sqlConfigMapKeyRef")]
    pub sql_config_map_key_ref: Option<SqlJobSqlConfigMapKeyRef>,
    /// SuccessfulJobsHistoryLimit defines the maximum number of successful Jobs to be displayed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulJobsHistoryLimit")]
    pub successful_jobs_history_limit: Option<i32>,
    /// TimeZone defines the timezone associated with the cron expression.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
    /// Tolerations to be used in the Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<SqlJobTolerations>>,
    /// Username to be impersonated when executing the SqlJob.
    pub username: String,
}

/// Affinity to be used in the Pod.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinity {
    /// AntiAffinityEnabled configures PodAntiAffinity so each Pod is scheduled in a different Node, enabling HA.
    /// Make sure you have at least as many Nodes available as the replicas to not end up with unscheduled Pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "antiAffinityEnabled")]
    pub anti_affinity_enabled: Option<bool>,
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeaffinity-v1-core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<SqlJobAffinityNodeAffinity>,
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#podantiaffinity-v1-core.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAntiAffinity")]
    pub pod_anti_affinity: Option<SqlJobAffinityPodAntiAffinity>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeaffinity-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselector-v1-core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#preferredschedulingterm-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorterm-v1-core
    pub preference: SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    pub weight: i32,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorterm-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorrequirement-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    pub key: String,
    /// A node selector operator is the set of operators that can be used in
    /// a node selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorrequirement-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    pub key: String,
    /// A node selector operator is the set of operators that can be used in
    /// a node selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselector-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorterm-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorrequirement-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    pub key: String,
    /// A node selector operator is the set of operators that can be used in
    /// a node selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#nodeselectorrequirement-v1-core
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    pub key: String,
    /// A node selector operator is the set of operators that can be used in
    /// a node selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#podantiaffinity-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#weightedpodaffinityterm-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#podaffinityterm-v1-core.
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#podaffinityterm-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselectorrequirement-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    /// A label selector operator is the set of operators that can be used in a selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#podaffinityterm-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    /// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#labelselectorrequirement-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobAffinityPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    /// A label selector operator is the set of operators that can be used in a selector requirement.
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#localobjectreference-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobDependsOn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Refer to the Kubernetes docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.31/#localobjectreference-v1-core.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobImagePullSecrets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// InheritMetadata defines the metadata to be inherited by children resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobInheritMetadata {
    /// Annotations to be added to children resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels to be added to children resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobMariaDbRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// WaitForIt indicates whether the controller using this reference should wait for MariaDB to be ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitForIt")]
    pub wait_for_it: Option<bool>,
}

/// UserPasswordSecretKeyRef is a reference to the impersonated user's password to be used when executing the SqlJob.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPasswordSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PodMetadata defines extra metadata for the Pod.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPodMetadata {
    /// Annotations to be added to children resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels to be added to children resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// SecurityContext holds pod-level security attributes and common container settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPodSecurityContext {
    /// AppArmorProfile defines a pod or container's AppArmor settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appArmorProfile")]
    pub app_armor_profile: Option<SqlJobPodSecurityContextAppArmorProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroup")]
    pub fs_group: Option<i64>,
    /// PodFSGroupChangePolicy holds policies that will be used for applying fsGroup to a volume
    /// when volume is mounted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroupChangePolicy")]
    pub fs_group_change_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsGroup")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUser")]
    pub run_as_user: Option<i64>,
    /// SELinuxOptions are the labels to be applied to the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seLinuxOptions")]
    pub se_linux_options: Option<SqlJobPodSecurityContextSeLinuxOptions>,
    /// SeccompProfile defines a pod/container's seccomp profile settings.
    /// Only one profile source may be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "seccompProfile")]
    pub seccomp_profile: Option<SqlJobPodSecurityContextSeccompProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supplementalGroups")]
    pub supplemental_groups: Option<Vec<i64>>,
}

/// AppArmorProfile defines a pod or container's AppArmor settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPodSecurityContextAppArmorProfile {
    /// localhostProfile indicates a profile loaded on the node that should be used.
    /// The profile must be preconfigured on the node to work.
    /// Must match the loaded name of the profile.
    /// Must be set if and only if type is "Localhost".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of AppArmor profile will be applied.
    /// Valid options are:
    ///   Localhost - a profile pre-loaded on the node.
    ///   RuntimeDefault - the container runtime's default profile.
    ///   Unconfined - no AppArmor enforcement.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// SELinuxOptions are the labels to be applied to the container
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPodSecurityContextSeLinuxOptions {
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

/// SeccompProfile defines a pod/container's seccomp profile settings.
/// Only one profile source may be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobPodSecurityContextSeccompProfile {
    /// localhostProfile indicates a profile defined in a file on the node should be used.
    /// The profile must be preconfigured on the node to work.
    /// Must be a descending path, relative to the kubelet's configured seccomp profile location.
    /// Must be set if type is "Localhost". Must NOT be set for any other type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localhostProfile")]
    pub localhost_profile: Option<String>,
    /// type indicates which kind of seccomp profile will be applied.
    /// Valid options are:
    /// 
    /// Localhost - a profile defined in a file on the node should be used.
    /// RuntimeDefault - the container runtime default profile should be used.
    /// Unconfined - no profile should be applied.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Resouces describes the compute resource requirements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobResources {
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// ResourceList is a set of (resource name, quantity) pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// SqlJobSpec defines the desired state of SqlJob
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SqlJobRestartPolicy {
    Always,
    OnFailure,
    Never,
}

/// Schedule defines when the SqlJob will be executed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobSchedule {
    /// Cron is a cron expression that defines the schedule.
    pub cron: String,
    /// Suspend defines whether the schedule is active or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

/// SecurityContext holds security configuration that will be applied to a container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobSecurityContext {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowPrivilegeEscalation")]
    pub allow_privilege_escalation: Option<bool>,
    /// Adds and removes POSIX capabilities from running containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<SqlJobSecurityContextCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsGroup")]
    pub run_as_group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runAsUser")]
    pub run_as_user: Option<i64>,
}

/// Adds and removes POSIX capabilities from running containers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobSecurityContextCapabilities {
    /// Added capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// Removed capabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// SqlConfigMapKeyRef is a reference to a ConfigMap containing the Sql script.
/// It is defaulted to a ConfigMap with the contents of the Sql field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobSqlConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// SqlJobStatus defines the observed state of SqlJob
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SqlJobStatus {
    /// Conditions for the SqlJob object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

