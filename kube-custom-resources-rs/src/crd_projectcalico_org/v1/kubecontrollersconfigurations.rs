// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/kubecontrollersconfigurations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// KubeControllersConfigurationSpec contains the values of the Kubernetes controllers configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "KubeControllersConfiguration", plural = "kubecontrollersconfigurations")]
#[kube(status = "KubeControllersConfigurationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KubeControllersConfigurationSpec {
    /// Controllers enables and configures individual Kubernetes controllers
    pub controllers: KubeControllersConfigurationControllers,
    /// DebugProfilePort configures the port to serve memory and cpu profiles on. If not specified, profiling
    /// is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "debugProfilePort")]
    pub debug_profile_port: Option<i32>,
    /// EtcdV3CompactionPeriod is the period between etcdv3 compaction requests. Set to 0 to disable. [Default: 10m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "etcdV3CompactionPeriod")]
    pub etcd_v3_compaction_period: Option<String>,
    /// HealthChecks enables or disables support for health checks [Default: Enabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthChecks")]
    pub health_checks: Option<String>,
    /// LogSeverityScreen is the log severity above which logs are sent to the stdout. [Default: Info]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logSeverityScreen")]
    pub log_severity_screen: Option<String>,
    /// PrometheusMetricsPort is the TCP port that the Prometheus metrics server should bind to. Set to 0 to disable. [Default: 9094]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prometheusMetricsPort")]
    pub prometheus_metrics_port: Option<i64>,
}

/// Controllers enables and configures individual Kubernetes controllers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllers {
    /// LoadBalancer enables and configures the LoadBalancer controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<KubeControllersConfigurationControllersLoadBalancer>,
    /// Namespace enables and configures the namespace controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<KubeControllersConfigurationControllersNamespace>,
    /// Node enables and configures the node controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<KubeControllersConfigurationControllersNode>,
    /// Policy enables and configures the policy controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<KubeControllersConfigurationControllersPolicy>,
    /// ServiceAccount enables and configures the service account controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<KubeControllersConfigurationControllersServiceAccount>,
    /// WorkloadEndpoint enables and configures the workload endpoint controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadEndpoint")]
    pub workload_endpoint: Option<KubeControllersConfigurationControllersWorkloadEndpoint>,
}

/// LoadBalancer enables and configures the LoadBalancer controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersLoadBalancer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "assignIPs")]
    pub assign_i_ps: Option<String>,
}

/// Namespace enables and configures the namespace controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersNamespace {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// Node enables and configures the node controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersNode {
    /// HostEndpoint controls syncing nodes to host endpoints. Disabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostEndpoint")]
    pub host_endpoint: Option<KubeControllersConfigurationControllersNodeHostEndpoint>,
    /// LeakGracePeriod is the period used by the controller to determine if an IP address has been leaked.
    /// Set to 0 to disable IP garbage collection. [Default: 15m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "leakGracePeriod")]
    pub leak_grace_period: Option<String>,
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
    /// SyncLabels controls whether to copy Kubernetes node labels to Calico nodes. [Default: Enabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncLabels")]
    pub sync_labels: Option<String>,
}

/// HostEndpoint controls syncing nodes to host endpoints. Disabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersNodeHostEndpoint {
    /// AutoCreate enables automatic creation of host endpoints for every node. [Default: Disabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoCreate")]
    pub auto_create: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDefaultHostEndpoint")]
    pub create_default_host_endpoint: Option<String>,
    /// Templates contains definition for creating AutoHostEndpoints
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<KubeControllersConfigurationControllersNodeHostEndpointTemplates>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersNodeHostEndpointTemplates {
    /// GenerateName is appended to the end of the generated AutoHostEndpoint name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateName")]
    pub generate_name: Option<String>,
    /// InterfaceCIDRs contains a list of CIRDs used for matching nodeIPs to the AutoHostEndpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "interfaceCIDRs")]
    pub interface_cid_rs: Option<Vec<String>>,
    /// Labels adds the specified labels to the generated AutoHostEndpoint, labels from node with the same name will be overwritten by values from the template label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// NodeSelector allows the AutoHostEndpoint to be created only for specific nodes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<String>,
}

/// Policy enables and configures the policy controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersPolicy {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// ServiceAccount enables and configures the service account controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersServiceAccount {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// WorkloadEndpoint enables and configures the workload endpoint controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationControllersWorkloadEndpoint {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// KubeControllersConfigurationStatus represents the status of the configuration. It's useful for admins to
/// be able to see the actual config that was applied, which can be modified by environment variables on the
/// kube-controllers process.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatus {
    /// EnvironmentVars contains the environment variables on the kube-controllers that influenced
    /// the RunningConfig.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "environmentVars")]
    pub environment_vars: Option<BTreeMap<String, String>>,
    /// RunningConfig contains the effective config that is running in the kube-controllers pod, after
    /// merging the API resource with any environment variables.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runningConfig")]
    pub running_config: Option<KubeControllersConfigurationStatusRunningConfig>,
}

/// RunningConfig contains the effective config that is running in the kube-controllers pod, after
/// merging the API resource with any environment variables.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfig {
    /// Controllers enables and configures individual Kubernetes controllers
    pub controllers: KubeControllersConfigurationStatusRunningConfigControllers,
    /// DebugProfilePort configures the port to serve memory and cpu profiles on. If not specified, profiling
    /// is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "debugProfilePort")]
    pub debug_profile_port: Option<i32>,
    /// EtcdV3CompactionPeriod is the period between etcdv3 compaction requests. Set to 0 to disable. [Default: 10m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "etcdV3CompactionPeriod")]
    pub etcd_v3_compaction_period: Option<String>,
    /// HealthChecks enables or disables support for health checks [Default: Enabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthChecks")]
    pub health_checks: Option<String>,
    /// LogSeverityScreen is the log severity above which logs are sent to the stdout. [Default: Info]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logSeverityScreen")]
    pub log_severity_screen: Option<String>,
    /// PrometheusMetricsPort is the TCP port that the Prometheus metrics server should bind to. Set to 0 to disable. [Default: 9094]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prometheusMetricsPort")]
    pub prometheus_metrics_port: Option<i64>,
}

/// Controllers enables and configures individual Kubernetes controllers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllers {
    /// LoadBalancer enables and configures the LoadBalancer controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<KubeControllersConfigurationStatusRunningConfigControllersLoadBalancer>,
    /// Namespace enables and configures the namespace controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<KubeControllersConfigurationStatusRunningConfigControllersNamespace>,
    /// Node enables and configures the node controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<KubeControllersConfigurationStatusRunningConfigControllersNode>,
    /// Policy enables and configures the policy controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<KubeControllersConfigurationStatusRunningConfigControllersPolicy>,
    /// ServiceAccount enables and configures the service account controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<KubeControllersConfigurationStatusRunningConfigControllersServiceAccount>,
    /// WorkloadEndpoint enables and configures the workload endpoint controller. Enabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadEndpoint")]
    pub workload_endpoint: Option<KubeControllersConfigurationStatusRunningConfigControllersWorkloadEndpoint>,
}

/// LoadBalancer enables and configures the LoadBalancer controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersLoadBalancer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "assignIPs")]
    pub assign_i_ps: Option<String>,
}

/// Namespace enables and configures the namespace controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersNamespace {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// Node enables and configures the node controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersNode {
    /// HostEndpoint controls syncing nodes to host endpoints. Disabled by default, set to nil to disable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostEndpoint")]
    pub host_endpoint: Option<KubeControllersConfigurationStatusRunningConfigControllersNodeHostEndpoint>,
    /// LeakGracePeriod is the period used by the controller to determine if an IP address has been leaked.
    /// Set to 0 to disable IP garbage collection. [Default: 15m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "leakGracePeriod")]
    pub leak_grace_period: Option<String>,
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
    /// SyncLabels controls whether to copy Kubernetes node labels to Calico nodes. [Default: Enabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncLabels")]
    pub sync_labels: Option<String>,
}

/// HostEndpoint controls syncing nodes to host endpoints. Disabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersNodeHostEndpoint {
    /// AutoCreate enables automatic creation of host endpoints for every node. [Default: Disabled]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoCreate")]
    pub auto_create: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createDefaultHostEndpoint")]
    pub create_default_host_endpoint: Option<String>,
    /// Templates contains definition for creating AutoHostEndpoints
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<KubeControllersConfigurationStatusRunningConfigControllersNodeHostEndpointTemplates>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersNodeHostEndpointTemplates {
    /// GenerateName is appended to the end of the generated AutoHostEndpoint name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateName")]
    pub generate_name: Option<String>,
    /// InterfaceCIDRs contains a list of CIRDs used for matching nodeIPs to the AutoHostEndpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "interfaceCIDRs")]
    pub interface_cid_rs: Option<Vec<String>>,
    /// Labels adds the specified labels to the generated AutoHostEndpoint, labels from node with the same name will be overwritten by values from the template label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// NodeSelector allows the AutoHostEndpoint to be created only for specific nodes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<String>,
}

/// Policy enables and configures the policy controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersPolicy {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// ServiceAccount enables and configures the service account controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersServiceAccount {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

/// WorkloadEndpoint enables and configures the workload endpoint controller. Enabled by default, set to nil to disable.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeControllersConfigurationStatusRunningConfigControllersWorkloadEndpoint {
    /// ReconcilerPeriod is the period to perform reconciliation with the Calico datastore. [Default: 5m]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcilerPeriod")]
    pub reconciler_period: Option<String>,
}

