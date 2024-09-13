// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/DataDog/datadog-operator/datadoghq.com/v1alpha1/datadogmetrics.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DatadogMetricSpec defines the desired state of DatadogMetric
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "datadoghq.com", version = "v1alpha1", kind = "DatadogMetric", plural = "datadogmetrics")]
#[kube(namespaced)]
#[kube(status = "DatadogMetricStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DatadogMetricSpec {
    /// ExternalMetricName is reserved for internal use
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalMetricName")]
    pub external_metric_name: Option<String>,
    /// MaxAge provides the max age for the metric query (overrides the default setting `external_metrics_provider.max_age`)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<String>,
    /// Query is the raw datadog query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// TimeWindow provides the time window for the metric query, defaults to MaxAge.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeWindow")]
    pub time_window: Option<String>,
}

/// DatadogMetricStatus defines the observed state of DatadogMetric
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatadogMetricStatus {
    /// List of autoscalers currently using this DatadogMetric
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoscalerReferences")]
    pub autoscaler_references: Option<String>,
    /// Conditions Represents the latest available observations of a DatadogMetric's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Value is the latest value of the metric
    #[serde(rename = "currentValue")]
    pub current_value: String,
}

