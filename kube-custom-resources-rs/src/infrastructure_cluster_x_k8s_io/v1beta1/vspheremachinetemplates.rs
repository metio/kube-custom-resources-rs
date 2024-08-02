// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1beta1/vspheremachinetemplates.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// VSphereMachineTemplateSpec defines the desired state of VSphereMachineTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "VSphereMachineTemplate", plural = "vspheremachinetemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereMachineTemplateSpec {
    /// VSphereMachineTemplateResource describes the data needed to create a VSphereMachine from a template.
    pub template: VSphereMachineTemplateTemplate,
}

/// VSphereMachineTemplateResource describes the data needed to create a VSphereMachine from a template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<VSphereMachineTemplateTemplateMetadata>,
    /// Spec is the specification of the desired behavior of the machine.
    pub spec: VSphereMachineTemplateTemplateSpec,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Spec is the specification of the desired behavior of the machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpec {
    /// AdditionalDisksGiB holds the sizes of additional disks of the virtual machine, in GiB
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalDisksGiB")]
    pub additional_disks_gi_b: Option<Vec<i64>>,
    /// CloneMode specifies the type of clone operation.
    /// The LinkedClone mode is only support for templates that have at least
    /// one snapshot. If the template has no snapshots, then CloneMode defaults
    /// to FullClone.
    /// When LinkedClone mode is enabled the DiskGiB field is ignored as it is
    /// not possible to expand disks of linked clones.
    /// Defaults to LinkedClone, but fails gracefully to FullClone if the source
    /// of the clone operation has no snapshots.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneMode")]
    pub clone_mode: Option<String>,
    /// CustomVMXKeys is a dictionary of advanced VMX options that can be set on VM
    /// Defaults to empty map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customVMXKeys")]
    pub custom_vmx_keys: Option<BTreeMap<String, String>>,
    /// Datacenter is the name or inventory path of the datacenter in which the
    /// virtual machine is created/located.
    /// Defaults to * which selects the default datacenter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// Datastore is the name or inventory path of the datastore in which the
    /// virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,
    /// DiskGiB is the size of a virtual machine's disk, in GiB.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskGiB")]
    pub disk_gi_b: Option<i32>,
    /// FailureDomain is the failure domain unique identifier this Machine should be attached to, as defined in Cluster API.
    /// For this infrastructure provider, the name is equivalent to the name of the VSphereDeploymentZone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// Folder is the name or inventory path of the folder in which the
    /// virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// GuestSoftPowerOffTimeout sets the wait timeout for shutdown in the VM guest.
    /// The VM will be powered off forcibly after the timeout if the VM is still
    /// up and running when the PowerOffMode is set to trySoft.
    /// 
    /// 
    /// This parameter only applies when the PowerOffMode is set to trySoft.
    /// 
    /// 
    /// If omitted, the timeout defaults to 5 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "guestSoftPowerOffTimeout")]
    pub guest_soft_power_off_timeout: Option<String>,
    /// HardwareVersion is the hardware version of the virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Check the compatibility with the ESXi version before setting the value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hardwareVersion")]
    pub hardware_version: Option<String>,
    /// MemoryMiB is the size of a virtual machine's memory, in MiB.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryMiB")]
    pub memory_mi_b: Option<i64>,
    /// Network is the network configuration for this machine's VM.
    pub network: VSphereMachineTemplateTemplateSpecNetwork,
    /// NumCPUs is the number of virtual processors in a virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numCPUs")]
    pub num_cp_us: Option<i32>,
    /// NumCPUs is the number of cores among which to distribute CPUs in this
    /// virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numCoresPerSocket")]
    pub num_cores_per_socket: Option<i32>,
    /// OS is the Operating System of the virtual machine
    /// Defaults to Linux
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// PciDevices is the list of pci devices used by the virtual machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pciDevices")]
    pub pci_devices: Option<Vec<VSphereMachineTemplateTemplateSpecPciDevices>>,
    /// PowerOffMode describes the desired behavior when powering off a VM.
    /// 
    /// 
    /// There are three, supported power off modes: hard, soft, and
    /// trySoft. The first mode, hard, is the equivalent of a physical
    /// system's power cord being ripped from the wall. The soft mode
    /// requires the VM's guest to have VM Tools installed and attempts to
    /// gracefully shut down the VM. Its variant, trySoft, first attempts
    /// a graceful shutdown, and if that fails or the VM is not in a powered off
    /// state after reaching the GuestSoftPowerOffTimeout, the VM is halted.
    /// 
    /// 
    /// If omitted, the mode defaults to hard.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "powerOffMode")]
    pub power_off_mode: Option<VSphereMachineTemplateTemplateSpecPowerOffMode>,
    /// ProviderID is the virtual machine's BIOS UUID formated as
    /// vsphere://12345678-1234-1234-1234-123456789abc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// ResourcePool is the name or inventory path of the resource pool in which
    /// the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
    /// Server is the IP address or FQDN of the vSphere server on which
    /// the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Snapshot is the name of the snapshot from which to create a linked clone.
    /// This field is ignored if LinkedClone is not enabled.
    /// Defaults to the source's current snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    /// StoragePolicyName of the storage policy to use with this
    /// Virtual Machine
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storagePolicyName")]
    pub storage_policy_name: Option<String>,
    /// TagIDs is an optional set of tags to add to an instance. Specified tagIDs
    /// must use URN-notation instead of display names.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagIDs")]
    pub tag_i_ds: Option<Vec<String>>,
    /// Template is the name or inventory path of the template used to clone
    /// the virtual machine.
    pub template: String,
    /// Thumbprint is the colon-separated SHA-1 checksum of the given vCenter server's host certificate
    /// When this is set to empty, this VirtualMachine would be created
    /// without TLS certificate validation of the communication between Cluster API Provider vSphere
    /// and the VMware vCenter server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

/// Network is the network configuration for this machine's VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetwork {
    /// Devices is the list of network devices used by the virtual machine.
    /// TODO(akutz) Make sure at least one network matches the
    ///             ClusterSpec.CloudProviderConfiguration.Network.Name
    pub devices: Vec<VSphereMachineTemplateTemplateSpecNetworkDevices>,
    /// PreferredAPIServeCIDR is the preferred CIDR for the Kubernetes API
    /// server endpoint on this machine
    /// 
    /// 
    /// Deprecated: This field is going to be removed in a future release.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredAPIServerCidr")]
    pub preferred_api_server_cidr: Option<String>,
    /// Routes is a list of optional, static routes applied to the virtual
    /// machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<VSphereMachineTemplateTemplateSpecNetworkRoutes>>,
}

/// NetworkDeviceSpec defines the network configuration for a virtual machine's
/// network device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkDevices {
    /// AddressesFromPools is a list of IPAddressPools that should be assigned
    /// to IPAddressClaims. The machine's cloud-init metadata will be populated
    /// with IPAddresses fulfilled by an IPAM provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressesFromPools")]
    pub addresses_from_pools: Option<Vec<VSphereMachineTemplateTemplateSpecNetworkDevicesAddressesFromPools>>,
    /// DeviceName may be used to explicitly assign a name to the network device
    /// as it exists in the guest operating system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// DHCP4 is a flag that indicates whether or not to use DHCP for IPv4
    /// on this device.
    /// If true then IPAddrs should not contain any IPv4 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp4: Option<bool>,
    /// DHCP4Overrides allows for the control over several DHCP behaviors.
    /// Overrides will only be applied when the corresponding DHCP flag is set.
    /// Only configured values will be sent, omitted values will default to
    /// distribution defaults.
    /// Dependent on support in the network stack for your distribution.
    /// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcp4Overrides")]
    pub dhcp4_overrides: Option<VSphereMachineTemplateTemplateSpecNetworkDevicesDhcp4Overrides>,
    /// DHCP6 is a flag that indicates whether or not to use DHCP for IPv6
    /// on this device.
    /// If true then IPAddrs should not contain any IPv6 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp6: Option<bool>,
    /// DHCP6Overrides allows for the control over several DHCP behaviors.
    /// Overrides will only be applied when the corresponding DHCP flag is set.
    /// Only configured values will be sent, omitted values will default to
    /// distribution defaults.
    /// Dependent on support in the network stack for your distribution.
    /// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcp6Overrides")]
    pub dhcp6_overrides: Option<VSphereMachineTemplateTemplateSpecNetworkDevicesDhcp6Overrides>,
    /// Gateway4 is the IPv4 gateway used by this device.
    /// Required when DHCP4 is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway4: Option<String>,
    /// Gateway4 is the IPv4 gateway used by this device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway6: Option<String>,
    /// IPAddrs is a list of one or more IPv4 and/or IPv6 addresses to assign
    /// to this device. IP addresses must also specify the segment length in
    /// CIDR notation.
    /// Required when DHCP4, DHCP6 and SkipIPAllocation are false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddrs")]
    pub ip_addrs: Option<Vec<String>>,
    /// MACAddr is the MAC address used by this device.
    /// It is generally a good idea to omit this field and allow a MAC address
    /// to be generated.
    /// Please note that this value must use the VMware OUI to work with the
    /// in-tree vSphere cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "macAddr")]
    pub mac_addr: Option<String>,
    /// MTU is the device’s Maximum Transmission Unit size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// Nameservers is a list of IPv4 and/or IPv6 addresses used as DNS
    /// nameservers.
    /// Please note that Linux allows only three nameservers (https://linux.die.net/man/5/resolv.conf).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// NetworkName is the name of the vSphere network to which the device
    /// will be connected.
    #[serde(rename = "networkName")]
    pub network_name: String,
    /// Routes is a list of optional, static routes applied to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<VSphereMachineTemplateTemplateSpecNetworkDevicesRoutes>>,
    /// SearchDomains is a list of search domains used when resolving IP
    /// addresses with DNS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "searchDomains")]
    pub search_domains: Option<Vec<String>>,
    /// SkipIPAllocation allows the device to not have IP address or DHCP configured.
    /// This is suitable for devices for which IP allocation is handled externally, eg. using Multus CNI.
    /// If true, CAPV will not verify IP address allocation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipIPAllocation")]
    pub skip_ip_allocation: Option<bool>,
}

/// TypedLocalObjectReference contains enough information to let you locate the
/// typed referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkDevicesAddressesFromPools {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// DHCP4Overrides allows for the control over several DHCP behaviors.
/// Overrides will only be applied when the corresponding DHCP flag is set.
/// Only configured values will be sent, omitted values will default to
/// distribution defaults.
/// Dependent on support in the network stack for your distribution.
/// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkDevicesDhcp4Overrides {
    /// Hostname is the name which will be sent to the DHCP server instead of
    /// the machine's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// RouteMetric is used to prioritize routes for devices. A lower metric for
    /// an interface will have a higher priority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeMetric")]
    pub route_metric: Option<i64>,
    /// SendHostname when `true`, the hostname of the machine will be sent to the
    /// DHCP server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendHostname")]
    pub send_hostname: Option<bool>,
    /// UseDNS when `true`, the DNS servers in the DHCP server will be used and
    /// take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDNS")]
    pub use_dns: Option<bool>,
    /// UseDomains can take the values `true`, `false`, or `route`. When `true`,
    /// the domain name from the DHCP server will be used as the DNS search
    /// domain for this device. When `route`, the domain name from the DHCP
    /// response will be used for routing DNS only, not for searching.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDomains")]
    pub use_domains: Option<String>,
    /// UseHostname when `true`, the hostname from the DHCP server will be set
    /// as the transient hostname of the machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useHostname")]
    pub use_hostname: Option<bool>,
    /// UseMTU when `true`, the MTU from the DHCP server will be set as the
    /// MTU of the device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useMTU")]
    pub use_mtu: Option<bool>,
    /// UseNTP when `true`, the NTP servers from the DHCP server will be used
    /// by systemd-timesyncd and take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useNTP")]
    pub use_ntp: Option<bool>,
    /// UseRoutes when `true`, the routes from the DHCP server will be installed
    /// in the routing table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRoutes")]
    pub use_routes: Option<String>,
}

/// DHCP6Overrides allows for the control over several DHCP behaviors.
/// Overrides will only be applied when the corresponding DHCP flag is set.
/// Only configured values will be sent, omitted values will default to
/// distribution defaults.
/// Dependent on support in the network stack for your distribution.
/// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkDevicesDhcp6Overrides {
    /// Hostname is the name which will be sent to the DHCP server instead of
    /// the machine's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// RouteMetric is used to prioritize routes for devices. A lower metric for
    /// an interface will have a higher priority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeMetric")]
    pub route_metric: Option<i64>,
    /// SendHostname when `true`, the hostname of the machine will be sent to the
    /// DHCP server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendHostname")]
    pub send_hostname: Option<bool>,
    /// UseDNS when `true`, the DNS servers in the DHCP server will be used and
    /// take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDNS")]
    pub use_dns: Option<bool>,
    /// UseDomains can take the values `true`, `false`, or `route`. When `true`,
    /// the domain name from the DHCP server will be used as the DNS search
    /// domain for this device. When `route`, the domain name from the DHCP
    /// response will be used for routing DNS only, not for searching.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDomains")]
    pub use_domains: Option<String>,
    /// UseHostname when `true`, the hostname from the DHCP server will be set
    /// as the transient hostname of the machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useHostname")]
    pub use_hostname: Option<bool>,
    /// UseMTU when `true`, the MTU from the DHCP server will be set as the
    /// MTU of the device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useMTU")]
    pub use_mtu: Option<bool>,
    /// UseNTP when `true`, the NTP servers from the DHCP server will be used
    /// by systemd-timesyncd and take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useNTP")]
    pub use_ntp: Option<bool>,
    /// UseRoutes when `true`, the routes from the DHCP server will be installed
    /// in the routing table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRoutes")]
    pub use_routes: Option<String>,
}

/// NetworkRouteSpec defines a static network route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkDevicesRoutes {
    /// Metric is the weight/priority of the route.
    pub metric: i32,
    /// To is an IPv4 or IPv6 address.
    pub to: String,
    /// Via is an IPv4 or IPv6 address.
    pub via: String,
}

/// NetworkRouteSpec defines a static network route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecNetworkRoutes {
    /// Metric is the weight/priority of the route.
    pub metric: i32,
    /// To is an IPv4 or IPv6 address.
    pub to: String,
    /// Via is an IPv4 or IPv6 address.
    pub via: String,
}

/// PCIDeviceSpec defines virtual machine's PCI configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereMachineTemplateTemplateSpecPciDevices {
    /// CustomLabel is the hardware label of a virtual machine's PCI device.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customLabel")]
    pub custom_label: Option<String>,
    /// DeviceID is the device ID of a virtual machine's PCI, in integer.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with VGPUProfile as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceId")]
    pub device_id: Option<i32>,
    /// VGPUProfile is the profile name of a virtual machine's vGPU, in string.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with DeviceID and VendorID as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vGPUProfile")]
    pub v_gpu_profile: Option<String>,
    /// VendorId is the vendor ID of a virtual machine's PCI, in integer.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with VGPUProfile as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vendorId")]
    pub vendor_id: Option<i32>,
}

/// Spec is the specification of the desired behavior of the machine.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereMachineTemplateTemplateSpecPowerOffMode {
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "soft")]
    Soft,
    #[serde(rename = "trySoft")]
    TrySoft,
}

