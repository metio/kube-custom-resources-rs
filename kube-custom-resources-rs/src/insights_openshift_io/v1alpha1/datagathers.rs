// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/insights.openshift.io/v1alpha1/datagathers.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// spec holds user settable values for configuration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "insights.openshift.io", version = "v1alpha1", kind = "DataGather", plural = "datagathers")]
#[kube(status = "DataGatherStatus")]
#[kube(schema = "disabled")]
pub struct DataGatherSpec {
    /// dataPolicy allows user to enable additional global obfuscation of the IP addresses and base domain in the Insights archive data. Valid values are "ClearText" and "ObfuscateNetworking". When set to ClearText the data is not obfuscated. When set to ObfuscateNetworking the IP addresses and the cluster domain name are obfuscated. When omitted, this means no opinion and the platform is left to choose a reasonable default, which is subject to change over time. The current default is ClearText.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataPolicy")]
    pub data_policy: Option<DataGatherDataPolicy>,
    /// gatherers is a list of gatherers configurations. The particular gatherers IDs can be found at https://github.com/openshift/insights-operator/blob/master/docs/gathered-data.md. Run the following command to get the names of last active gatherers: "oc get insightsoperators.operator.openshift.io cluster -o json | jq '.status.gatherStatus.gatherers[].name'"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gatherers: Option<Vec<DataGatherGatherers>>,
}

/// spec holds user settable values for configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataGatherDataPolicy {
    #[serde(rename = "")]
    KopiumEmpty,
    ClearText,
    ObfuscateNetworking,
}

/// gathererConfig allows to configure specific gatherers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherGatherers {
    /// name is the name of specific gatherer
    pub name: String,
    /// state allows you to configure specific gatherer. Valid values are "Enabled", "Disabled" and omitted. When omitted, this means no opinion and the platform is left to choose a reasonable default. The current default is Enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DataGatherGatherersState>,
}

/// gathererConfig allows to configure specific gatherers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataGatherGatherersState {
    #[serde(rename = "")]
    KopiumEmpty,
    Enabled,
    Disabled,
}

/// status holds observed values from the cluster. They may not be overridden.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherStatus {
    /// conditions provide details on the status of the gatherer job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// dataGatherState reflects the current state of the data gathering process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataGatherState")]
    pub data_gather_state: Option<DataGatherStatusDataGatherState>,
    /// finishTime is the time when Insights data gathering finished.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finishTime")]
    pub finish_time: Option<String>,
    /// gatherers is a list of active gatherers (and their statuses) in the last gathering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gatherers: Option<Vec<DataGatherStatusGatherers>>,
    /// insightsReport provides general Insights analysis results. When omitted, this means no data gathering has taken place yet or the corresponding Insights analysis (identified by "insightsRequestID") is not available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insightsReport")]
    pub insights_report: Option<DataGatherStatusInsightsReport>,
    /// insightsRequestID is an Insights request ID to track the status of the Insights analysis (in console.redhat.com processing pipeline) for the corresponding Insights data archive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insightsRequestID")]
    pub insights_request_id: Option<String>,
    /// relatedObjects is a list of resources which are useful when debugging or inspecting the data gathering Pod
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relatedObjects")]
    pub related_objects: Option<Vec<DataGatherStatusRelatedObjects>>,
    /// startTime is the time when Insights data gathering started.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// status holds observed values from the cluster. They may not be overridden.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataGatherStatusDataGatherState {
    Running,
    Completed,
    Failed,
    Pending,
}

/// gathererStatus represents information about a particular data gatherer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherStatusGatherers {
    /// conditions provide details on the status of each gatherer.
    pub conditions: Vec<Condition>,
    /// lastGatherDuration represents the time spent gathering.
    #[serde(rename = "lastGatherDuration")]
    pub last_gather_duration: String,
    /// name is the name of the gatherer.
    pub name: String,
}

/// insightsReport provides general Insights analysis results. When omitted, this means no data gathering has taken place yet or the corresponding Insights analysis (identified by "insightsRequestID") is not available.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherStatusInsightsReport {
    /// downloadedAt is the time when the last Insights report was downloaded. An empty value means that there has not been any Insights report downloaded yet and it usually appears in disconnected clusters (or clusters when the Insights data gathering is disabled).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "downloadedAt")]
    pub downloaded_at: Option<String>,
    /// healthChecks provides basic information about active Insights health checks in a cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthChecks")]
    pub health_checks: Option<Vec<DataGatherStatusInsightsReportHealthChecks>>,
    /// uri provides the URL link from which the report was downloaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// healthCheck represents an Insights health check attributes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherStatusInsightsReportHealthChecks {
    /// advisorURI provides the URL link to the Insights Advisor.
    #[serde(rename = "advisorURI")]
    pub advisor_uri: String,
    /// description provides basic description of the healtcheck.
    pub description: String,
    /// state determines what the current state of the health check is. Health check is enabled by default and can be disabled by the user in the Insights advisor user interface.
    pub state: DataGatherStatusInsightsReportHealthChecksState,
    /// totalRisk of the healthcheck. Indicator of the total risk posed by the detected issue; combination of impact and likelihood. The values can be from 1 to 4, and the higher the number, the more important the issue.
    #[serde(rename = "totalRisk")]
    pub total_risk: i32,
}

/// healthCheck represents an Insights health check attributes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataGatherStatusInsightsReportHealthChecksState {
    Enabled,
    Disabled,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataGatherStatusRelatedObjects {
    /// group is the API Group of the Resource. Enter empty string for the core group. This value should consist of only lowercase alphanumeric characters, hyphens and periods. Example: "", "apps", "build.openshift.io", etc.
    pub group: String,
    /// name of the referent.
    pub name: String,
    /// namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the type that is being referenced. It is normally the plural form of the resource kind in lowercase. This value should consist of only lowercase alphanumeric characters and hyphens. Example: "deployments", "deploymentconfigs", "pods", etc.
    pub resource: String,
}

