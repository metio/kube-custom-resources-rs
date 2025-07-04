// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/controlplaneio-fluxcd/flux-operator/fluxcd.controlplane.io/v1/fluxreports.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FluxReportSpec defines the observed state of a Flux installation.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "fluxcd.controlplane.io", version = "v1", kind = "FluxReport", plural = "fluxreports")]
#[kube(namespaced)]
#[kube(status = "FluxReportStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FluxReportSpec {
    /// Cluster is the version information of the Kubernetes cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<FluxReportCluster>,
    /// ComponentsStatus is the status of the Flux controller deployments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<FluxReportComponents>>,
    /// Distribution is the version information of the Flux installation.
    pub distribution: FluxReportDistribution,
    /// Operator is the version information of the Flux Operator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<FluxReportOperator>,
    /// ReconcilersStatus is the list of Flux reconcilers and
    /// their statistics grouped by API kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconcilers: Option<Vec<FluxReportReconcilers>>,
    /// SyncStatus is the status of the cluster sync
    /// Source and Kustomization resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<FluxReportSync>,
}

/// Cluster is the version information of the Kubernetes cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportCluster {
    /// Nodes is the number of nodes in the Kubernetes cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i64>,
    /// Platform is the os/arch of the Kubernetes control plane.
    pub platform: String,
    /// ServerVersion is the version of the Kubernetes API server.
    #[serde(rename = "serverVersion")]
    pub server_version: String,
}

/// FluxComponentStatus defines the observed state of a Flux component.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportComponents {
    /// Image is the container image of the Flux component.
    pub image: String,
    /// Name is the name of the Flux component.
    pub name: String,
    /// Ready is the readiness status of the Flux component.
    pub ready: bool,
    /// Status is a human-readable message indicating details
    /// about the Flux component observed state.
    pub status: String,
}

/// Distribution is the version information of the Flux installation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportDistribution {
    /// Entitlement is the entitlement verification status.
    pub entitlement: String,
    /// ManagedBy is the name of the operator managing the Flux instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedBy")]
    pub managed_by: Option<String>,
    /// Status is a human-readable message indicating details
    /// about the distribution observed state.
    pub status: String,
    /// Version is the version of the Flux instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Operator is the version information of the Flux Operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportOperator {
    /// APIVersion is the API version of the Flux Operator.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Platform is the os/arch of Flux Operator.
    pub platform: String,
    /// Version is the version number of Flux Operator.
    pub version: String,
}

/// FluxReconcilerStatus defines the observed state of a Flux reconciler.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportReconcilers {
    /// APIVersion is the API version of the Flux resource.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind is the kind of the Flux resource.
    pub kind: String,
    /// Stats is the reconcile statics of the Flux resource kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<FluxReportReconcilersStats>,
}

/// Stats is the reconcile statics of the Flux resource kind.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportReconcilersStats {
    /// Failing is the number of reconciled
    /// resources in the Failing state.
    pub failing: i64,
    /// Running is the number of reconciled
    /// resources in the Running state.
    pub running: i64,
    /// Suspended is the number of reconciled
    /// resources in the Suspended state.
    pub suspended: i64,
    /// TotalSize is the total size of the artifacts in storage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalSize")]
    pub total_size: Option<String>,
}

/// SyncStatus is the status of the cluster sync
/// Source and Kustomization resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportSync {
    /// ID is the identifier of the sync.
    pub id: String,
    /// Path is the kustomize path of the sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Ready is the readiness status of the sync.
    pub ready: bool,
    /// Source is the URL of the source repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Status is a human-readable message indicating details
    /// about the sync observed state.
    pub status: String,
}

/// FluxReportStatus defines the readiness of a FluxReport.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluxReportStatus {
    /// Conditions contains the readiness conditions of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
}

