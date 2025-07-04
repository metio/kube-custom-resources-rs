// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/DataDog/datadog-operator/datadoghq.com/v1alpha1/datadogmonitors.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DatadogMonitorSpec defines the desired state of DatadogMonitor
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "datadoghq.com", version = "v1alpha1", kind = "DatadogMonitor", plural = "datadogmonitors")]
#[kube(namespaced)]
#[kube(status = "DatadogMonitorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DatadogMonitorSpec {
    /// ControllerOptions are the optional parameters in the DatadogMonitor controller
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerOptions")]
    pub controller_options: Option<DatadogMonitorControllerOptions>,
    /// Message is a message to include with notifications for this monitor
    pub message: String,
    /// Name is the monitor name
    pub name: String,
    /// Options are the optional parameters associated with your monitor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<DatadogMonitorOptions>,
    /// Priority is an integer from 1 (high) to 5 (low) indicating alert severity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// Query is the Datadog monitor query
    pub query: String,
    /// RestrictedRoles is a list of unique role identifiers to define which roles are allowed to edit the monitor.
    /// `restricted_roles` is the successor of `locked`. For more information about `locked` and `restricted_roles`,
    /// see the [monitor options docs](https://docs.datadoghq.com/monitors/guide/monitor_api_options/#permissions-options).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restrictedRoles")]
    pub restricted_roles: Option<Vec<String>>,
    /// Tags is the monitor tags associated with your monitor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Type is the monitor type
    #[serde(rename = "type")]
    pub r#type: DatadogMonitorType,
}

/// ControllerOptions are the optional parameters in the DatadogMonitor controller
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorControllerOptions {
    /// DisableRequiredTags disables the automatic addition of required tags to monitors.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableRequiredTags")]
    pub disable_required_tags: Option<bool>,
}

/// Options are the optional parameters associated with your monitor
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorOptions {
    /// A Boolean indicating whether to send a log sample when the log monitor triggers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableLogsSample")]
    pub enable_logs_sample: Option<bool>,
    /// A message to include with a re-notification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "escalationMessage")]
    pub escalation_message: Option<String>,
    /// Time (in seconds) to delay evaluation, as a non-negative integer. For example, if the value is set to 300 (5min),
    /// the timeframe is set to last_5m and the time is 7:00, the monitor evaluates data from 6:50 to 6:55.
    /// This is useful for AWS CloudWatch and other backfilled metrics to ensure the monitor always has data during evaluation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evaluationDelay")]
    pub evaluation_delay: Option<i64>,
    /// A Boolean indicating whether the log alert monitor triggers a single alert or multiple alerts when any group breaches a threshold.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupbySimpleMonitor")]
    pub groupby_simple_monitor: Option<bool>,
    /// A Boolean indicating whether notifications from this monitor automatically inserts its triggering tags into the title.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeTags")]
    pub include_tags: Option<bool>,
    /// DEPRECATED: Whether or not the monitor is locked (only editable by creator and admins). Use `restricted_roles` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Time (in seconds) to allow a host to boot and applications to fully start before starting the evaluation of
    /// monitor results. Should be a non negative integer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newGroupDelay")]
    pub new_group_delay: Option<i64>,
    /// The number of minutes before a monitor notifies after data stops reporting. Datadog recommends at least 2x the
    /// monitor timeframe for metric alerts or 2 minutes for service checks. If omitted, 2x the evaluation timeframe
    /// is used for metric alerts, and 24 hours is used for service checks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noDataTimeframe")]
    pub no_data_timeframe: Option<i64>,
    /// An enum that toggles the display of additional content sent in the monitor notification.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notificationPresetName")]
    pub notification_preset_name: Option<String>,
    /// A Boolean indicating whether tagged users are notified on changes to this monitor.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notifyAudit")]
    pub notify_audit: Option<bool>,
    /// A string indicating the granularity a monitor alerts on. Only available for monitors with groupings.
    /// For instance, a monitor grouped by cluster, namespace, and pod can be configured to only notify on each new
    /// cluster violating the alert conditions by setting notify_by to ["cluster"]. Tags mentioned in notify_by must
    /// be a subset of the grouping tags in the query. For example, a query grouped by cluster and namespace cannot
    /// notify on region. Setting notify_by to [*] configures the monitor to notify as a simple-alert.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notifyBy")]
    pub notify_by: Option<Vec<String>>,
    /// A Boolean indicating whether this monitor notifies when data stops reporting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notifyNoData")]
    pub notify_no_data: Option<bool>,
    /// An enum that controls how groups or monitors are treated if an evaluation does not return data points.
    /// The default option results in different behavior depending on the monitor query type.
    /// For monitors using Count queries, an empty monitor evaluation is treated as 0 and is compared to the threshold conditions.
    /// For monitors using any query type other than Count, for example Gauge, Measure, or Rate, the monitor shows the last known status.
    /// This option is only available for APM Trace Analytics, Audit Trail, CI, Error Tracking, Event, Logs, and RUM monitors
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onMissingData")]
    pub on_missing_data: Option<String>,
    /// The number of minutes after the last notification before a monitor re-notifies on the current status.
    /// It only re-notifies if it’s not resolved.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renotifyInterval")]
    pub renotify_interval: Option<i64>,
    /// The number of times re-notification messages should be sent on the current status at the provided re-notification interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renotifyOccurrences")]
    pub renotify_occurrences: Option<i64>,
    /// The types of statuses for which re-notification messages should be sent. Valid values are alert, warn, no data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renotifyStatuses")]
    pub renotify_statuses: Option<Vec<String>>,
    /// A Boolean indicating whether this monitor needs a full window of data before it’s evaluated. We highly
    /// recommend you set this to false for sparse metrics, otherwise some evaluations are skipped. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireFullWindow")]
    pub require_full_window: Option<bool>,
    /// Configuration options for scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulingOptions")]
    pub scheduling_options: Option<DatadogMonitorOptionsSchedulingOptions>,
    /// A struct of the alerting time window options.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "thresholdWindows")]
    pub threshold_windows: Option<DatadogMonitorOptionsThresholdWindows>,
    /// A struct of the different monitor threshold values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<DatadogMonitorOptionsThresholds>,
    /// The number of hours of the monitor not reporting data before it automatically resolves from a triggered state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutH")]
    pub timeout_h: Option<i64>,
}

/// Configuration options for scheduling.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorOptionsSchedulingOptions {
    /// Configuration options for the evaluation window. If hour_starts is set, no other fields may be set.
    /// Otherwise, day_starts and month_starts must be set together.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evaluationWindow")]
    pub evaluation_window: Option<DatadogMonitorOptionsSchedulingOptionsEvaluationWindow>,
}

/// Configuration options for the evaluation window. If hour_starts is set, no other fields may be set.
/// Otherwise, day_starts and month_starts must be set together.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorOptionsSchedulingOptionsEvaluationWindow {
    /// The time of the day at which a one day cumulative evaluation window starts. Must be defined in UTC time in HH:mm format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dayStarts")]
    pub day_starts: Option<String>,
    /// The minute of the hour at which a one hour cumulative evaluation window starts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hourStarts")]
    pub hour_starts: Option<i32>,
    /// The day of the month at which a one month cumulative evaluation window starts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monthStarts")]
    pub month_starts: Option<i32>,
}

/// A struct of the alerting time window options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorOptionsThresholdWindows {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recoveryWindow")]
    pub recovery_window: Option<String>,
    /// Describes how long a metric must be anomalous before an alert triggers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerWindow")]
    pub trigger_window: Option<String>,
}

/// A struct of the different monitor threshold values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorOptionsThresholds {
    /// The monitor CRITICAL threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<String>,
    /// The monitor CRITICAL recovery threshold.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "criticalRecovery")]
    pub critical_recovery: Option<String>,
    /// The monitor OK threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ok: Option<String>,
    /// The monitor UNKNOWN threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<String>,
    /// The monitor WARNING threshold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    /// The monitor WARNING recovery threshold.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "warningRecovery")]
    pub warning_recovery: Option<String>,
}

/// DatadogMonitorSpec defines the desired state of DatadogMonitor
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatadogMonitorType {
    #[serde(rename = "metric alert")]
    MetricAlert,
    #[serde(rename = "query alert")]
    QueryAlert,
    #[serde(rename = "service check")]
    ServiceCheck,
    #[serde(rename = "event alert")]
    EventAlert,
    #[serde(rename = "log alert")]
    LogAlert,
    #[serde(rename = "process alert")]
    ProcessAlert,
    #[serde(rename = "rum alert")]
    RumAlert,
    #[serde(rename = "trace-analytics alert")]
    TraceAnalyticsAlert,
    #[serde(rename = "slo alert")]
    SloAlert,
    #[serde(rename = "event-v2 alert")]
    EventV2Alert,
    #[serde(rename = "audit alert")]
    AuditAlert,
    #[serde(rename = "composite")]
    Composite,
}

/// DatadogMonitorStatus defines the observed state of DatadogMonitor
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorStatus {
    /// Conditions Represents the latest available observations of a DatadogMonitor's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Created is the time the monitor was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Creator is the identify of the monitor creator
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// CurrentHash tracks the hash of the current DatadogMonitorSpec to know
    /// if the Spec has changed and needs an update
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentHash")]
    pub current_hash: Option<String>,
    /// DowntimeStatus defines whether the monitor is downtimed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downtimeStatus")]
    pub downtime_status: Option<DatadogMonitorStatusDowntimeStatus>,
    /// ID is the monitor ID generated in Datadog
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// MonitorLastForceSyncTime is the last time the API monitor was last force synced with the DatadogMonitor resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorLastForceSyncTime")]
    pub monitor_last_force_sync_time: Option<String>,
    /// MonitorState is the overall state of monitor
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorState")]
    pub monitor_state: Option<String>,
    /// MonitorStateLastTransitionTime is the last time the monitor state changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorStateLastTransitionTime")]
    pub monitor_state_last_transition_time: Option<String>,
    /// MonitorStateLastUpdateTime is the last time the monitor state updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorStateLastUpdateTime")]
    pub monitor_state_last_update_time: Option<String>,
    /// MonitorStateSyncStatus shows the health of syncing the monitor state to Datadog
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitorStateSyncStatus")]
    pub monitor_state_sync_status: Option<String>,
    /// Primary defines whether the monitor is managed by the Kubernetes custom
    /// resource (true) or outside Kubernetes (false)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// TriggeredState only includes details for monitor groups that are triggering
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggeredState")]
    pub triggered_state: Option<Vec<DatadogMonitorStatusTriggeredState>>,
}

/// DowntimeStatus defines whether the monitor is downtimed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorStatusDowntimeStatus {
    /// DowntimeID is the downtime ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downtimeID")]
    pub downtime_id: Option<i64>,
    /// IsDowntimed shows the downtime status of the monitor.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDowntimed")]
    pub is_downtimed: Option<bool>,
}

/// DatadogMonitorTriggeredState represents the details of a triggering DatadogMonitor
/// The DatadogMonitor is triggering if one of its groups is in Alert, Warn, or No Data
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMonitorStatusTriggeredState {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// MonitorGroup is the name of the triggering group
    #[serde(rename = "monitorGroup")]
    pub monitor_group: String,
    /// DatadogMonitorState represents the overall DatadogMonitor state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

