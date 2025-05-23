// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/tinkerbellmachineconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// TinkerbellMachineConfigSpec defines the desired state of TinkerbellMachineConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "TinkerbellMachineConfig", plural = "tinkerbellmachineconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TinkerbellMachineConfigSpec {
    /// HardwareSelector models a simple key-value selector used in Tinkerbell provisioning.
    #[serde(rename = "hardwareSelector")]
    pub hardware_selector: BTreeMap<String, String>,
    /// HostOSConfiguration defines the configuration settings on the host OS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostOSConfiguration")]
    pub host_os_configuration: Option<TinkerbellMachineConfigHostOsConfiguration>,
    #[serde(rename = "osFamily")]
    pub os_family: String,
    /// OSImageURL can be used to override the default OS image path to pull from a local server.
    /// OSImageURL is a URL to the OS image used during provisioning. It must include
    /// the Kubernetes version(s). For example, a URL used for Kubernetes 1.27 could
    /// be http://localhost:8080/ubuntu-2204-1.27.tgz
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "osImageURL")]
    pub os_image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateRef")]
    pub template_ref: Option<TinkerbellMachineConfigTemplateRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TinkerbellMachineConfigUsers>>,
}

/// HostOSConfiguration defines the configuration settings on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfiguration {
    /// BottlerocketConfiguration defines the Bottlerocket configuration on the host OS.
    /// These settings only take effect when the `osFamily` is bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bottlerocketConfiguration")]
    pub bottlerocket_configuration: Option<TinkerbellMachineConfigHostOsConfigurationBottlerocketConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certBundles")]
    pub cert_bundles: Option<Vec<TinkerbellMachineConfigHostOsConfigurationCertBundles>>,
    /// NTPConfiguration defines the NTP configuration on the host OS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ntpConfiguration")]
    pub ntp_configuration: Option<TinkerbellMachineConfigHostOsConfigurationNtpConfiguration>,
}

/// BottlerocketConfiguration defines the Bottlerocket configuration on the host OS.
/// These settings only take effect when the `osFamily` is bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationBottlerocketConfiguration {
    /// Boot defines the boot settings for bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationBoot>,
    /// Kernel defines the kernel settings for bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel: Option<TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationKernel>,
    /// Kubernetes defines the Kubernetes settings on the host OS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationKubernetes>,
}

/// Boot defines the boot settings for bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationBoot {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootKernelParameters")]
    pub boot_kernel_parameters: Option<BTreeMap<String, String>>,
}

/// Kernel defines the kernel settings for bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationKernel {
    /// SysctlSettings defines the kernel sysctl settings to set for bottlerocket nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sysctlSettings")]
    pub sysctl_settings: Option<BTreeMap<String, String>>,
}

/// Kubernetes defines the Kubernetes settings on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationBottlerocketConfigurationKubernetes {
    /// AllowedUnsafeSysctls defines the list of unsafe sysctls that can be set on a node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedUnsafeSysctls")]
    pub allowed_unsafe_sysctls: Option<Vec<String>>,
    /// ClusterDNSIPs defines IP addresses of the DNS servers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDNSIPs")]
    pub cluster_dnsi_ps: Option<Vec<String>>,
    /// ClusterDomain defines the DNS domain for the cluster, allowing all Kubernetes-run containers
    /// to search this domain before the host’s search domains
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDomain")]
    pub cluster_domain: Option<String>,
    /// ContainerLogMaxFiles specifies the maximum number of container log
    /// files that can be present for a container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerLogMaxFiles")]
    pub container_log_max_files: Option<i64>,
    /// ContainerLogMaxSize is a quantity defining the maximum size of
    /// the container log file before it is rotated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerLogMaxSize")]
    pub container_log_max_size: Option<String>,
    /// CPUCFSQuota enables CPU CFS quota enforcement for containers that specify CPU limits
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuCFSQuota")]
    pub cpu_cfs_quota: Option<bool>,
    /// CPUManagerPolicy is the name of the policy to use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuManagerPolicy")]
    pub cpu_manager_policy: Option<String>,
    /// CPUManagerPolicyOptions is a set of key=value which allows to set extra options to
    /// fine tune the behaviour of the cpu manager policies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuManagerPolicyOptions")]
    pub cpu_manager_policy_options: Option<BTreeMap<String, String>>,
    /// CPUManagerReconcilePeriod is the reconciliation period for the CPU Manager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuManagerReconcilePeriod")]
    pub cpu_manager_reconcile_period: Option<String>,
    /// EventBurst is the maximum size of a burst of event creations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventBurst")]
    pub event_burst: Option<i64>,
    /// EventRecordQPS is the maximum event creations per second.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventRecordQPS")]
    pub event_record_qps: Option<i64>,
    /// EvictionHard is a map of signal names to quantities that defines hard eviction thresholds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionHard")]
    pub eviction_hard: Option<BTreeMap<String, String>>,
    /// EvictionMaxPodGracePeriod is the maximum allowed grace period (in seconds) to use
    /// when terminating pods in response to a soft eviction threshold being met.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionMaxPodGracePeriod")]
    pub eviction_max_pod_grace_period: Option<i64>,
    /// EvictionSoft is a map of signal names to quantities that defines soft eviction thresholds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoft")]
    pub eviction_soft: Option<BTreeMap<String, String>>,
    /// EvictionSoftGracePeriod is a map of signal names to quantities that defines grace periods
    /// for each soft eviction signal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionSoftGracePeriod")]
    pub eviction_soft_grace_period: Option<BTreeMap<String, String>>,
    /// ImageGCHighThresholdPercent is the percent of disk usage after which image garbage
    /// collection is always run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCHighThresholdPercent")]
    pub image_gc_high_threshold_percent: Option<i64>,
    /// ImageGCLowThresholdPercent is the percent of disk usage before which image garbage collection is never run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageGCLowThresholdPercent")]
    pub image_gc_low_threshold_percent: Option<i64>,
    /// KubeAPIBurst  is the burst to allow while talking with kubernetes API server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIBurst")]
    pub kube_api_burst: Option<i64>,
    /// KubeAPIQPS is the QPS to use while talking with kubernetes apiserver.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeAPIQPS")]
    pub kube_apiqps: Option<i64>,
    /// KubeReserved is a set of ResourceName=ResourceQuantity pairs that describe resources
    /// reserved for kubernetes system components
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeReserved")]
    pub kube_reserved: Option<BTreeMap<String, String>>,
    /// MaxPods defines the maximum number of pods that can run on a node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPods")]
    pub max_pods: Option<i64>,
    /// MemoryManagerPolicy is the name of the policy to use by memory manager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryManagerPolicy")]
    pub memory_manager_policy: Option<String>,
    /// PodPidsLimit is the maximum number of PIDs in any pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podPidsLimit")]
    pub pod_pids_limit: Option<i64>,
    /// ProviderID sets the unique ID of the instance that an external provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// RegistryBurst is the maximum size of bursty pulls.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryBurst")]
    pub registry_burst: Option<i64>,
    /// RegistryPullQPS is the limit of registry pulls per second.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryPullQPS")]
    pub registry_pull_qps: Option<i64>,
    /// ShutdownGracePeriod specifies the total duration that the node should delay
    /// the shutdown and total grace period for pod termination during a node shutdown.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shutdownGracePeriod")]
    pub shutdown_grace_period: Option<String>,
    /// ShutdownGracePeriodCriticalPods specifies the duration used to terminate
    /// critical pods during a node shutdown.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shutdownGracePeriodCriticalPods")]
    pub shutdown_grace_period_critical_pods: Option<String>,
    /// SystemReserved is a set of ResourceName=ResourceQuantity pairs that describe
    /// resources reserved for non-kubernetes components.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemReserved")]
    pub system_reserved: Option<BTreeMap<String, String>>,
    /// TopologyManagerPolicy is the name of the topology manager policy to use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyManagerPolicy")]
    pub topology_manager_policy: Option<String>,
    /// TopologyManagerScope represents the scope of topology hint generation
    /// that topology manager requests and hint providers generate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyManagerScope")]
    pub topology_manager_scope: Option<String>,
}

/// Cert defines additional trusted cert bundles on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationCertBundles {
    /// Data defines the cert bundle data.
    pub data: String,
    /// Name defines the cert bundle name.
    pub name: String,
}

/// NTPConfiguration defines the NTP configuration on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigHostOsConfigurationNtpConfiguration {
    /// Servers defines a list of NTP servers to be configured on the host OS.
    pub servers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigTemplateRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// UserConfiguration defines the configuration of the user to be added to the VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigUsers {
    pub name: String,
    #[serde(rename = "sshAuthorizedKeys")]
    pub ssh_authorized_keys: Vec<String>,
}

/// TinkerbellMachineConfigStatus defines the observed state of TinkerbellMachineConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineConfigStatus {
}

