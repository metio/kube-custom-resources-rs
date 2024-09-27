// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/ocm-agent-operator/ocmagent.managed.openshift.io/v1alpha1/managedfleetnotificationrecords.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ManagedFleetNotificationRecordStatus defines the observed state of ManagedFleetNotificationRecord
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedFleetNotificationRecordStatus {
    /// Managed Fleet Notification name
    #[serde(rename = "managementCluster")]
    pub management_cluster: String,
    /// An array structure to record the history for each hosted cluster
    #[serde(rename = "notificationRecordByName")]
    pub notification_record_by_name: Vec<ManagedFleetNotificationRecordStatusNotificationRecordByName>,
}

/// NotificationRecordByName groups the notification record item by notification name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedFleetNotificationRecordStatusNotificationRecordByName {
    /// Name of the notification
    #[serde(rename = "notificationName")]
    pub notification_name: String,
    /// Notification record item with the notification name
    #[serde(rename = "notificationRecordItems")]
    pub notification_record_items: Vec<ManagedFleetNotificationRecordStatusNotificationRecordByNameNotificationRecordItems>,
    /// Resend interval for the notification
    #[serde(rename = "resendWait")]
    pub resend_wait: i32,
}

/// NotificationRecordItem defines the basic structure of a notification record
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedFleetNotificationRecordStatusNotificationRecordByNameNotificationRecordItems {
    /// FiringNotificationSentCount records the number of notifications sent for the alert status firing
    #[serde(rename = "firingNotificationSentCount")]
    pub firing_notification_sent_count: i64,
    /// The uuid for the hosted cluster per entry
    #[serde(rename = "hostedClusterID")]
    pub hosted_cluster_id: String,
    /// The last notification sent timestamp
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// ResolvedNotificationSentCount records the number of notifications sent for the alert status resolving
    #[serde(rename = "resolvedNotificationSentCount")]
    pub resolved_notification_sent_count: i64,
}
