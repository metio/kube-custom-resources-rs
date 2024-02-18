// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/DataDog/datadog-operator/datadoghq.com/v1alpha1/datadogslos.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "datadoghq.com", version = "v1alpha1", kind = "DatadogSLO", plural = "datadogslos")]
#[kube(namespaced)]
#[kube(status = "DatadogSLOStatus")]
#[kube(schema = "disabled")]
pub struct DatadogSLOSpec {
    /// ControllerOptions are the optional parameters in the DatadogSLO controller
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerOptions")]
    pub controller_options: Option<DatadogSLOControllerOptions>,
    /// Description is a user-defined description of the service level objective. Always included in service level objective responses (but may be null). Optional in create/update requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Groups is a list of (up to 100) monitor groups that narrow the scope of a monitor service level objective. Included in service level objective responses if it is not empty. Optional in create/update requests for monitor service level objectives, but may only be used when the length of the monitor_ids field is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// MonitorIDs is a list of monitor IDs that defines the scope of a monitor service level objective. Required if type is monitor.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorIDs")]
    pub monitor_i_ds: Option<Vec<i64>>,
    /// Name is the name of the service level objective.
    pub name: String,
    /// Query is the query for a metric-based SLO. Required if type is metric. Note that only the `sum by` aggregator is allowed, which sums all request counts. `Average`, `max`, nor `min` request aggregators are not supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<DatadogSLOQuery>,
    /// Tags is a list of tags to associate with your service level objective. This can help you categorize and filter service level objectives in the service level objectives page of the UI. Note: it's not currently possible to filter by these tags when querying via the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// TargetThreshold is the target threshold such that when the service level indicator is above this threshold over the given timeframe, the objective is being met.
    #[serde(rename = "targetThreshold")]
    pub target_threshold: IntOrString,
    /// The SLO time window options.
    pub timeframe: String,
    /// Type is the type of the service level objective.
    #[serde(rename = "type")]
    pub r#type: String,
    /// WarningThreshold is a optional warning threshold such that when the service level indicator is below this value for the given threshold, but above the target threshold, the objective appears in a "warning" state. This value must be greater than the target threshold.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "warningThreshold")]
    pub warning_threshold: Option<IntOrString>,
}

/// ControllerOptions are the optional parameters in the DatadogSLO controller
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatadogSLOControllerOptions {
    /// DisableRequiredTags disables the automatic addition of required tags to SLOs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableRequiredTags")]
    pub disable_required_tags: Option<bool>,
}

/// Query is the query for a metric-based SLO. Required if type is metric. Note that only the `sum by` aggregator is allowed, which sums all request counts. `Average`, `max`, nor `min` request aggregators are not supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatadogSLOQuery {
    /// Denominator is a Datadog metric query for total (valid) events.
    pub denominator: String,
    /// Numerator is a Datadog metric query for good events.
    pub numerator: String,
}

/// DatadogSLOStatus defines the observed state of a DatadogSLO.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatadogSLOStatus {
    /// Conditions represents the latest available observations of the state of a DatadogSLO.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DatadogSLOStatusConditions>>,
    /// Created is the time the SLO was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Creator is the identity of the SLO creator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// CurrentHash tracks the hash of the current DatadogSLOSpec to know if the Spec has changed and needs an update.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentHash")]
    pub current_hash: Option<String>,
    /// ID is the SLO ID generated in Datadog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// LastForceSyncTime is the last time the API SLO was last force synced with the DatadogSLO resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastForceSyncTime")]
    pub last_force_sync_time: Option<String>,
    /// SyncStatus shows the health of syncing the SLO state to Datadog.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncStatus")]
    pub sync_status: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{     // Represents the observations of a foo's current state.     // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"     // +patchMergeKey=type     // +patchStrategy=merge     // +listType=map     // +listMapKey=type     Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///      // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatadogSLOStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: DatadogSLOStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{     // Represents the observations of a foo's current state.     // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"     // +patchMergeKey=type     // +patchStrategy=merge     // +listType=map     // +listMapKey=type     Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///      // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatadogSLOStatusConditionsStatus {
    True,
    False,
    Unknown,
}
