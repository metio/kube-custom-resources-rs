// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta2/ibmvpcmachines.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// IBMVPCMachineSpec defines the desired state of IBMVPCMachine.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta2", kind = "IBMVPCMachine", plural = "ibmvpcmachines")]
#[kube(namespaced)]
#[kube(status = "IBMVPCMachineStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMVPCMachineSpec {
    /// BootVolume contains machines's boot volume configurations like size, iops etc..
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootVolume")]
    pub boot_volume: Option<IBMVPCMachineBootVolume>,
    /// CatalogOffering is the Catalog Offering OS image which would be installed on the instance.
    /// An OfferingCRN or VersionCRN is required, the PlanCRN is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "catalogOffering")]
    pub catalog_offering: Option<IBMVPCMachineCatalogOffering>,
    /// Image is the OS image which would be install on the instance.
    /// ID will take higher precedence over Name if both specified.
    pub image: IBMVPCMachineImage,
    /// LoadBalancerPoolMembers is the set of IBM Cloud VPC Load Balancer Backend Pools the machine should be added to as a member.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerPoolMembers")]
    pub load_balancer_pool_members: Option<Vec<IBMVPCMachineLoadBalancerPoolMembers>>,
    /// Name of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// PlacementTarget is the placement restrictions to use for the virtual server instance. No restrictions are used when this field is not defined.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "placementTarget")]
    pub placement_target: Option<IBMVPCMachinePlacementTarget>,
    /// PrimaryNetworkInterface is required to specify subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryNetworkInterface")]
    pub primary_network_interface: Option<IBMVPCMachinePrimaryNetworkInterface>,
    /// Profile indicates the flavor of instance. Example: bx2-8x32	means 8 vCPUs	32 GB RAM	16 Gbps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// ProviderID is the unique identifier as specified by the cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// SSHKeys is the SSH pub keys that will be used to access VM.
    /// ID will take higher precedence over Name if both specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKeys")]
    pub ssh_keys: Option<Vec<IBMVPCMachineSshKeys>>,
    /// Zone is the place where the instance should be created. Example: us-south-3
    pub zone: String,
}

/// BootVolume contains machines's boot volume configurations like size, iops etc..
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineBootVolume {
    /// DeleteVolumeOnInstanceDelete If set to true, when deleting the instance the volume will also be deleted.
    /// Default is set as true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteVolumeOnInstanceDelete")]
    pub delete_volume_on_instance_delete: Option<bool>,
    /// EncryptionKey is the root key to use to wrap the data encryption key for the volume and this points to the CRN
    /// and possible values are as follows.
    /// The CRN of the [Key Protect Root
    /// Key](https://cloud.ibm.com/docs/key-protect?topic=key-protect-getting-started-tutorial) or [Hyper Protect Crypto
    /// Service Root Key](https://cloud.ibm.com/docs/hs-crypto?topic=hs-crypto-get-started) for this resource.
    /// If unspecified, the `encryption` type for the volume will be `provider_managed`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionKeyCRN")]
    pub encryption_key_crn: Option<String>,
    /// Iops is the maximum I/O operations per second (IOPS) to use for the volume. Applicable only to volumes using a profile
    /// family of `custom`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// Name is the unique user-defined name for this volume.
    /// Default will be autogenerated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Profile is the volume profile for the bootdisk, refer https://cloud.ibm.com/docs/vpc?topic=vpc-block-storage-profiles
    /// for more information.
    /// Default to general-purpose
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<IBMVPCMachineBootVolumeProfile>,
    /// SizeGiB is the size of the virtual server's boot disk in GiB.
    /// Default to the size of the image's `minimum_provisioned_size`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeGiB")]
    pub size_gi_b: Option<i64>,
}

/// BootVolume contains machines's boot volume configurations like size, iops etc..
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IBMVPCMachineBootVolumeProfile {
    #[serde(rename = "general-purpose")]
    GeneralPurpose,
    #[serde(rename = "5iops-tier")]
    r#_5iopsTier,
    #[serde(rename = "10iops-tier")]
    r#_10iopsTier,
    #[serde(rename = "custom")]
    Custom,
}

/// CatalogOffering is the Catalog Offering OS image which would be installed on the instance.
/// An OfferingCRN or VersionCRN is required, the PlanCRN is optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineCatalogOffering {
    /// OfferingCRN defines the IBM Cloud Catalog Offering CRN. Using the OfferingCRN expects that the latest version of the Offering will be used.
    /// If a specific version should be used instead, rely on VersionCRN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offeringCRN")]
    pub offering_crn: Option<String>,
    /// PlanCRN defines the IBM Cloud Catalog Offering Plan CRN to use for the Offering.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "planCRN")]
    pub plan_crn: Option<String>,
    /// VersionCRN defines the IBM Cloud Catalog Offering Version CRN. A specific version of the Catalog Offering will be used, as defined by this CRN.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionCRN")]
    pub version_crn: Option<String>,
}

/// Image is the OS image which would be install on the instance.
/// ID will take higher precedence over Name if both specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineImage {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// VPCLoadBalancerBackendPoolMember represents a VPC Load Balancer Backend Pool Member.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineLoadBalancerPoolMembers {
    /// LoadBalancer defines the Load Balancer the Pool Member is for.
    #[serde(rename = "loadBalancer")]
    pub load_balancer: IBMVPCMachineLoadBalancerPoolMembersLoadBalancer,
    /// Pool defines the Load Balancer Pool the Pool Member should be in.
    pub pool: IBMVPCMachineLoadBalancerPoolMembersPool,
    /// Port defines the Port the Load Balancer Pool Member listens for traffic.
    pub port: i64,
    /// Weight of the service member. Only applicable if the pool algorithm is "weighted_round_robin".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// LoadBalancer defines the Load Balancer the Pool Member is for.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineLoadBalancerPoolMembersLoadBalancer {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Pool defines the Load Balancer Pool the Pool Member should be in.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineLoadBalancerPoolMembersPool {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PlacementTarget is the placement restrictions to use for the virtual server instance. No restrictions are used when this field is not defined.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePlacementTarget {
    /// DedicatedHost defines the Dedicated Host to place a VPC Machine (Instance) on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedHost")]
    pub dedicated_host: Option<IBMVPCMachinePlacementTargetDedicatedHost>,
    /// DedicatedHostGroup defines the Dedicated Host Group to use when placing a VPC Machine (Instance).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedHostGroup")]
    pub dedicated_host_group: Option<IBMVPCMachinePlacementTargetDedicatedHostGroup>,
    /// PlacementGroup defines the Placement Group to use when placing a VPC Machine (Instance).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "placementGroup")]
    pub placement_group: Option<IBMVPCMachinePlacementTargetPlacementGroup>,
}

/// DedicatedHost defines the Dedicated Host to place a VPC Machine (Instance) on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePlacementTargetDedicatedHost {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DedicatedHostGroup defines the Dedicated Host Group to use when placing a VPC Machine (Instance).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePlacementTargetDedicatedHostGroup {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PlacementGroup defines the Placement Group to use when placing a VPC Machine (Instance).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePlacementTargetPlacementGroup {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PrimaryNetworkInterface is required to specify subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePrimaryNetworkInterface {
    /// SecurityGroups defines a set of IBM Cloud VPC Security Groups to attach to the network interface.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<Vec<IBMVPCMachinePrimaryNetworkInterfaceSecurityGroups>>,
    /// Subnet ID of the network interface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}

/// VPCResource represents a VPC resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachinePrimaryNetworkInterfaceSecurityGroups {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// IBMVPCResourceReference is a reference to a specific VPC resource by ID or Name
/// Only one of ID or Name may be specified. Specifying more than one will result in
/// a validation error.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineSshKeys {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// IBMVPCMachineStatus defines the observed state of IBMVPCMachine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineStatus {
    /// Addresses contains the IBM Cloud instance associated addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<IBMVPCMachineStatusAddresses>>,
    /// Conditions deefines current service state of the IBMVPCMachine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureMessage will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a more verbose string suitable
    /// for logging and human consumption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// FailureReason will be set in the event that there is a terminal problem
    /// reconciling the Machine and will contain a succinct value suitable
    /// for machine interpretation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// InstanceID defines the IBM Cloud VPC Instance UUID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceID")]
    pub instance_id: Option<String>,
    /// InstanceStatus is the status of the IBM Cloud instance for this machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceState")]
    pub instance_state: Option<String>,
    /// LoadBalancerPoolMembers is the status of IBM Cloud VPC Load Balancer Backend Pools the machine is a member.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerPoolMembers")]
    pub load_balancer_pool_members: Option<Vec<IBMVPCMachineStatusLoadBalancerPoolMembers>>,
    /// Ready is true when the provider resource is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

/// NodeAddress contains information for the node's address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineStatusAddresses {
    /// The node address.
    pub address: String,
    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// VPCLoadBalancerBackendPoolMember represents a VPC Load Balancer Backend Pool Member.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineStatusLoadBalancerPoolMembers {
    /// LoadBalancer defines the Load Balancer the Pool Member is for.
    #[serde(rename = "loadBalancer")]
    pub load_balancer: IBMVPCMachineStatusLoadBalancerPoolMembersLoadBalancer,
    /// Pool defines the Load Balancer Pool the Pool Member should be in.
    pub pool: IBMVPCMachineStatusLoadBalancerPoolMembersPool,
    /// Port defines the Port the Load Balancer Pool Member listens for traffic.
    pub port: i64,
    /// Weight of the service member. Only applicable if the pool algorithm is "weighted_round_robin".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// LoadBalancer defines the Load Balancer the Pool Member is for.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineStatusLoadBalancerPoolMembersLoadBalancer {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Pool defines the Load Balancer Pool the Pool Member should be in.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCMachineStatusLoadBalancerPoolMembersPool {
    /// id of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// name of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

