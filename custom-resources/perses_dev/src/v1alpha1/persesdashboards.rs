// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/perses/perses-operator/perses.dev/v1alpha1/persesdashboards.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "perses.dev", version = "v1alpha1", kind = "PersesDashboard", plural = "persesdashboards")]
#[kube(namespaced)]
#[kube(status = "PersesDashboardStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PersesDashboardSpec {
    /// Datasources is an optional list of datasource definition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datasources: Option<BTreeMap<String, PersesDashboardDatasources>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<PersesDashboardDisplay>,
    /// Duration is the default time range to use when getting data to fill the dashboard
    pub duration: String,
    pub layouts: Vec<PersesDashboardLayouts>,
    pub panels: BTreeMap<String, PersesDashboardPanels>,
    /// RefreshInterval is the default refresh interval to use when landing on the dashboard
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PersesDashboardVariables>>,
}

/// Datasources is an optional list of datasource definition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardDatasources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<PersesDashboardDatasourcesDisplay>,
    /// Plugin will contain the datasource configuration.
    /// The data typed is available in Cue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<PersesDashboardDatasourcesPlugin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardDatasourcesDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Plugin will contain the datasource configuration.
/// The data typed is available in Cue.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardDatasourcesPlugin {
    pub kind: String,
    pub spec: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardLayouts {
    pub kind: String,
    pub spec: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PersesDashboardPanelsSpec>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpec {
    pub display: PersesDashboardPanelsSpecDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<PersesDashboardPanelsSpecLinks>>,
    pub plugin: PersesDashboardPanelsSpecPlugin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<PersesDashboardPanelsSpecQueries>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renderVariables")]
    pub render_variables: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetBlank")]
    pub target_blank: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecPlugin {
    pub kind: String,
    pub spec: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecQueries {
    pub kind: String,
    pub spec: PersesDashboardPanelsSpecQueriesSpec,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecQueriesSpec {
    pub plugin: PersesDashboardPanelsSpecQueriesSpecPlugin,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardPanelsSpecQueriesSpecPlugin {
    pub kind: String,
    pub spec: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardVariables {
    /// Kind is the type of the variable. Depending on the value of Kind, it will change the content of Spec.
    pub kind: String,
    pub spec: serde_json::Value,
}

/// PersesDashboardStatus defines the observed state of PersesDashboard
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PersesDashboardStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

