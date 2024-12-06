// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1beta1/machinedeployments.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// MachineDeploymentSpec defines the desired state of MachineDeployment.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1beta1", kind = "MachineDeployment", plural = "machinedeployments")]
#[kube(namespaced)]
#[kube(status = "MachineDeploymentStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachineDeploymentSpec {
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// minReadySeconds is the minimum number of seconds for which a Node for a newly created machine should be ready before considering the replica available.
    /// Defaults to 0 (machine will be considered available as soon as the Node is ready)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    /// Indicates that the deployment is paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// The maximum time in seconds for a deployment to make progress before it
    /// is considered to be failed. The deployment controller will continue to
    /// process failed deployments and a condition with a ProgressDeadlineExceeded
    /// reason will be surfaced in the deployment status. Note that progress will
    /// not be estimated during the time a deployment is paused. Defaults to 600s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "progressDeadlineSeconds")]
    pub progress_deadline_seconds: Option<i32>,
    /// Number of desired machines.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// 
    /// Defaults to:
    /// * if the Kubernetes autoscaler min size and max size annotations are set:
    ///   - if it's a new MachineDeployment, use min size
    ///   - if the replicas field of the old MachineDeployment is < min size, use min size
    ///   - if the replicas field of the old MachineDeployment is > max size, use max size
    ///   - if the replicas field of the old MachineDeployment is in the (min size, max size) range, keep the value from the oldMD
    /// * otherwise use 1
    /// Note: Defaulting will be run whenever the replicas field is not set:
    /// * A new MachineDeployment is created with replicas not set.
    /// * On an existing MachineDeployment the replicas field was first set and is now unset.
    /// Those cases are especially relevant for the following Kubernetes autoscaler use cases:
    /// * A new MachineDeployment is created and replicas should be managed by the autoscaler
    /// * An existing MachineDeployment which initially wasn't controlled by the autoscaler
    ///   should be later controlled by the autoscaler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The number of old MachineSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 1.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/issues/10479 for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    /// rolloutAfter is a field to indicate a rollout should be performed
    /// after the specified time even if no changes have been made to the
    /// MachineDeployment.
    /// Example: In the YAML the time can be specified in the RFC3339 format.
    /// To specify the rolloutAfter target as March 9, 2023, at 9 am UTC
    /// use "2023-03-09T09:00:00Z".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutAfter")]
    pub rollout_after: Option<String>,
    /// Label selector for machines. Existing MachineSets whose machines are
    /// selected by this will be the ones affected by this deployment.
    /// It must match the machine template's labels.
    pub selector: MachineDeploymentSelector,
    /// The deployment strategy to use to replace existing machines with
    /// new ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<MachineDeploymentStrategy>,
    /// template describes the machines that will be created.
    pub template: MachineDeploymentTemplate,
}

/// Label selector for machines. Existing MachineSets whose machines are
/// selected by this will be the ones affected by this deployment.
/// It must match the machine template's labels.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineDeploymentSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The deployment strategy to use to replace existing machines with
/// new ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStrategy {
    /// remediation controls the strategy of remediating unhealthy machines
    /// and how remediating operations should occur during the lifecycle of the dependant MachineSets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<MachineDeploymentStrategyRemediation>,
    /// Rolling update config params. Present only if
    /// MachineDeploymentStrategyType = RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<MachineDeploymentStrategyRollingUpdate>,
    /// type of deployment. Allowed values are RollingUpdate and OnDelete.
    /// The default is RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<MachineDeploymentStrategyType>,
}

/// remediation controls the strategy of remediating unhealthy machines
/// and how remediating operations should occur during the lifecycle of the dependant MachineSets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStrategyRemediation {
    /// maxInFlight determines how many in flight remediations should happen at the same time.
    /// 
    /// Remediation only happens on the MachineSet with the most current revision, while
    /// older MachineSets (usually present during rollout operations) aren't allowed to remediate.
    /// 
    /// Note: In general (independent of remediations), unhealthy machines are always
    /// prioritized during scale down operations over healthy ones.
    /// 
    /// MaxInFlight can be set to a fixed number or a percentage.
    /// Example: when this is set to 20%, the MachineSet controller deletes at most 20% of
    /// the desired replicas.
    /// 
    /// If not set, remediation is limited to all machines (bounded by replicas)
    /// under the active MachineSet's management.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxInFlight")]
    pub max_in_flight: Option<IntOrString>,
}

/// Rolling update config params. Present only if
/// MachineDeploymentStrategyType = RollingUpdate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStrategyRollingUpdate {
    /// deletePolicy defines the policy used by the MachineDeployment to identify nodes to delete when downscaling.
    /// Valid values are "Random, "Newest", "Oldest"
    /// When no value is supplied, the default DeletePolicy of MachineSet is used
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletePolicy")]
    pub delete_policy: Option<MachineDeploymentStrategyRollingUpdateDeletePolicy>,
    /// The maximum number of machines that can be scheduled above the
    /// desired number of machines.
    /// Value can be an absolute number (ex: 5) or a percentage of
    /// desired machines (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// Defaults to 1.
    /// Example: when this is set to 30%, the new MachineSet can be scaled
    /// up immediately when the rolling update starts, such that the total
    /// number of old and new machines do not exceed 130% of desired
    /// machines. Once old machines have been killed, new MachineSet can
    /// be scaled up further, ensuring that total number of machines running
    /// at any time during the update is at most 130% of desired machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSurge")]
    pub max_surge: Option<IntOrString>,
    /// The maximum number of machines that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired
    /// machines (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// Defaults to 0.
    /// Example: when this is set to 30%, the old MachineSet can be scaled
    /// down to 70% of desired machines immediately when the rolling update
    /// starts. Once new machines are ready, old MachineSet can be scaled
    /// down further, followed by scaling up the new MachineSet, ensuring
    /// that the total number of machines available at all times
    /// during the update is at least 70% of desired machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
}

/// Rolling update config params. Present only if
/// MachineDeploymentStrategyType = RollingUpdate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineDeploymentStrategyRollingUpdateDeletePolicy {
    Random,
    Newest,
    Oldest,
}

/// The deployment strategy to use to replace existing machines with
/// new ones.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineDeploymentStrategyType {
    RollingUpdate,
    OnDelete,
}

/// template describes the machines that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MachineDeploymentTemplateMetadata>,
    /// Specification of the desired behavior of the machine.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MachineDeploymentTemplateSpec>,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateMetadata {
    /// annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Specification of the desired behavior of the machine.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpec {
    /// bootstrap is a reference to a local struct which encapsulates
    /// fields to configure the Machine’s bootstrapping mechanism.
    pub bootstrap: MachineDeploymentTemplateSpecBootstrap,
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// failureDomain is the failure domain the machine will be created in.
    /// Must match a key in the FailureDomains map stored on the cluster object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// infrastructureRef is a required reference to a custom resource
    /// offered by an infrastructure provider.
    #[serde(rename = "infrastructureRef")]
    pub infrastructure_ref: ObjectReference,
    /// nodeDeletionTimeout defines how long the controller will attempt to delete the Node that the Machine
    /// hosts after the Machine is marked for deletion. A duration of 0 will retry deletion indefinitely.
    /// Defaults to 10 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDeletionTimeout")]
    pub node_deletion_timeout: Option<String>,
    /// nodeDrainTimeout is the total amount of time that the controller will spend on draining a node.
    /// The default value is 0, meaning that the node can be drained without any time limitations.
    /// NOTE: NodeDrainTimeout is different from `kubectl drain --timeout`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDrainTimeout")]
    pub node_drain_timeout: Option<String>,
    /// nodeVolumeDetachTimeout is the total amount of time that the controller will spend on waiting for all volumes
    /// to be detached. The default value is 0, meaning that the volumes can be detached without any time limitations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeVolumeDetachTimeout")]
    pub node_volume_detach_timeout: Option<String>,
    /// providerID is the identification ID of the machine provided by the provider.
    /// This field must match the provider ID as seen on the node object corresponding to this machine.
    /// This field is required by higher level consumers of cluster-api. Example use case is cluster autoscaler
    /// with cluster-api as provider. Clean-up logic in the autoscaler compares machines to nodes to find out
    /// machines at provider which could not get registered as Kubernetes nodes. With cluster-api as a
    /// generic out-of-tree provider for autoscaler, this field is required by autoscaler to be
    /// able to have a provider view of the list of machines. Another list of nodes is queried from the k8s apiserver
    /// and then a comparison is done to find out unregistered machines and are marked for delete.
    /// This field will be set by the actuators and consumed by higher level entities like autoscaler that will
    /// be interfacing with cluster-api as generic provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// readinessGates specifies additional conditions to include when evaluating Machine Ready condition.
    /// 
    /// This field can be used e.g. by Cluster API control plane providers to extend the semantic of the
    /// Ready condition for the Machine they control, like the kubeadm control provider adding ReadinessGates
    /// for the APIServerPodHealthy, SchedulerPodHealthy conditions, etc.
    /// 
    /// Another example are external controllers, e.g. responsible to install special software/hardware on the Machines;
    /// they can include the status of those components with a new condition and add this condition to ReadinessGates.
    /// 
    /// NOTE: this field is considered only for computing v1beta2 conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readinessGates")]
    pub readiness_gates: Option<Vec<MachineDeploymentTemplateSpecReadinessGates>>,
    /// version defines the desired Kubernetes version.
    /// This field is meant to be optionally used by bootstrap providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// bootstrap is a reference to a local struct which encapsulates
/// fields to configure the Machine’s bootstrapping mechanism.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecBootstrap {
    /// configRef is a reference to a bootstrap provider-specific resource
    /// that holds configuration details. The reference is optional to
    /// allow users/operators to specify Bootstrap.DataSecretName without
    /// the need of a controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<ObjectReference>,
    /// dataSecretName is the name of the secret that stores the bootstrap data script.
    /// If nil, the Machine should remain in the Pending state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSecretName")]
    pub data_secret_name: Option<String>,
}

/// configRef is a reference to a bootstrap provider-specific resource
/// that holds configuration details. The reference is optional to
/// allow users/operators to specify Bootstrap.DataSecretName without
/// the need of a controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecBootstrapConfigRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// infrastructureRef is a required reference to a custom resource
/// offered by an infrastructure provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// MachineReadinessGate contains the type of a Machine condition to be used as a readiness gate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecReadinessGates {
    /// conditionType refers to a positive polarity condition (status true means good) with matching type in the Machine's condition list.
    /// If the conditions doesn't exist, it will be treated as unknown.
    /// Note: Both Cluster API conditions or conditions added by 3rd party controllers can be used as readiness gates.
    #[serde(rename = "conditionType")]
    pub condition_type: String,
}

/// MachineDeploymentStatus defines the observed state of MachineDeployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStatus {
    /// Total number of available machines (ready for at least minReadySeconds)
    /// targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions defines current service state of the MachineDeployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation observed by the deployment controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// phase represents the current phase of a MachineDeployment (ScalingUp, ScalingDown, Running, Failed, or Unknown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Total number of ready machines targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment
    /// (their labels match the selector).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is the same as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Total number of unavailable machines targeted by this deployment.
    /// This is the total number of machines that are still required for
    /// the deployment to have 100% available capacity. They may either
    /// be machines that are running but not yet available or machines
    /// that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment
    /// that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReplicas")]
    pub updated_replicas: Option<i32>,
    /// v1beta2 groups all the fields that will be added or modified in MachineDeployment's status with the V1Beta2 version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v1beta2: Option<MachineDeploymentStatusV1beta2>,
}

/// v1beta2 groups all the fields that will be added or modified in MachineDeployment's status with the V1Beta2 version.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStatusV1beta2 {
    /// availableReplicas is the number of available replicas for this MachineDeployment. A machine is considered available when Machine's Available condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions represents the observations of a MachineDeployment's current state.
    /// Known condition types are Available, MachinesReady, MachinesUpToDate, ScalingUp, ScalingDown, Remediating, Deleting, Paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// readyReplicas is the number of ready replicas for this MachineDeployment. A machine is considered ready when Machine's Ready condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// upToDateReplicas is the number of up-to-date replicas targeted by this deployment. A machine is considered up-to-date when Machine's UpToDate condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upToDateReplicas")]
    pub up_to_date_replicas: Option<i32>,
}

