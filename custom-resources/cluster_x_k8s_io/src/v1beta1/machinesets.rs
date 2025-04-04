// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1beta1/machinesets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// spec is the desired state of MachineSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1beta1", kind = "MachineSet", plural = "machinesets")]
#[kube(namespaced)]
#[kube(status = "MachineSetStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachineSetSpec {
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// deletePolicy defines the policy used to identify nodes to delete when downscaling.
    /// Defaults to "Random".  Valid values are "Random, "Newest", "Oldest"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletePolicy")]
    pub delete_policy: Option<MachineSetDeletePolicy>,
    /// machineNamingStrategy allows changing the naming pattern used when creating Machines.
    /// Note: InfraMachines & BootstrapConfigs will use the same name as the corresponding Machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineNamingStrategy")]
    pub machine_naming_strategy: Option<MachineSetMachineNamingStrategy>,
    /// minReadySeconds is the minimum number of seconds for which a Node for a newly created machine should be ready before considering the replica available.
    /// Defaults to 0 (machine will be considered available as soon as the Node is ready)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    /// replicas is the number of desired replicas.
    /// This is a pointer to distinguish between explicit zero and unspecified.
    /// 
    /// Defaults to:
    /// * if the Kubernetes autoscaler min size and max size annotations are set:
    ///   - if it's a new MachineSet, use min size
    ///   - if the replicas field of the old MachineSet is < min size, use min size
    ///   - if the replicas field of the old MachineSet is > max size, use max size
    ///   - if the replicas field of the old MachineSet is in the (min size, max size) range, keep the value from the oldMS
    /// * otherwise use 1
    /// Note: Defaulting will be run whenever the replicas field is not set:
    /// * A new MachineSet is created with replicas not set.
    /// * On an existing MachineSet the replicas field was first set and is now unset.
    /// Those cases are especially relevant for the following Kubernetes autoscaler use cases:
    /// * A new MachineSet is created and replicas should be managed by the autoscaler
    /// * An existing MachineSet which initially wasn't controlled by the autoscaler
    ///   should be later controlled by the autoscaler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is a label query over machines that should match the replica count.
    /// Label keys and values that must match in order to be controlled by this MachineSet.
    /// It must match the machine template's labels.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    pub selector: MachineSetSelector,
    /// template is the object that describes the machine that will be created if
    /// insufficient replicas are detected.
    /// Object references to custom resources are treated as templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<MachineSetTemplate>,
}

/// spec is the desired state of MachineSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineSetDeletePolicy {
    Random,
    Newest,
    Oldest,
}

/// machineNamingStrategy allows changing the naming pattern used when creating Machines.
/// Note: InfraMachines & BootstrapConfigs will use the same name as the corresponding Machines.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetMachineNamingStrategy {
    /// template defines the template to use for generating the names of the
    /// Machine objects.
    /// If not defined, it will fallback to `{{ .machineSet.name }}-{{ .random }}`.
    /// If the generated name string exceeds 63 characters, it will be trimmed to
    /// 58 characters and will
    /// get concatenated with a random suffix of length 5.
    /// Length of the template string must not exceed 256 characters.
    /// The template allows the following variables `.cluster.name`,
    /// `.machineSet.name` and `.random`.
    /// The variable `.cluster.name` retrieves the name of the cluster object
    /// that owns the Machines being created.
    /// The variable `.machineSet.name` retrieves the name of the MachineSet
    /// object that owns the Machines being created.
    /// The variable `.random` is substituted with random alphanumeric string,
    /// without vowels, of length 5. This variable is required part of the
    /// template. If not provided, validation will fail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

/// selector is a label query over machines that should match the replica count.
/// Label keys and values that must match in order to be controlled by this MachineSet.
/// It must match the machine template's labels.
/// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineSetSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetSelectorMatchExpressions {
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

/// template is the object that describes the machine that will be created if
/// insufficient replicas are detected.
/// Object references to custom resources are treated as templates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetTemplate {
    /// metadata is the standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MachineSetTemplateMetadata>,
    /// spec is the specification of the desired behavior of the machine.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MachineSetTemplateSpec>,
}

/// metadata is the standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetTemplateMetadata {
    /// annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// labels is a map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// spec is the specification of the desired behavior of the machine.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetTemplateSpec {
    /// bootstrap is a reference to a local struct which encapsulates
    /// fields to configure the Machine’s bootstrapping mechanism.
    pub bootstrap: MachineSetTemplateSpecBootstrap,
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
    /// NOTE: This field is considered only for computing v1beta2 conditions.
    /// NOTE: In case readinessGates conditions start with the APIServer, ControllerManager, Scheduler prefix, and all those
    /// readiness gates condition are reporting the same message, when computing the Machine's Ready condition those
    /// readinessGates will be replaced by a single entry reporting "Control plane components: " + message.
    /// This helps to improve readability of conditions bubbling up to the Machine's owner resource / to the Cluster).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readinessGates")]
    pub readiness_gates: Option<Vec<MachineSetTemplateSpecReadinessGates>>,
    /// version defines the desired Kubernetes version.
    /// This field is meant to be optionally used by bootstrap providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// bootstrap is a reference to a local struct which encapsulates
/// fields to configure the Machine’s bootstrapping mechanism.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetTemplateSpecBootstrap {
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
pub struct MachineSetTemplateSpecBootstrapConfigRef {
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
pub struct MachineSetTemplateSpecInfrastructureRef {
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
pub struct MachineSetTemplateSpecReadinessGates {
    /// conditionType refers to a condition with matching type in the Machine's condition list.
    /// If the conditions doesn't exist, it will be treated as unknown.
    /// Note: Both Cluster API conditions or conditions added by 3rd party controllers can be used as readiness gates.
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    /// polarity of the conditionType specified in this readinessGate.
    /// Valid values are Positive, Negative and omitted.
    /// When omitted, the default behaviour will be Positive.
    /// A positive polarity means that the condition should report a true status under normal conditions.
    /// A negative polarity means that the condition should report a false status under normal conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polarity: Option<MachineSetTemplateSpecReadinessGatesPolarity>,
}

/// MachineReadinessGate contains the type of a Machine condition to be used as a readiness gate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineSetTemplateSpecReadinessGatesPolarity {
    Positive,
    Negative,
}

/// status is the observed state of MachineSet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetStatus {
    /// availableReplicas is the number of available replicas (ready for at least minReadySeconds) for this MachineSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions defines current service state of the MachineSet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// failureMessage will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a more verbose string suitable
    /// for logging and human consumption.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// failureReason will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a succinct value suitable
    /// for machine interpretation.
    /// 
    /// In the event that there is a terminal problem reconciling the
    /// replicas, both FailureReason and FailureMessage will be set. FailureReason
    /// will be populated with a succinct value suitable for machine
    /// interpretation, while FailureMessage will contain a more verbose
    /// string suitable for logging and human consumption.
    /// 
    /// These fields should not be set for transitive errors that a
    /// controller faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the MachineTemplate's spec or the configuration of
    /// the machine controller, and that manual intervention is required. Examples
    /// of terminal errors would be invalid combinations of settings in the
    /// spec, values that are unsupported by the machine controller, or the
    /// responsible machine controller itself being critically misconfigured.
    /// 
    /// Any transient errors that occur during the reconciliation of Machines
    /// can be added as events to the MachineSet object and/or logged in the
    /// controller's output.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// fullyLabeledReplicas is the number of replicas that have labels matching the labels of the machine template of the MachineSet.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullyLabeledReplicas")]
    pub fully_labeled_replicas: Option<i32>,
    /// observedGeneration reflects the generation of the most recently observed MachineSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of ready replicas for this MachineSet. A machine is considered ready when the node has been created and is "Ready".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// replicas is the most recently observed number of replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is the same as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// v1beta2 groups all the fields that will be added or modified in MachineSet's status with the V1Beta2 version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v1beta2: Option<MachineSetStatusV1beta2>,
}

/// v1beta2 groups all the fields that will be added or modified in MachineSet's status with the V1Beta2 version.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineSetStatusV1beta2 {
    /// availableReplicas is the number of available replicas for this MachineSet. A machine is considered available when Machine's Available condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions represents the observations of a MachineSet's current state.
    /// Known condition types are MachinesReady, MachinesUpToDate, ScalingUp, ScalingDown, Remediating, Deleting, Paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// readyReplicas is the number of ready replicas for this MachineSet. A machine is considered ready when Machine's Ready condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// upToDateReplicas is the number of up-to-date replicas for this MachineSet. A machine is considered up-to-date when Machine's UpToDate condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upToDateReplicas")]
    pub up_to_date_replicas: Option<i32>,
}

