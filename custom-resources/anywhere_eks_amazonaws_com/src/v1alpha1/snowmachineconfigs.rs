// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/snowmachineconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// SnowMachineConfigSpec defines the desired state of SnowMachineConfigSpec.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "SnowMachineConfig", plural = "snowmachineconfigs")]
#[kube(namespaced)]
#[kube(status = "SnowMachineConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SnowMachineConfigSpec {
    /// The AMI ID from which to create the machine instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amiID")]
    pub ami_id: Option<String>,
    /// ContainersVolume provides the configuration options for the containers data storage volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containersVolume")]
    pub containers_volume: Option<SnowMachineConfigContainersVolume>,
    /// Devices contains a device ip list assigned by the user to provision machines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    /// HostOSConfiguration provides OS specific configurations for the machine
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostOSConfiguration")]
    pub host_os_configuration: Option<SnowMachineConfigHostOsConfiguration>,
    /// InstanceType is the type of instance to create.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    /// Network provides the custom network setting for the machine.
    pub network: SnowMachineConfigNetwork,
    /// NonRootVolumes provides the configuration options for the non root storage volumes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonRootVolumes")]
    pub non_root_volumes: Option<Vec<SnowMachineConfigNonRootVolumes>>,
    /// OSFamily is the node instance OS.
    /// Valid values: "bottlerocket" and "ubuntu".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "osFamily")]
    pub os_family: Option<String>,
    /// PhysicalNetworkConnector is the physical network connector type to use for creating direct network interfaces (DNI).
    /// Valid values: "SFP_PLUS" (default), "QSFP" and "RJ45".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "physicalNetworkConnector")]
    pub physical_network_connector: Option<String>,
    /// SSHKeyName is the name of the ssh key defined in the aws snow key pairs, to attach to the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKeyName")]
    pub ssh_key_name: Option<String>,
}

/// ContainersVolume provides the configuration options for the containers data storage volume.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigContainersVolume {
    /// Device name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// Size specifies size (in Gi) of the storage device.
    /// Must be greater than the image snapshot size or 8 (whichever is greater).
    pub size: i64,
    /// Type is the type of the volume (sbp1 for capacity-optimized HDD, sbg1 performance-optimized SSD, default is sbp1)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SnowMachineConfigContainersVolumeType>,
}

/// ContainersVolume provides the configuration options for the containers data storage volume.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SnowMachineConfigContainersVolumeType {
    #[serde(rename = "sbp1")]
    Sbp1,
    #[serde(rename = "sbg1")]
    Sbg1,
}

/// HostOSConfiguration provides OS specific configurations for the machine
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfiguration {
    /// BottlerocketConfiguration defines the Bottlerocket configuration on the host OS.
    /// These settings only take effect when the `osFamily` is bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bottlerocketConfiguration")]
    pub bottlerocket_configuration: Option<SnowMachineConfigHostOsConfigurationBottlerocketConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certBundles")]
    pub cert_bundles: Option<Vec<SnowMachineConfigHostOsConfigurationCertBundles>>,
    /// NTPConfiguration defines the NTP configuration on the host OS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ntpConfiguration")]
    pub ntp_configuration: Option<SnowMachineConfigHostOsConfigurationNtpConfiguration>,
}

/// BottlerocketConfiguration defines the Bottlerocket configuration on the host OS.
/// These settings only take effect when the `osFamily` is bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfigurationBottlerocketConfiguration {
    /// Boot defines the boot settings for bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<SnowMachineConfigHostOsConfigurationBottlerocketConfigurationBoot>,
    /// Kernel defines the kernel settings for bottlerocket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel: Option<SnowMachineConfigHostOsConfigurationBottlerocketConfigurationKernel>,
    /// Kubernetes defines the Kubernetes settings on the host OS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<SnowMachineConfigHostOsConfigurationBottlerocketConfigurationKubernetes>,
}

/// Boot defines the boot settings for bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfigurationBottlerocketConfigurationBoot {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootKernelParameters")]
    pub boot_kernel_parameters: Option<BTreeMap<String, String>>,
}

/// Kernel defines the kernel settings for bottlerocket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfigurationBottlerocketConfigurationKernel {
    /// SysctlSettings defines the kernel sysctl settings to set for bottlerocket nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sysctlSettings")]
    pub sysctl_settings: Option<BTreeMap<String, String>>,
}

/// Kubernetes defines the Kubernetes settings on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfigurationBottlerocketConfigurationKubernetes {
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
pub struct SnowMachineConfigHostOsConfigurationCertBundles {
    /// Data defines the cert bundle data.
    pub data: String,
    /// Name defines the cert bundle name.
    pub name: String,
}

/// NTPConfiguration defines the NTP configuration on the host OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigHostOsConfigurationNtpConfiguration {
    /// Servers defines a list of NTP servers to be configured on the host OS.
    pub servers: Vec<String>,
}

/// Network provides the custom network setting for the machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigNetwork {
    /// DirectNetworkInterfaces contains a list of direct network interface (DNI) configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directNetworkInterfaces")]
    pub direct_network_interfaces: Option<Vec<SnowMachineConfigNetworkDirectNetworkInterfaces>>,
}

/// SnowDirectNetworkInterface defines a direct network interface (DNI) configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigNetworkDirectNetworkInterfaces {
    /// DHCP defines whether DHCP is used to assign ip for the DNI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    /// Index is the index number of DNI used to clarify the position in the list. Usually starts with 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// IPPool contains a reference to a snow ip pool which provides a range of ip addresses.
    /// When specified, an ip address selected from the pool is allocated to this DNI.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipPoolRef")]
    pub ip_pool_ref: Option<SnowMachineConfigNetworkDirectNetworkInterfacesIpPoolRef>,
    /// Primary indicates whether the DNI is primary or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// VlanID is the vlan id assigned by the user for the DNI.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vlanID")]
    pub vlan_id: Option<i32>,
}

/// IPPool contains a reference to a snow ip pool which provides a range of ip addresses.
/// When specified, an ip address selected from the pool is allocated to this DNI.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigNetworkDirectNetworkInterfacesIpPoolRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Volume encapsulates the configuration options for the storage device
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigNonRootVolumes {
    /// Device name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// Size specifies size (in Gi) of the storage device.
    /// Must be greater than the image snapshot size or 8 (whichever is greater).
    pub size: i64,
    /// Type is the type of the volume (sbp1 for capacity-optimized HDD, sbg1 performance-optimized SSD, default is sbp1)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SnowMachineConfigNonRootVolumesType>,
}

/// Volume encapsulates the configuration options for the storage device
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SnowMachineConfigNonRootVolumesType {
    #[serde(rename = "sbp1")]
    Sbp1,
    #[serde(rename = "sbg1")]
    Sbg1,
}

/// SnowMachineConfigStatus defines the observed state of SnowMachineConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowMachineConfigStatus {
    /// FailureMessage indicates that there is a fatal problem reconciling the
    /// state, and will be set to a descriptive error message.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// SpecValid is set to true if vspheredatacenterconfig is validated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specValid")]
    pub spec_valid: Option<bool>,
}

