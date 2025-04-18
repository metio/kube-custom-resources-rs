// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/elastic/cloud-on-k8s/beat.k8s.elastic.co/v1beta1/beats.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// BeatSpec defines the desired state of a Beat.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "beat.k8s.elastic.co", version = "v1beta1", kind = "Beat", plural = "beats")]
#[kube(namespaced)]
#[kube(status = "BeatStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BeatSpec {
    /// Config holds the Beat configuration. At most one of [`Config`, `ConfigRef`] can be specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    /// ConfigRef contains a reference to an existing Kubernetes Secret holding the Beat configuration.
    /// Beat settings must be specified as yaml, under a single "beat.yml" entry. At most one of [`Config`, `ConfigRef`]
    /// can be specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<BeatConfigRef>,
    /// DaemonSet specifies the Beat should be deployed as a DaemonSet, and allows providing its spec.
    /// Cannot be used along with `deployment`. If both are absent a default for the Type is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "daemonSet")]
    pub daemon_set: Option<BeatDaemonSet>,
    /// Deployment specifies the Beat should be deployed as a Deployment, and allows providing its spec.
    /// Cannot be used along with `daemonSet`. If both are absent a default for the Type is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<BeatDeployment>,
    /// ElasticsearchRef is a reference to an Elasticsearch cluster running in the same Kubernetes cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "elasticsearchRef")]
    pub elasticsearch_ref: Option<BeatElasticsearchRef>,
    /// Image is the Beat Docker image to deploy. Version and Type have to match the Beat in the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// KibanaRef is a reference to a Kibana instance running in the same Kubernetes cluster.
    /// It allows automatic setup of dashboards and visualizations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kibanaRef")]
    pub kibana_ref: Option<BeatKibanaRef>,
    /// Monitoring enables you to collect and ship logs and metrics for this Beat.
    /// Metricbeat and/or Filebeat sidecars are configured and send monitoring data to an
    /// Elasticsearch monitoring cluster running in the same Kubernetes cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<BeatMonitoring>,
    /// RevisionHistoryLimit is the number of revisions to retain to allow rollback in the underlying DaemonSet or Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    /// SecureSettings is a list of references to Kubernetes Secrets containing sensitive configuration options for the Beat.
    /// Secrets data can be then referenced in the Beat config using the Secret's keys or as specified in `Entries` field of
    /// each SecureSetting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secureSettings")]
    pub secure_settings: Option<Vec<BeatSecureSettings>>,
    /// ServiceAccountName is used to check access from the current resource to Elasticsearch resource in a different namespace.
    /// Can only be used if ECK is enforcing RBAC on references.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// Type is the type of the Beat to deploy (filebeat, metricbeat, heartbeat, auditbeat, journalbeat, packetbeat, and so on).
    /// Any string can be used, but well-known types will have the image field defaulted and have the appropriate
    /// Elasticsearch roles created automatically. It also allows for dashboard setup when combined with a `KibanaRef`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Version of the Beat.
    pub version: String,
}

/// ConfigRef contains a reference to an existing Kubernetes Secret holding the Beat configuration.
/// Beat settings must be specified as yaml, under a single "beat.yml" entry. At most one of [`Config`, `ConfigRef`]
/// can be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatConfigRef {
    /// SecretName is the name of the secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// DaemonSet specifies the Beat should be deployed as a DaemonSet, and allows providing its spec.
/// Cannot be used along with `deployment`. If both are absent a default for the Type is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDaemonSet {
    /// PodTemplateSpec describes the data a pod should have when created from a template
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podTemplate")]
    pub pod_template: Option<BTreeMap<String, serde_json::Value>>,
    /// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<BeatDaemonSetUpdateStrategy>,
}

/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDaemonSetUpdateStrategy {
    /// Rolling update config params. Present only if type = "RollingUpdate".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<BeatDaemonSetUpdateStrategyRollingUpdate>,
    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Rolling update config params. Present only if type = "RollingUpdate".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDaemonSetUpdateStrategyRollingUpdate {
    /// The maximum number of nodes with an existing available DaemonSet pod that
    /// can have an updated DaemonSet pod during during an update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up to a minimum of 1.
    /// Default value is 0.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their a new pod created before the old pod is marked as deleted.
    /// The update starts by launching new pods on 30% of nodes. Once an updated
    /// pod is available (Ready for at least minReadySeconds) the old DaemonSet pod
    /// on that node is marked deleted. If the old pod becomes unavailable for any
    /// reason (Ready transitions to false, is evicted, or is drained) an updated
    /// pod is immediatedly created on that node without considering surge limits.
    /// Allowing surge implies the possibility that the resources consumed by the
    /// daemonset on any given node can double if the readiness check fails, and
    /// so resource intensive daemonsets should take into account that they may
    /// cause evictions during disruption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSurge")]
    pub max_surge: Option<IntOrString>,
    /// The maximum number of DaemonSet pods that can be unavailable during the
    /// update. Value can be an absolute number (ex: 5) or a percentage of total
    /// number of DaemonSet pods at the start of the update (ex: 10%). Absolute
    /// number is calculated from percentage by rounding up.
    /// This cannot be 0 if MaxSurge is 0
    /// Default value is 1.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their pods stopped for an update at any given time. The update
    /// starts by stopping at most 30% of those DaemonSet pods and then brings
    /// up new DaemonSet pods in their place. Once the new pods are available,
    /// it then proceeds onto other DaemonSet pods, thus ensuring that at least
    /// 70% of original number of DaemonSet pods are available at all times during
    /// the update.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
}

/// Deployment specifies the Beat should be deployed as a Deployment, and allows providing its spec.
/// Cannot be used along with `daemonSet`. If both are absent a default for the Type is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDeployment {
    /// PodTemplateSpec describes the data a pod should have when created from a template
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podTemplate")]
    pub pod_template: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// DeploymentStrategy describes how to replace existing pods with new ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<BeatDeploymentStrategy>,
}

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDeploymentStrategy {
    /// Rolling update config params. Present only if DeploymentStrategyType =
    /// RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<BeatDeploymentStrategyRollingUpdate>,
    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Rolling update config params. Present only if DeploymentStrategyType =
/// RollingUpdate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatDeploymentStrategyRollingUpdate {
    /// The maximum number of pods that can be scheduled above the desired number of
    /// pods.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when
    /// the rolling update starts, such that the total number of old and new pods do not exceed
    /// 130% of desired pods. Once old pods have been killed,
    /// new ReplicaSet can be scaled up further, ensuring that total number of pods running
    /// at any time during the update is at most 130% of desired pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSurge")]
    pub max_surge: Option<IntOrString>,
    /// The maximum number of pods that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods
    /// immediately when the rolling update starts. Once new pods are ready, old ReplicaSet
    /// can be scaled down further, followed by scaling up the new ReplicaSet, ensuring
    /// that the total number of pods available at all times during the update is at
    /// least 70% of desired pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
}

/// ElasticsearchRef is a reference to an Elasticsearch cluster running in the same Kubernetes cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatElasticsearchRef {
    /// Name of an existing Kubernetes object corresponding to an Elastic resource managed by ECK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the Kubernetes object. If empty, defaults to the current namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// SecretName is the name of an existing Kubernetes secret that contains connection information for associating an
    /// Elastic resource not managed by the operator. The referenced secret must contain the following:
    /// - `url`: the URL to reach the Elastic resource
    /// - `username`: the username of the user to be authenticated to the Elastic resource
    /// - `password`: the password of the user to be authenticated to the Elastic resource
    /// - `ca.crt`: the CA certificate in PEM format (optional)
    /// - `api-key`: the key to authenticate against the Elastic resource instead of a username and password (supported only for `elasticsearchRefs` in AgentSpec and in BeatSpec)
    /// This field cannot be used in combination with the other fields name, namespace or serviceName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// ServiceName is the name of an existing Kubernetes service which is used to make requests to the referenced
    /// object. It has to be in the same namespace as the referenced resource. If left empty, the default HTTP service of
    /// the referenced resource is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

/// KibanaRef is a reference to a Kibana instance running in the same Kubernetes cluster.
/// It allows automatic setup of dashboards and visualizations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatKibanaRef {
    /// Name of an existing Kubernetes object corresponding to an Elastic resource managed by ECK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the Kubernetes object. If empty, defaults to the current namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// SecretName is the name of an existing Kubernetes secret that contains connection information for associating an
    /// Elastic resource not managed by the operator. The referenced secret must contain the following:
    /// - `url`: the URL to reach the Elastic resource
    /// - `username`: the username of the user to be authenticated to the Elastic resource
    /// - `password`: the password of the user to be authenticated to the Elastic resource
    /// - `ca.crt`: the CA certificate in PEM format (optional)
    /// - `api-key`: the key to authenticate against the Elastic resource instead of a username and password (supported only for `elasticsearchRefs` in AgentSpec and in BeatSpec)
    /// This field cannot be used in combination with the other fields name, namespace or serviceName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// ServiceName is the name of an existing Kubernetes service which is used to make requests to the referenced
    /// object. It has to be in the same namespace as the referenced resource. If left empty, the default HTTP service of
    /// the referenced resource is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

/// Monitoring enables you to collect and ship logs and metrics for this Beat.
/// Metricbeat and/or Filebeat sidecars are configured and send monitoring data to an
/// Elasticsearch monitoring cluster running in the same Kubernetes cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatMonitoring {
    /// Logs holds references to Elasticsearch clusters which receive log data from an associated resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<BeatMonitoringLogs>,
    /// Metrics holds references to Elasticsearch clusters which receive monitoring data from this resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BeatMonitoringMetrics>,
}

/// Logs holds references to Elasticsearch clusters which receive log data from an associated resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatMonitoringLogs {
    /// ElasticsearchRefs is a reference to a list of monitoring Elasticsearch clusters running in the same Kubernetes cluster.
    /// Due to existing limitations, only a single Elasticsearch cluster is currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "elasticsearchRefs")]
    pub elasticsearch_refs: Option<Vec<BeatMonitoringLogsElasticsearchRefs>>,
}

/// ObjectSelector defines a reference to a Kubernetes object which can be an Elastic resource managed by the operator
/// or a Secret describing an external Elastic resource not managed by the operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatMonitoringLogsElasticsearchRefs {
    /// Name of an existing Kubernetes object corresponding to an Elastic resource managed by ECK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the Kubernetes object. If empty, defaults to the current namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// SecretName is the name of an existing Kubernetes secret that contains connection information for associating an
    /// Elastic resource not managed by the operator. The referenced secret must contain the following:
    /// - `url`: the URL to reach the Elastic resource
    /// - `username`: the username of the user to be authenticated to the Elastic resource
    /// - `password`: the password of the user to be authenticated to the Elastic resource
    /// - `ca.crt`: the CA certificate in PEM format (optional)
    /// - `api-key`: the key to authenticate against the Elastic resource instead of a username and password (supported only for `elasticsearchRefs` in AgentSpec and in BeatSpec)
    /// This field cannot be used in combination with the other fields name, namespace or serviceName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// ServiceName is the name of an existing Kubernetes service which is used to make requests to the referenced
    /// object. It has to be in the same namespace as the referenced resource. If left empty, the default HTTP service of
    /// the referenced resource is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

/// Metrics holds references to Elasticsearch clusters which receive monitoring data from this resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatMonitoringMetrics {
    /// ElasticsearchRefs is a reference to a list of monitoring Elasticsearch clusters running in the same Kubernetes cluster.
    /// Due to existing limitations, only a single Elasticsearch cluster is currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "elasticsearchRefs")]
    pub elasticsearch_refs: Option<Vec<BeatMonitoringMetricsElasticsearchRefs>>,
}

/// ObjectSelector defines a reference to a Kubernetes object which can be an Elastic resource managed by the operator
/// or a Secret describing an external Elastic resource not managed by the operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatMonitoringMetricsElasticsearchRefs {
    /// Name of an existing Kubernetes object corresponding to an Elastic resource managed by ECK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the Kubernetes object. If empty, defaults to the current namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// SecretName is the name of an existing Kubernetes secret that contains connection information for associating an
    /// Elastic resource not managed by the operator. The referenced secret must contain the following:
    /// - `url`: the URL to reach the Elastic resource
    /// - `username`: the username of the user to be authenticated to the Elastic resource
    /// - `password`: the password of the user to be authenticated to the Elastic resource
    /// - `ca.crt`: the CA certificate in PEM format (optional)
    /// - `api-key`: the key to authenticate against the Elastic resource instead of a username and password (supported only for `elasticsearchRefs` in AgentSpec and in BeatSpec)
    /// This field cannot be used in combination with the other fields name, namespace or serviceName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// ServiceName is the name of an existing Kubernetes service which is used to make requests to the referenced
    /// object. It has to be in the same namespace as the referenced resource. If left empty, the default HTTP service of
    /// the referenced resource is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
}

/// SecretSource defines a data source based on a Kubernetes Secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatSecureSettings {
    /// Entries define how to project each key-value pair in the secret to filesystem paths.
    /// If not defined, all keys will be projected to similarly named paths in the filesystem.
    /// If defined, only the specified keys will be projected to the corresponding paths.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<BeatSecureSettingsEntries>>,
    /// SecretName is the name of the secret.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// KeyToPath defines how to map a key in a Secret object to a filesystem path.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatSecureSettingsEntries {
    /// Key is the key contained in the secret.
    pub key: String,
    /// Path is the relative file path to map the key to.
    /// Path must not be an absolute file path and must not contain any ".." components.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// BeatStatus defines the observed state of a Beat.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BeatStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableNodes")]
    pub available_nodes: Option<i32>,
    /// AssociationStatus is the status of an association resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "elasticsearchAssociationStatus")]
    pub elasticsearch_association_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedNodes")]
    pub expected_nodes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// AssociationStatus is the status of an association resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kibanaAssociationStatus")]
    pub kibana_association_status: Option<String>,
    /// AssociationStatusMap is the map of association's namespaced name string to its AssociationStatus. For resources that
    /// have a single Association of a given type (for ex. single ES reference), this map contains a single entry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringAssociationStatus")]
    pub monitoring_association_status: Option<BTreeMap<String, String>>,
    /// ObservedGeneration represents the .metadata.generation that the status is based upon.
    /// It corresponds to the metadata generation, which is updated on mutation by the API Server.
    /// If the generation observed in status diverges from the generation in metadata, the Beats
    /// controller has not yet processed the changes contained in the Beats specification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Version of the stack resource currently running. During version upgrades, multiple versions may run
    /// in parallel: this value specifies the lowest version currently running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

