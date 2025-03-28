// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/dataprotection.kubeblocks.io/v1alpha1/backupschedules.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackupScheduleSpec defines the desired state of BackupSchedule.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dataprotection.kubeblocks.io", version = "v1alpha1", kind = "BackupSchedule", plural = "backupschedules")]
#[kube(namespaced)]
#[kube(status = "BackupScheduleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupScheduleSpec {
    /// Specifies the backupPolicy to be applied for the `schedules`.
    #[serde(rename = "backupPolicyName")]
    pub backup_policy_name: String,
    /// Defines the list of backup schedules.
    pub schedules: Vec<BackupScheduleSchedules>,
    /// Defines the deadline in minutes for starting the backup workload if it
    /// misses its scheduled time for any reason.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startingDeadlineMinutes")]
    pub starting_deadline_minutes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleSchedules {
    /// Specifies the backup method name that is defined in backupPolicy.
    #[serde(rename = "backupMethod")]
    pub backup_method: String,
    /// Specifies the cron expression for the schedule. The timezone is in UTC.
    /// see https://en.wikipedia.org/wiki/Cron.
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,
    /// Specifies whether the backup schedule is enabled or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Specifies the name of the schedule. Names cannot be duplicated.
    /// If the name is empty, it will be considered the same as the value of the backupMethod below.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specifies a list of name-value pairs representing parameters and their corresponding values.
    /// Parameters match the schema specified in the `actionset.spec.parametersSchema`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<BackupScheduleSchedulesParameters>>,
    /// Determines the duration for which the backup should be kept.
    /// KubeBlocks will remove all backups that are older than the RetentionPeriod.
    /// For example, RetentionPeriod of `30d` will keep only the backups of last 30 days.
    /// Sample duration format:
    /// 
    /// 
    /// - years: 	2y
    /// - months: 	6mo
    /// - days: 		30d
    /// - hours: 	12h
    /// - minutes: 	30m
    /// 
    /// 
    /// You can also combine the above durations. For example: 30d12h30m
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionPeriod")]
    pub retention_period: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleSchedulesParameters {
    /// Represents the name of the parameter.
    pub name: String,
    /// Represents the parameter values.
    pub value: String,
}

/// BackupScheduleStatus defines the observed state of BackupSchedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleStatus {
    /// Represents an error that caused the backup to fail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Represents the most recent generation observed for this BackupSchedule.
    /// It refers to the BackupSchedule's generation, which is updated on mutation
    /// by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Describes the phase of the BackupSchedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Describes the status of each schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<BTreeMap<String, BackupScheduleStatusSchedules>>,
}

/// Describes the status of each schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupScheduleStatusSchedules {
    /// Represents an error that caused the backup to fail.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Records the last time the backup was scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    /// Records the last time the backup was successfully completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSuccessfulTime")]
    pub last_successful_time: Option<String>,
    /// Describes the phase of the schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

