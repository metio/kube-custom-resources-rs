// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/clusters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterSpec defines the desired state of Cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "Cluster", plural = "clusters")]
#[kube(namespaced)]
#[kube(status = "ClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterSpec {
    /// BundlesRef contains a reference to the Bundles containing the desired dependencies for the cluster. DEPRECATED: Use EksaVersion instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bundlesRef")]
    pub bundles_ref: Option<ClusterBundlesRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNetwork")]
    pub cluster_network: Option<ClusterClusterNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneConfiguration")]
    pub control_plane_configuration: Option<ClusterControlPlaneConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datacenterRef")]
    pub datacenter_ref: Option<ClusterDatacenterRef>,
    /// EksaVersion is the semver identifying the release of eks-a used to populate the cluster components.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eksaVersion")]
    pub eksa_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "etcdEncryption")]
    pub etcd_encryption: Option<Vec<ClusterEtcdEncryption>>,
    /// ExternalEtcdConfiguration defines the configuration options for using unstacked etcd topology.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalEtcdConfiguration")]
    pub external_etcd_configuration: Option<ClusterExternalEtcdConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitOpsRef")]
    pub git_ops_ref: Option<ClusterGitOpsRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityProviderRefs")]
    pub identity_provider_refs: Option<Vec<ClusterIdentityProviderRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesVersion")]
    pub kubernetes_version: Option<String>,
    /// MachineHealthCheck allows to configure timeouts for machine health checks. Machine Health Checks are responsible for remediating unhealthy Machines. Configuring these values will decide how long to wait to remediate unhealthy machine or determine health of nodes' machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineHealthCheck")]
    pub machine_health_check: Option<ClusterMachineHealthCheck>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementCluster")]
    pub management_cluster: Option<ClusterManagementCluster>,
    /// PackageConfiguration for installing EKS Anywhere curated packages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<ClusterPackages>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIamConfig")]
    pub pod_iam_config: Option<ClusterPodIamConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyConfiguration")]
    pub proxy_configuration: Option<ClusterProxyConfiguration>,
    /// RegistryMirrorConfiguration defines the settings for image registry mirror.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryMirrorConfiguration")]
    pub registry_mirror_configuration: Option<ClusterRegistryMirrorConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerNodeGroupConfigurations")]
    pub worker_node_group_configurations: Option<Vec<ClusterWorkerNodeGroupConfigurations>>,
}

/// BundlesRef contains a reference to the Bundles containing the desired dependencies for the cluster. DEPRECATED: Use EksaVersion instead.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBundlesRef {
    /// APIVersion refers to the Bundles APIVersion
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Name refers to the name of the Bundles object in the cluster
    pub name: String,
    /// Namespace refers to the Bundles's namespace
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetwork {
    /// Deprecated. Use CNIConfig
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cni: Option<String>,
    /// CNIConfig specifies the CNI plugin to be installed in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cniConfig")]
    pub cni_config: Option<ClusterClusterNetworkCniConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<ClusterClusterNetworkDns>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<ClusterClusterNetworkNodes>,
    /// Comma-separated list of CIDR blocks to use for pod and service subnets. Defaults to 192.168.0.0/16 for pod subnet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<ClusterClusterNetworkPods>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<ClusterClusterNetworkServices>,
}

/// CNIConfig specifies the CNI plugin to be installed in the cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkCniConfig {
    /// CiliumConfig contains configuration specific to the Cilium CNI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cilium: Option<ClusterClusterNetworkCniConfigCilium>,
    /// KindnetdConfig contains configuration specific to the Kindnetd CNI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kindnetd: Option<ClusterClusterNetworkCniConfigKindnetd>,
}

/// CiliumConfig contains configuration specific to the Cilium CNI.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkCniConfigCilium {
    /// EgressMasquaradeInterfaces determines which network interfaces are used for masquerading. Accepted values are a valid interface name or interface prefix.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressMasqueradeInterfaces")]
    pub egress_masquerade_interfaces: Option<String>,
    /// IPv4NativeRoutingCIDR specifies the CIDR to use when RoutingMode is set to direct. When specified, Cilium assumes networking for this CIDR is preconfigured and hands traffic destined for that range to the Linux network stack without applying any SNAT. If this is not set autoDirectNodeRoutes will be set to true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv4NativeRoutingCIDR")]
    pub ipv4_native_routing_cidr: Option<String>,
    /// IPv6NativeRoutingCIDR specifies the IPv6 CIDR to use when RoutingMode is set to direct. When specified, Cilium assumes networking for this CIDR is preconfigured and hands traffic destined for that range to the Linux network stack without applying any SNAT. If this is not set autoDirectNodeRoutes will be set to true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipv6NativeRoutingCIDR")]
    pub ipv6_native_routing_cidr: Option<String>,
    /// PolicyEnforcementMode determines communication allowed between pods. Accepted values are default, always, never.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyEnforcementMode")]
    pub policy_enforcement_mode: Option<String>,
    /// RoutingMode indicates the routing tunnel mode to use for Cilium. Accepted values are overlay (geneve tunnel with overlay) or direct (tunneling disabled with direct routing) Defaults to overlay.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routingMode")]
    pub routing_mode: Option<String>,
    /// SkipUpgrade indicicates that Cilium maintenance should be skipped during upgrades. This can be used when operators wish to self manage the Cilium installation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipUpgrade")]
    pub skip_upgrade: Option<bool>,
}

/// KindnetdConfig contains configuration specific to the Kindnetd CNI.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkCniConfigKindnetd {
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkDns {
    /// ResolvConf refers to the DNS resolver configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvConf")]
    pub resolv_conf: Option<ClusterClusterNetworkDnsResolvConf>,
}

/// ResolvConf refers to the DNS resolver configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkDnsResolvConf {
    /// Path defines the path to the file that contains the DNS resolver configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkNodes {
    /// CIDRMaskSize defines the mask size for node cidr in the cluster, default for ipv4 is 24. This is an optional field
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrMaskSize")]
    pub cidr_mask_size: Option<i64>,
}

/// Comma-separated list of CIDR blocks to use for pod and service subnets. Defaults to 192.168.0.0/16 for pod subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkPods {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlocks")]
    pub cidr_blocks: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterClusterNetworkServices {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cidrBlocks")]
    pub cidr_blocks: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfiguration {
    /// APIServerExtraArgs defines the flags to configure for the API server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiServerExtraArgs")]
    pub api_server_extra_args: Option<BTreeMap<String, String>>,
    /// CertSANs is a slice of domain names or IPs to be added as Subject Name Alternatives of the Kube API Servers Certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certSans")]
    pub cert_sans: Option<Vec<String>>,
    /// Count defines the number of desired control plane nodes. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Endpoint defines the host ip and port to use for the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<ClusterControlPlaneConfigurationEndpoint>,
    /// Labels define the labels to assign to the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// MachineGroupRef defines the machine group configuration for the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineGroupRef")]
    pub machine_group_ref: Option<ClusterControlPlaneConfigurationMachineGroupRef>,
    /// MachineHealthCheck is a control-plane level override for the timeouts and maxUnhealthy specified in the top-level MHC configuration. If not configured, the defaults in the top-level MHC configuration are used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineHealthCheck")]
    pub machine_health_check: Option<ClusterControlPlaneConfigurationMachineHealthCheck>,
    /// SkipLoadBalancerDeployment skip deploying control plane load balancer. Make sure your infrastructure can handle control plane load balancing when you set this field to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipLoadBalancerDeployment")]
    pub skip_load_balancer_deployment: Option<bool>,
    /// Taints define the set of taints to be applied on control plane nodes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<ClusterControlPlaneConfigurationTaints>>,
    /// UpgradeRolloutStrategy determines the rollout strategy to use for rolling upgrades and related parameters/knobs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeRolloutStrategy")]
    pub upgrade_rollout_strategy: Option<ClusterControlPlaneConfigurationUpgradeRolloutStrategy>,
}

/// Endpoint defines the host ip and port to use for the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationEndpoint {
    /// Host defines the ip that you want to use to connect to the control plane
    pub host: String,
}

/// MachineGroupRef defines the machine group configuration for the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationMachineGroupRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// MachineHealthCheck is a control-plane level override for the timeouts and maxUnhealthy specified in the top-level MHC configuration. If not configured, the defaults in the top-level MHC configuration are used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationMachineHealthCheck {
    /// MaxUnhealthy is used to configure the maximum number of unhealthy machines in machine health checks. This setting applies to both control plane and worker machines. If the number of unhealthy machines exceeds the limit set by maxUnhealthy, further remediation will not be performed. If not configured, the default value is set to "100%" for controlplane machines and "40%" for worker machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnhealthy")]
    pub max_unhealthy: Option<IntOrString>,
    /// NodeStartupTimeout is used to configure the node startup timeout in machine health checks. It determines how long a MachineHealthCheck should wait for a Node to join the cluster, before considering a Machine unhealthy. If not configured, the default value is set to "10m0s" (10 minutes) for all providers. For Tinkerbell provider the default is "20m0s".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStartupTimeout")]
    pub node_startup_timeout: Option<String>,
    /// UnhealthyMachineTimeout is used to configure the unhealthy machine timeout in machine health checks. If any unhealthy conditions are met for the amount of time specified as the timeout, the machines are considered unhealthy. If not configured, the default value is set to "5m0s" (5 minutes).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyMachineTimeout")]
    pub unhealthy_machine_timeout: Option<String>,
}

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationTaints {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// UpgradeRolloutStrategy determines the rollout strategy to use for rolling upgrades and related parameters/knobs
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationUpgradeRolloutStrategy {
    /// ControlPlaneRollingUpdateParams is API for rolling update strategy knobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<ClusterControlPlaneConfigurationUpgradeRolloutStrategyRollingUpdate>,
    /// UpgradeRolloutStrategyType defines the types of upgrade rollout strategies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ControlPlaneRollingUpdateParams is API for rolling update strategy knobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterControlPlaneConfigurationUpgradeRolloutStrategyRollingUpdate {
    #[serde(rename = "maxSurge")]
    pub max_surge: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDatacenterRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// EtcdEncryption defines the configuration for ETCD encryption.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEtcdEncryption {
    pub providers: Vec<ClusterEtcdEncryptionProviders>,
    /// Resources defines a list of objects and custom resources definitions that should be encrypted.
    pub resources: Vec<String>,
}

/// EtcdEncryptionProvider defines the configuration for ETCD encryption providers. Currently only KMS provider is supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEtcdEncryptionProviders {
    /// KMS defines the configuration for KMS Encryption provider.
    pub kms: ClusterEtcdEncryptionProvidersKms,
}

/// KMS defines the configuration for KMS Encryption provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterEtcdEncryptionProvidersKms {
    /// CacheSize defines the maximum number of encrypted objects to be cached in memory. The default value is 1000. You can set this to a negative value to disable caching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cachesize: Option<i32>,
    /// Name defines the name of KMS plugin to be used.
    pub name: String,
    /// SocketListenAddress defines a UNIX socket address that the KMS provider listens on.
    #[serde(rename = "socketListenAddress")]
    pub socket_listen_address: String,
    /// Timeout for kube-apiserver to wait for KMS plugin. Default is 3s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// ExternalEtcdConfiguration defines the configuration options for using unstacked etcd topology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalEtcdConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// MachineGroupRef defines the machine group configuration for the etcd machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineGroupRef")]
    pub machine_group_ref: Option<ClusterExternalEtcdConfigurationMachineGroupRef>,
}

/// MachineGroupRef defines the machine group configuration for the etcd machines.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterExternalEtcdConfigurationMachineGroupRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterGitOpsRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterIdentityProviderRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// MachineHealthCheck allows to configure timeouts for machine health checks. Machine Health Checks are responsible for remediating unhealthy Machines. Configuring these values will decide how long to wait to remediate unhealthy machine or determine health of nodes' machines.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterMachineHealthCheck {
    /// MaxUnhealthy is used to configure the maximum number of unhealthy machines in machine health checks. This setting applies to both control plane and worker machines. If the number of unhealthy machines exceeds the limit set by maxUnhealthy, further remediation will not be performed. If not configured, the default value is set to "100%" for controlplane machines and "40%" for worker machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnhealthy")]
    pub max_unhealthy: Option<IntOrString>,
    /// NodeStartupTimeout is used to configure the node startup timeout in machine health checks. It determines how long a MachineHealthCheck should wait for a Node to join the cluster, before considering a Machine unhealthy. If not configured, the default value is set to "10m0s" (10 minutes) for all providers. For Tinkerbell provider the default is "20m0s".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStartupTimeout")]
    pub node_startup_timeout: Option<String>,
    /// UnhealthyMachineTimeout is used to configure the unhealthy machine timeout in machine health checks. If any unhealthy conditions are met for the amount of time specified as the timeout, the machines are considered unhealthy. If not configured, the default value is set to "5m0s" (5 minutes).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyMachineTimeout")]
    pub unhealthy_machine_timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterManagementCluster {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PackageConfiguration for installing EKS Anywhere curated packages.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackages {
    /// Controller package controller configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<ClusterPackagesController>,
    /// Cronjob for ecr token refresher
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cronjob: Option<ClusterPackagesCronjob>,
    /// Disable package controller on cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

/// Controller package controller configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackagesController {
    /// Digest package controller digest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// DisableWebhooks on package controller
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableWebhooks")]
    pub disable_webhooks: Option<bool>,
    /// Env of package controller in the format `key=value`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Repository package controller repository
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Resources of package controller
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ClusterPackagesControllerResources>,
    /// Tag package controller tag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Resources of package controller
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackagesControllerResources {
    /// ImageResource resources for container image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ClusterPackagesControllerResourcesLimits>,
    /// Requests for image resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<ClusterPackagesControllerResourcesRequests>,
}

/// ImageResource resources for container image.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackagesControllerResourcesLimits {
    /// CPU image cpu
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// Memory image memory
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

/// Requests for image resources
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackagesControllerResourcesRequests {
    /// CPU image cpu
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// Memory image memory
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

/// Cronjob for ecr token refresher
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPackagesCronjob {
    /// Digest ecr token refresher digest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Disable on cron job
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// Repository ecr token refresher repository
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    /// Tag ecr token refresher tag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPodIamConfig {
    #[serde(rename = "serviceAccountIssuer")]
    pub service_account_issuer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterProxyConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpProxy")]
    pub http_proxy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsProxy")]
    pub https_proxy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noProxy")]
    pub no_proxy: Option<Vec<String>>,
}

/// RegistryMirrorConfiguration defines the settings for image registry mirror.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRegistryMirrorConfiguration {
    /// Authenticate defines if registry requires authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// CACertContent defines the contents registry mirror CA certificate
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertContent")]
    pub ca_cert_content: Option<String>,
    /// Endpoint defines the registry mirror endpoint to use for pulling images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// InsecureSkipVerify skips the registry certificate verification. Only use this solution for isolated testing or in a tightly controlled, air-gapped environment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipVerify")]
    pub insecure_skip_verify: Option<bool>,
    /// OCINamespaces defines the mapping from an upstream registry to a local namespace where upstream artifacts are placed into
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ociNamespaces")]
    pub oci_namespaces: Option<Vec<ClusterRegistryMirrorConfigurationOciNamespaces>>,
    /// Port defines the port exposed for registry mirror endpoint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

/// OCINamespace represents an entity in a local reigstry to group related images.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRegistryMirrorConfigurationOciNamespaces {
    /// Namespace refers to the name of a namespace in the local registry
    pub namespace: String,
    /// Registry refers to the name of the upstream registry
    pub registry: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurations {
    /// AutoScalingConfiguration defines the auto scaling configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoscalingConfiguration")]
    pub autoscaling_configuration: Option<ClusterWorkerNodeGroupConfigurationsAutoscalingConfiguration>,
    /// Count defines the number of desired worker nodes. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// KuberenetesVersion defines the version for worker nodes. If not set, the top level spec kubernetesVersion will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesVersion")]
    pub kubernetes_version: Option<String>,
    /// Labels define the labels to assign to the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// MachineGroupRef defines the machine group configuration for the worker nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineGroupRef")]
    pub machine_group_ref: Option<ClusterWorkerNodeGroupConfigurationsMachineGroupRef>,
    /// MachineHealthCheck is a worker node level override for the timeouts and maxUnhealthy specified in the top-level MHC configuration. If not configured, the defaults in the top-level MHC configuration are used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineHealthCheck")]
    pub machine_health_check: Option<ClusterWorkerNodeGroupConfigurationsMachineHealthCheck>,
    /// Name refers to the name of the worker node group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Taints define the set of taints to be applied on worker nodes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<ClusterWorkerNodeGroupConfigurationsTaints>>,
    /// UpgradeRolloutStrategy determines the rollout strategy to use for rolling upgrades and related parameters/knobs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeRolloutStrategy")]
    pub upgrade_rollout_strategy: Option<ClusterWorkerNodeGroupConfigurationsUpgradeRolloutStrategy>,
}

/// AutoScalingConfiguration defines the auto scaling configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsAutoscalingConfiguration {
    /// MaxCount defines the maximum number of nodes for the associated resource group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxCount")]
    pub max_count: Option<i64>,
    /// MinCount defines the minimum number of nodes for the associated resource group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minCount")]
    pub min_count: Option<i64>,
}

/// MachineGroupRef defines the machine group configuration for the worker nodes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsMachineGroupRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// MachineHealthCheck is a worker node level override for the timeouts and maxUnhealthy specified in the top-level MHC configuration. If not configured, the defaults in the top-level MHC configuration are used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsMachineHealthCheck {
    /// MaxUnhealthy is used to configure the maximum number of unhealthy machines in machine health checks. This setting applies to both control plane and worker machines. If the number of unhealthy machines exceeds the limit set by maxUnhealthy, further remediation will not be performed. If not configured, the default value is set to "100%" for controlplane machines and "40%" for worker machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnhealthy")]
    pub max_unhealthy: Option<IntOrString>,
    /// NodeStartupTimeout is used to configure the node startup timeout in machine health checks. It determines how long a MachineHealthCheck should wait for a Node to join the cluster, before considering a Machine unhealthy. If not configured, the default value is set to "10m0s" (10 minutes) for all providers. For Tinkerbell provider the default is "20m0s".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeStartupTimeout")]
    pub node_startup_timeout: Option<String>,
    /// UnhealthyMachineTimeout is used to configure the unhealthy machine timeout in machine health checks. If any unhealthy conditions are met for the amount of time specified as the timeout, the machines are considered unhealthy. If not configured, the default value is set to "5m0s" (5 minutes).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyMachineTimeout")]
    pub unhealthy_machine_timeout: Option<String>,
}

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsTaints {
    /// Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// UpgradeRolloutStrategy determines the rollout strategy to use for rolling upgrades and related parameters/knobs
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsUpgradeRolloutStrategy {
    /// WorkerNodesRollingUpdateParams is API for rolling update strategy knobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<ClusterWorkerNodeGroupConfigurationsUpgradeRolloutStrategyRollingUpdate>,
    /// UpgradeRolloutStrategyType defines the types of upgrade rollout strategies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// WorkerNodesRollingUpdateParams is API for rolling update strategy knobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterWorkerNodeGroupConfigurationsUpgradeRolloutStrategyRollingUpdate {
    #[serde(rename = "maxSurge")]
    pub max_surge: i64,
    #[serde(rename = "maxUnavailable")]
    pub max_unavailable: i64,
}

/// ClusterStatus defines the observed state of Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatus {
    /// ChildrenReconciledGeneration represents the sum of the .metadata.generation for all the linked objects for the cluster, observed the last time the cluster was successfully reconciled. NOTE: This field was added for internal use and we do not provide guarantees to its behavior if changed externally. Its meaning and implementation are subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "childrenReconciledGeneration")]
    pub children_reconciled_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// EksdReleaseRef defines the properties of the EKS-D object on the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eksdReleaseRef")]
    pub eksd_release_ref: Option<ClusterStatusEksdReleaseRef>,
    /// Descriptive message about a fatal problem while reconciling a cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// Machine readable value about a terminal problem while reconciling the cluster set at the same time as failureMessage
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ReconciledGeneration represents the .metadata.generation the last time the cluster was successfully reconciled. It is the latest generation observed by the controller. NOTE: This field was added for internal use and we do not provide guarantees to its behavior if changed externally. Its meaning and implementation are subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconciledGeneration")]
    pub reconciled_generation: Option<i64>,
}

/// EksdReleaseRef defines the properties of the EKS-D object on the cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatusEksdReleaseRef {
    /// ApiVersion refers to the EKS-D API version
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind refers to the Release kind for the EKS-D object
    pub kind: String,
    /// Name refers to the name of the EKS-D object on the cluster
    pub name: String,
    /// Namespace refers to the namespace for the EKS-D release resources
    pub namespace: String,
}

