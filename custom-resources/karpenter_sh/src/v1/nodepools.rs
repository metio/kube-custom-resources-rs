// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/karpenter-provider-aws/karpenter.sh/v1/nodepools.yaml
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

/// NodePoolSpec is the top level nodepool specification. Nodepools
/// launch nodes in response to pods that are unschedulable. A single nodepool
/// is capable of managing a diverse set of nodes. Node properties are determined
/// from a combination of nodepool and pod scheduling constraints.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "karpenter.sh", version = "v1", kind = "NodePool", plural = "nodepools")]
#[kube(status = "NodePoolStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodePoolSpec {
    /// Disruption contains the parameters that relate to Karpenter's disruption logic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disruption: Option<NodePoolDisruption>,
    /// Limits define a set of bounds for provisioning capacity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Template contains the template of possibilities for the provisioning logic to launch a NodeClaim with.
    /// NodeClaims launched from this NodePool will often be further constrained than the template specifies.
    pub template: NodePoolTemplate,
    /// Weight is the priority given to the nodepool during scheduling. A higher
    /// numerical weight indicates that this nodepool will be ordered
    /// ahead of other nodepools with lower weights. A nodepool with no weight
    /// will be treated as if it is a nodepool with a weight of 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

/// Disruption contains the parameters that relate to Karpenter's disruption logic
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolDisruption {
    /// Budgets is a list of Budgets.
    /// If there are multiple active budgets, Karpenter uses
    /// the most restrictive value. If left undefined,
    /// this will default to one budget with a value to 10%.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<NodePoolDisruptionBudgets>>,
    /// ConsolidateAfter is the duration the controller will wait
    /// before attempting to terminate nodes that are underutilized.
    /// Refer to ConsolidationPolicy for how underutilization is considered.
    #[serde(rename = "consolidateAfter")]
    pub consolidate_after: String,
    /// ConsolidationPolicy describes which nodes Karpenter can disrupt through its consolidation
    /// algorithm. This policy defaults to "WhenEmptyOrUnderutilized" if not specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consolidationPolicy")]
    pub consolidation_policy: Option<NodePoolDisruptionConsolidationPolicy>,
}

/// Budget defines when Karpenter will restrict the
/// number of Node Claims that can be terminating simultaneously.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolDisruptionBudgets {
    /// Duration determines how long a Budget is active since each Schedule hit.
    /// Only minutes and hours are accepted, as cron does not work in seconds.
    /// If omitted, the budget is always active.
    /// This is required if Schedule is set.
    /// This regex has an optional 0s at the end since the duration.String() always adds
    /// a 0s at the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Nodes dictates the maximum number of NodeClaims owned by this NodePool
    /// that can be terminating at once. This is calculated by counting nodes that
    /// have a deletion timestamp set, or are actively being deleted by Karpenter.
    /// This field is required when specifying a budget.
    /// This cannot be of type intstr.IntOrString since kubebuilder doesn't support pattern
    /// checking for int nodes for IntOrString nodes.
    /// Ref: https://github.com/kubernetes-sigs/controller-tools/blob/55efe4be40394a288216dab63156b0a64fb82929/pkg/crd/markers/validation.go#L379-L388
    pub nodes: String,
    /// Reasons is a list of disruption methods that this budget applies to. If Reasons is not set, this budget applies to all methods.
    /// Otherwise, this will apply to each reason defined.
    /// allowed reasons are Underutilized, Empty, and Drifted and additional CloudProvider-specific reasons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    /// Schedule specifies when a budget begins being active, following
    /// the upstream cronjob syntax. If omitted, the budget is always active.
    /// Timezones are not supported.
    /// This field is required if Duration is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

/// Disruption contains the parameters that relate to Karpenter's disruption logic
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodePoolDisruptionConsolidationPolicy {
    WhenEmpty,
    WhenEmptyOrUnderutilized,
}

/// Template contains the template of possibilities for the provisioning logic to launch a NodeClaim with.
/// NodeClaims launched from this NodePool will often be further constrained than the template specifies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<NodePoolTemplateMetadata>,
    /// NodeClaimTemplateSpec describes the desired state of the NodeClaim in the Nodepool
    /// NodeClaimTemplateSpec is used in the NodePool's NodeClaimTemplate, with the resource requests omitted since
    /// users are not able to set resource requests in the NodePool.
    pub spec: NodePoolTemplateSpec,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolTemplateMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// NodeClaimTemplateSpec describes the desired state of the NodeClaim in the Nodepool
/// NodeClaimTemplateSpec is used in the NodePool's NodeClaimTemplate, with the resource requests omitted since
/// users are not able to set resource requests in the NodePool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolTemplateSpec {
    /// ExpireAfter is the duration the controller will wait
    /// before terminating a node, measured from when the node is created. This
    /// is useful to implement features like eventually consistent node upgrade,
    /// memory leak protection, and disruption testing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expireAfter")]
    pub expire_after: Option<String>,
    /// NodeClassRef is a reference to an object that defines provider specific configuration
    #[serde(rename = "nodeClassRef")]
    pub node_class_ref: NodePoolTemplateSpecNodeClassRef,
    /// Requirements are layered with GetLabels and applied to every node.
    pub requirements: Vec<NodePoolTemplateSpecRequirements>,
    /// StartupTaints are taints that are applied to nodes upon startup which are expected to be removed automatically
    /// within a short period of time, typically by a DaemonSet that tolerates the taint. These are commonly used by
    /// daemonsets to allow initialization and enforce startup ordering.  StartupTaints are ignored for provisioning
    /// purposes in that pods are not required to tolerate a StartupTaint in order to have nodes provisioned for them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startupTaints")]
    pub startup_taints: Option<Vec<NodePoolTemplateSpecStartupTaints>>,
    /// Taints will be applied to the NodeClaim's node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<NodePoolTemplateSpecTaints>>,
    /// TerminationGracePeriod is the maximum duration the controller will wait before forcefully deleting the pods on a node, measured from when deletion is first initiated.
    /// 
    /// Warning: this feature takes precedence over a Pod's terminationGracePeriodSeconds value, and bypasses any blocked PDBs or the karpenter.sh/do-not-disrupt annotation.
    /// 
    /// This field is intended to be used by cluster administrators to enforce that nodes can be cycled within a given time period.
    /// When set, drifted nodes will begin draining even if there are pods blocking eviction. Draining will respect PDBs and the do-not-disrupt annotation until the TGP is reached.
    /// 
    /// Karpenter will preemptively delete pods so their terminationGracePeriodSeconds align with the node's terminationGracePeriod.
    /// If a pod would be terminated without being granted its full terminationGracePeriodSeconds prior to the node timeout,
    /// that pod will be deleted at T = node timeout - pod terminationGracePeriodSeconds.
    /// 
    /// The feature can also be used to allow maximum time limits for long-running jobs which can delay node termination with preStop hooks.
    /// If left undefined, the controller will wait indefinitely for pods to be drained.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terminationGracePeriod")]
    pub termination_grace_period: Option<String>,
}

/// NodeClassRef is a reference to an object that defines provider specific configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolTemplateSpecNodeClassRef {
    /// API version of the referent
    pub group: String,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds"
    pub kind: String,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,
}

/// A node selector requirement with min values is a selector that contains values, a key, an operator that relates the key and values
/// and minValues that represent the requirement to have at least that many values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodePoolTemplateSpecRequirements {
    /// The label key that the selector applies to.
    pub key: String,
    /// This field is ALPHA and can be dropped or replaced at any time
    /// MinValues is the minimum number of unique values required to define the flexibility of the specific requirement.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minValues")]
    pub min_values: Option<i64>,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: NodePoolTemplateSpecRequirementsOperator,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement with min values is a selector that contains values, a key, an operator that relates the key and values
/// and minValues that represent the requirement to have at least that many values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodePoolTemplateSpecRequirementsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodePoolTemplateSpecStartupTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: NodePoolTemplateSpecStartupTaintsEffect,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodePoolTemplateSpecStartupTaintsEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodePoolTemplateSpecTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: NodePoolTemplateSpecTaintsEffect,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodePoolTemplateSpecTaintsEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

/// NodePoolStatus defines the observed state of NodePool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodePoolStatus {
    /// Conditions contains signals for health and readiness
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Resources is the list of resources that have been provisioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, IntOrString>>,
}

