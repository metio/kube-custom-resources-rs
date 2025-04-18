// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/alexandrevilain/temporal-operator/temporal.io/v1beta1/temporalschedules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TemporalScheduleSpec defines the desired state of Schedule.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "temporal.io", version = "v1beta1", kind = "TemporalSchedule", plural = "temporalschedules")]
#[kube(namespaced)]
#[kube(status = "TemporalScheduleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TemporalScheduleSpec {
    /// AllowDeletion makes the controller delete the Temporal schedule if the
    /// CRD is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowDeletion")]
    pub allow_deletion: Option<bool>,
    /// Memo is optional non-indexed info that will be shown in list workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<BTreeMap<String, serde_json::Value>>,
    /// Reference to the temporal namespace the schedule will be created in.
    #[serde(rename = "namespaceRef")]
    pub namespace_ref: TemporalScheduleNamespaceRef,
    /// Schedule contains all fields related to a schedule.
    pub schedule: TemporalScheduleSchedule,
    /// SearchAttributes is optional indexed info that can be used in query of List/Scan/Count workflow APIs. The key
    /// and value type must be registered on Temporal server side. For supported operations on different server versions
    /// see [Visibility].
    /// 
    /// [Visibility]: https://docs.temporal.io/visibility
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "searchAttributes")]
    pub search_attributes: Option<BTreeMap<String, serde_json::Value>>,
}

/// Reference to the temporal namespace the schedule will be created in.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleNamespaceRef {
    /// The name of the temporal object to reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The namespace of the temporal object to reference.
    /// Defaults to the namespace of the requested resource if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Schedule contains all fields related to a schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleSchedule {
    /// ScheduleAction contains the actions that the schedule should perform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<TemporalScheduleScheduleAction>,
    /// SchedulePolicies represent policies for overlaps, catchups, pause on failure, and workflow ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<TemporalScheduleSchedulePolicy>,
    /// ScheduleSpec is a complete description of a set of absolute timestamps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<TemporalScheduleScheduleSpec>,
    /// ScheduleState describes the current state of a schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TemporalScheduleScheduleState>,
}

/// ScheduleAction contains the actions that the schedule should perform.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleAction {
    /// ScheduleWorkflowAction describes a workflow to launch.
    pub workflow: TemporalScheduleScheduleActionWorkflow,
}

/// ScheduleWorkflowAction describes a workflow to launch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleActionWorkflow {
    /// WorkflowExecutionTimeout is the timeout for duration of workflow execution.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionTimeout")]
    pub execution_timeout: Option<String>,
    /// WorkflowID represents the business identifier of the workflow execution.
    /// The WorkflowID of the started workflow may not match this exactly,
    /// it may have a timestamp appended for uniqueness.
    /// Defaults to a uuid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Inputs contains arguments to pass to the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    /// Memo is optional non-indexed info that will be shown in list workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<BTreeMap<String, serde_json::Value>>,
    /// RetryPolicy is the retry policy for the workflow. If a retry policy is specified,
    /// in case of workflow failure server will start new workflow execution if
    /// needed based on the retry policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryPolicy")]
    pub retry_policy: Option<TemporalScheduleScheduleActionWorkflowRetryPolicy>,
    /// WorkflowRunTimeout is the timeout for duration of a single workflow run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runTimeout")]
    pub run_timeout: Option<String>,
    /// SearchAttributes is optional indexed info that can be used in query of List/Scan/Count workflow APIs. The key
    /// and value type must be registered on Temporal server side. For supported operations on different server versions
    /// see [Visibility].
    /// 
    /// [Visibility]: https://docs.temporal.io/visibility
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "searchAttributes")]
    pub search_attributes: Option<BTreeMap<String, serde_json::Value>>,
    /// TaskQueue represents a workflow task queue.
    /// This is also the name of the activity task queue on which activities are scheduled.
    #[serde(rename = "taskQueue")]
    pub task_queue: String,
    /// WorkflowTaskTimeout is The timeout for processing workflow task from the time the worker
    /// pulled this task.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "taskTimeout")]
    pub task_timeout: Option<String>,
    /// WorkflowType represents the identifier used by a workflow author to define the workflow
    /// Workflow type name.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// RetryPolicy is the retry policy for the workflow. If a retry policy is specified,
/// in case of workflow failure server will start new workflow execution if
/// needed based on the retry policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleActionWorkflowRetryPolicy {
    /// Coefficient used to calculate the next retry interval.
    /// The next retry interval is previous interval multiplied by the coefficient.
    /// Must be 1 or larger.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffCoefficient")]
    pub backoff_coefficient: Option<IntOrString>,
    /// Interval of the first retry. If retryBackoffCoefficient is 1.0 then it is used for all retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialInterval")]
    pub initial_interval: Option<String>,
    /// Maximum number of attempts. When exceeded the retries stop even if not expired yet.
    /// 1 disables retries. 0 means unlimited (up to the timeouts).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumAttempts")]
    pub maximum_attempts: Option<i32>,
    /// Maximum interval between retries. Exponential backoff leads to interval increase.
    /// This value is the cap of the increase. Default is 100x of the initial interval.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumInterval")]
    pub maximum_interval: Option<String>,
    /// Non-Retryable errors types. Will stop retrying if the error type matches this list. Note that
    /// this is not a substring match, the error *type* (not message) must match exactly.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonRetryableErrorTypes")]
    pub non_retryable_error_types: Option<Vec<String>>,
}

/// SchedulePolicies represent policies for overlaps, catchups, pause on failure, and workflow ID.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleSchedulePolicy {
    /// CatchupWindow The Temporal Server might be down or unavailable at the time
    /// when a Schedule should take an Action. When the Server comes back up,
    /// CatchupWindow controls which missed Actions should be taken at that point.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "catchupWindow")]
    pub catchup_window: Option<String>,
    /// Overlap controls what happens when an Action would be started by a
    /// Schedule at the same time that an older Action is still running.
    /// 
    /// Supported values:
    /// 
    /// "skip" - Default. Nothing happens; the Workflow Execution is not started.
    /// 
    /// "bufferOne" - Starts the Workflow Execution as soon as the current one completes.
    /// The buffer is limited to one. If another Workflow Execution is supposed to start,
    /// but one is already in the buffer, only the one in the buffer eventually starts.
    /// 
    /// "bufferAll" - Allows an unlimited number of Workflows to buffer. They are started sequentially.
    /// 
    /// "cancelOther" - Cancels the running Workflow Execution, and then starts the new one
    /// after the old one completes cancellation.
    /// 
    /// "terminateOther" - Terminates the running Workflow Execution and starts the new one immediately.
    /// 
    /// "allowAll" - Starts any number of concurrent Workflow Executions.
    /// With this policy (and only this policy), more than one Workflow Execution,
    /// started by the Schedule, can run simultaneously.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlap: Option<TemporalScheduleSchedulePolicyOverlap>,
    /// PauseOnFailure if true, and a workflow run fails or times out, turn on "paused".
    /// This applies after retry policies: the full chain of retries must fail to trigger a pause here.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pauseOnFailure")]
    pub pause_on_failure: Option<bool>,
}

/// SchedulePolicies represent policies for overlaps, catchups, pause on failure, and workflow ID.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TemporalScheduleSchedulePolicyOverlap {
    #[serde(rename = "skip")]
    Skip,
    #[serde(rename = "bufferOne")]
    BufferOne,
    #[serde(rename = "bufferAll")]
    BufferAll,
    #[serde(rename = "cancelOther")]
    CancelOther,
    #[serde(rename = "terminateOther")]
    TerminateOther,
    #[serde(rename = "allowAll")]
    AllowAll,
}

/// ScheduleSpec is a complete description of a set of absolute timestamps.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpec {
    /// Calendars represents calendar-based specifications of times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<TemporalScheduleScheduleSpecCalendars>>,
    /// Crons are cron based specifications of times.
    /// Crons is provided for easy migration from legacy Cron Workflows. For new
    /// use cases, we recommend using ScheduleSpec.Calendars or ScheduleSpec.
    /// Intervals for readability and maintainability. Once a schedule is created all
    /// expressions in Crons will be translated to ScheduleSpec.Calendars on the server.
    /// 
    /// For example, `0 12 * * MON-WED,FRI` is every M/Tu/W/F at noon
    /// 
    /// The string can have 5, 6, or 7 fields, separated by spaces, and they are interpreted in the
    /// same way as a ScheduleCalendarSpec:
    /// 
    /// 	- 5 fields:         Minute, Hour, DayOfMonth, Month, DayOfWeek
    /// 
    /// 	- 6 fields:         Minute, Hour, DayOfMonth, Month, DayOfWeek, Year
    /// 
    /// 	- 7 fields: Second, Minute, Hour, DayOfMonth, Month, DayOfWeek, Year
    /// 
    /// Notes:
    /// 	- If Year is not given, it defaults to *.
    /// 	- If Second is not given, it defaults to 0.
    /// 	- Shorthands @yearly, @monthly, @weekly, @daily, and @hourly are also
    /// 		accepted instead of the 5-7 time fields.
    /// 	- @every <interval>[/<phase>] is accepted and gets compiled into an
    /// 		IntervalSpec instead. <interval> and <phase> should be a decimal integer
    /// 		with a unit suffix s, m, h, or d.
    /// 	- Optionally, the string can be preceded by CRON_TZ=<time zone name> or
    /// 		TZ=<time zone name>, which will get copied to ScheduleSpec.TimeZoneName. (In which case the ScheduleSpec.TimeZone field should be left empty.)
    /// 	- Optionally, "#" followed by a comment can appear at the end of the string.
    /// 	- Note that the special case that some cron implementations have for
    /// 		treating DayOfMonth and DayOfWeek as "or" instead of "and" when both
    /// 		are set is not implemented.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crons: Option<Vec<String>>,
    /// EndAt represents the end of the schedule. Any times after `endAt` will be skipped.
    /// Defaults to the end of time.
    /// For example: 2024-05-13T00:00:00Z
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endAt")]
    pub end_at: Option<String>,
    /// ExcludeCalendars defines any matching times that will be skipped.
    /// 
    /// All fields of the ScheduleCalendarSpec including seconds must match a time for the time to be skipped.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeCalendars")]
    pub exclude_calendars: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendars>>,
    /// Intervals represents interval-based specifications of times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intervals: Option<Vec<TemporalScheduleScheduleSpecIntervals>>,
    /// Jitter represents a duration that is used to apply a jitter to scheduled times.
    /// All times will be incremented by a random value from 0 to this amount of jitter, capped
    /// by the time until the next schedule.
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter: Option<String>,
    /// StartAt represents the start of the schedule. Any times before `startAt` will be skipped.
    /// Together, `startAt` and `endAt` make an inclusive interval.
    /// Defaults to the beginning of time.
    /// For example: 2024-05-13T00:00:00Z
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startAt")]
    pub start_at: Option<String>,
    /// TimeZoneName represents the IANA time zone name, for example `US/Pacific`.
    /// 
    /// The definition will be loaded by Temporal Server from the environment it runs in.
    /// 
    /// Calendar spec matching is based on literal matching of the clock time
    /// with no special handling of DST: if you write a calendar spec that fires
    /// at 2:30am and specify a time zone that follows DST, that action will not
    /// be triggered on the day that has no 2:30am. Similarly, an action that
    /// fires at 1:30am will be triggered twice on the day that has two 1:30s.
    /// 
    /// Note: No actions are taken on leap-seconds (e.g. 23:59:60 UTC).
    /// Defaults to UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timezoneName")]
    pub timezone_name: Option<String>,
}

/// ScheduleCalendarSpec is an event specification relative to the calendar, similar to a traditional cron specification.
/// A timestamp matches if at least one range of each field matches the
/// corresponding fields of the timestamp, except for year: if year is missing,
/// that means all years match. For all fields besides year, at least one Range must be present to match anything.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendars {
    /// Comment describes the intention of this schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// DayOfMonth range to match (1-31)
    /// Defaults to match all days.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dayOfMonth")]
    pub day_of_month: Option<Vec<TemporalScheduleScheduleSpecCalendarsDayOfMonth>>,
    /// DayOfWeek range to match (0-6; 0 is Sunday)
    /// Defaults to match all days of the week.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dayOfWeek")]
    pub day_of_week: Option<Vec<TemporalScheduleScheduleSpecCalendarsDayOfWeek>>,
    /// Hour range to match (0-23).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<Vec<TemporalScheduleScheduleSpecCalendarsHour>>,
    /// Minute range to match (0-59).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<Vec<TemporalScheduleScheduleSpecCalendarsMinute>>,
    /// Month range to match (1-12).
    /// Defaults to match all months.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<Vec<TemporalScheduleScheduleSpecCalendarsMonth>>,
    /// Second range to match (0-59).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second: Option<Vec<TemporalScheduleScheduleSpecCalendarsSecond>>,
    /// Year range to match.
    /// Defaults to match all years.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<Vec<TemporalScheduleScheduleSpecCalendarsYear>>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsDayOfMonth {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsDayOfWeek {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsHour {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsMinute {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsMonth {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsSecond {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecCalendarsYear {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// ScheduleCalendarSpec is an event specification relative to the calendar, similar to a traditional cron specification.
/// A timestamp matches if at least one range of each field matches the
/// corresponding fields of the timestamp, except for year: if year is missing,
/// that means all years match. For all fields besides year, at least one Range must be present to match anything.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendars {
    /// Comment describes the intention of this schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// DayOfMonth range to match (1-31)
    /// Defaults to match all days.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dayOfMonth")]
    pub day_of_month: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsDayOfMonth>>,
    /// DayOfWeek range to match (0-6; 0 is Sunday)
    /// Defaults to match all days of the week.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dayOfWeek")]
    pub day_of_week: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsDayOfWeek>>,
    /// Hour range to match (0-23).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsHour>>,
    /// Minute range to match (0-59).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsMinute>>,
    /// Month range to match (1-12).
    /// Defaults to match all months.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsMonth>>,
    /// Second range to match (0-59).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsSecond>>,
    /// Year range to match.
    /// Defaults to match all years.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<Vec<TemporalScheduleScheduleSpecExcludeCalendarsYear>>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsDayOfMonth {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsDayOfWeek {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsHour {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsMinute {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsMonth {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsSecond {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// If end < start, then end is interpreted as
/// equal to start. This means you can use a Range with start set to a value, and
/// end and step unset to represent a single value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecExcludeCalendarsYear {
    /// End of the range (inclusive).
    /// Defaults to start.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    /// Start of the range (inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Step to be take between each value.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// ScheduleIntervalSpec matches times that can be expressed as:
/// 
/// 	Epoch + (n * every) + offset
/// 
/// 	where n is all integers ≥ 0.
/// 
/// For example, an `every` of 1 hour with `offset` of zero would match every hour, on the hour. The same `every` but an `offset`
/// of 19 minutes would match every `xx:19:00`. An `every` of 28 days with `offset` zero would match `2022-02-17T00:00:00Z`
/// (among other times). The same `every` with `offset` of 3 days, 5 hours, and 23 minutes would match `2022-02-20T05:23:00Z`
/// instead.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleSpecIntervals {
    /// Every describes the period to repeat the interval.
    pub every: String,
    /// Offset is a fixed offset added to the intervals period.
    /// Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

/// ScheduleState describes the current state of a schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleScheduleState {
    /// LimitedActions limits actions. While true RemainingActions will be decremented for each action taken.
    /// Skipped actions (due to overlap policy) do not count against remaining actions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitedActions")]
    pub limited_actions: Option<bool>,
    /// Note is an informative human-readable message with contextual notes, e.g. the reason
    /// a Schedule is paused. The system may overwrite this message on certain
    /// conditions, e.g. when pause-on-failure happens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Paused is true if the schedule is paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// RemainingActions represents the Actions remaining in this Schedule.
    /// Once this number hits 0, no further Actions are taken.
    /// manual actions through backfill or ScheduleHandle.Trigger still run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remainingActions")]
    pub remaining_actions: Option<i64>,
}

/// TemporalScheduleStatus defines the observed state of Schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalScheduleStatus {
    /// Conditions represent the latest available observations of the Schedule state.
    pub conditions: Vec<Condition>,
}

