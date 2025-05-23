// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1beta1/helmcharts.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// HelmChartSpec defines the desired state of a Helm chart.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1beta1", kind = "HelmChart", plural = "helmcharts")]
#[kube(namespaced)]
#[kube(status = "HelmChartStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct HelmChartSpec {
    /// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessFrom")]
    pub access_from: Option<HelmChartAccessFrom>,
    /// The name or path the Helm chart is available at in the SourceRef.
    pub chart: String,
    /// The interval at which to check the Source for updates.
    pub interval: String,
    /// Determines what enables the creation of a new artifact. Valid values are
    /// ('ChartVersion', 'Revision').
    /// See the documentation of the values for an explanation on their behavior.
    /// Defaults to ChartVersion when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileStrategy")]
    pub reconcile_strategy: Option<HelmChartReconcileStrategy>,
    /// The reference to the Source the chart is available at.
    #[serde(rename = "sourceRef")]
    pub source_ref: HelmChartSourceRef,
    /// This flag tells the controller to suspend the reconciliation of this source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Alternative values file to use as the default chart values, expected to
    /// be a relative path in the SourceRef. Deprecated in favor of ValuesFiles,
    /// for backwards compatibility the file defined here is merged before the
    /// ValuesFiles items. Ignored when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFile")]
    pub values_file: Option<String>,
    /// Alternative list of values files to use as the chart values (values.yaml
    /// is not included by default), expected to be a relative path in the SourceRef.
    /// Values files are merged in the order of this list with the last file overriding
    /// the first. Ignored when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFiles")]
    pub values_files: Option<Vec<String>>,
    /// The chart version semver expression, ignored for charts from GitRepository
    /// and Bucket sources. Defaults to latest when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// AccessFrom defines an Access Control List for allowing cross-namespace references to this object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartAccessFrom {
    /// NamespaceSelectors is the list of namespace selectors to which this ACL applies.
    /// Items in this list are evaluated using a logical OR operation.
    #[serde(rename = "namespaceSelectors")]
    pub namespace_selectors: Vec<HelmChartAccessFromNamespaceSelectors>,
}

/// NamespaceSelector selects the namespaces to which this ACL applies.
/// An empty map of MatchLabels matches all namespaces in a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartAccessFromNamespaceSelectors {
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// HelmChartSpec defines the desired state of a Helm chart.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmChartReconcileStrategy {
    ChartVersion,
    Revision,
}

/// The reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmChartSourceRef {
    /// APIVersion of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent, valid values are ('HelmRepository', 'GitRepository',
    /// 'Bucket').
    pub kind: HelmChartSourceRefKind,
    /// Name of the referent.
    pub name: String,
}

/// The reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmChartSourceRefKind {
    HelmRepository,
    GitRepository,
    Bucket,
}

/// HelmChartStatus defines the observed state of the HelmChart.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartStatus {
    /// Artifact represents the output of the last successful chart sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmChartStatusArtifact>,
    /// Conditions holds the conditions for the HelmChart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the download link for the last chart pulled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful chart sync.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartStatusArtifact {
    /// Checksum is the SHA256 checksum of the artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of this
    /// artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Path is the relative file path of this artifact.
    pub path: String,
    /// Revision is a human readable identifier traceable in the origin source
    /// system. It can be a Git commit SHA, Git tag, a Helm index timestamp, a Helm
    /// chart version, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// URL is the HTTP address of this artifact.
    pub url: String,
}

