// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/grafana/grafana-operator/grafana.integreatly.org/v1beta1/grafanamutetimings.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GrafanaMuteTimingSpec defines the desired state of GrafanaMuteTiming
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "grafana.integreatly.org", version = "v1beta1", kind = "GrafanaMuteTiming", plural = "grafanamutetimings")]
#[kube(namespaced)]
#[kube(status = "GrafanaMuteTimingStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GrafanaMuteTimingSpec {
    /// Allow the Operator to match this resource with Grafanas outside the current namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCrossNamespaceImport")]
    pub allow_cross_namespace_import: Option<bool>,
    /// Whether to enable or disable editing of the mute timing in Grafana UI
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// Selects Grafana instances for import
    #[serde(rename = "instanceSelector")]
    pub instance_selector: GrafanaMuteTimingInstanceSelector,
    /// A unique name for the mute timing
    pub name: String,
    /// How often the resource is synced, defaults to 10m0s if not set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncPeriod")]
    pub resync_period: Option<String>,
    /// Time intervals for muting
    pub time_intervals: Vec<GrafanaMuteTimingTimeIntervals>,
}

/// Selects Grafana instances for import
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaMuteTimingInstanceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GrafanaMuteTimingInstanceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaMuteTimingInstanceSelectorMatchExpressions {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaMuteTimingTimeIntervals {
    /// The date 1-31 of a month. Negative values can also be used to represent days that begin at the end of the month.
    /// For example: -1 for the last day of the month.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_of_month: Option<Vec<String>>,
    /// Depending on the location, the time range is displayed in local time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The months of the year in either numerical or the full calendar month.
    /// For example: 1, may.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub months: Option<Vec<String>>,
    /// The time inclusive of the start and exclusive of the end time (in UTC if no location has been selected, otherwise local time).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<Vec<GrafanaMuteTimingTimeIntervalsTimes>>,
    /// The day or range of days of the week.
    /// For example: monday, thursday
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<String>>,
    /// The year or years for the interval.
    /// For example: 2021
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub years: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaMuteTimingTimeIntervalsTimes {
    /// end time
    pub end_time: String,
    /// start time
    pub start_time: String,
}

/// The most recent observed state of a Grafana resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GrafanaMuteTimingStatus {
    /// Results when synchonizing resource with Grafana instances
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Last time the resource was synchronized with Grafana instances
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastResync")]
    pub last_resync: Option<String>,
}

