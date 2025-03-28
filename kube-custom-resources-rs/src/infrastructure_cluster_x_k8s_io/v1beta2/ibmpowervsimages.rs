// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta2/ibmpowervsimages.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// IBMPowerVSImageSpec defines the desired state of IBMPowerVSImage.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta2", kind = "IBMPowerVSImage", plural = "ibmpowervsimages")]
#[kube(namespaced)]
#[kube(status = "IBMPowerVSImageStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMPowerVSImageSpec {
    /// Cloud Object Storage bucket name; bucket-name[/optional/folder]
    pub bucket: String,
    /// ClusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// DeletePolicy defines the policy used to identify images to be preserved beyond the lifecycle of associated cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletePolicy")]
    pub delete_policy: Option<IBMPowerVSImageDeletePolicy>,
    /// Cloud Object Storage image filename.
    pub object: String,
    /// Cloud Object Storage region.
    pub region: String,
    /// serviceInstance is the reference to the Power VS workspace on which the server instance(VM) will be created.
    /// Power VS workspace is a container for all Power VS instances at a specific geographic region.
    /// serviceInstance can be created via IBM Cloud catalog or CLI.
    /// supported serviceInstance identifier in PowerVSResource are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
    /// More detail about Power VS service instance.
    /// https://cloud.ibm.com/docs/power-iaas?topic=power-iaas-creating-power-virtual-server
    /// when omitted system will dynamically create the service instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceInstance")]
    pub service_instance: Option<IBMPowerVSImageServiceInstance>,
    /// ServiceInstanceID is the id of the power cloud instance where the image will get imported.
    /// Deprecated: use ServiceInstance instead
    #[serde(rename = "serviceInstanceID")]
    pub service_instance_id: String,
    /// Type of storage, storage pool with the most available space will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageType")]
    pub storage_type: Option<IBMPowerVSImageStorageType>,
}

/// IBMPowerVSImageSpec defines the desired state of IBMPowerVSImage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IBMPowerVSImageDeletePolicy {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "retain")]
    Retain,
}

/// serviceInstance is the reference to the Power VS workspace on which the server instance(VM) will be created.
/// Power VS workspace is a container for all Power VS instances at a specific geographic region.
/// serviceInstance can be created via IBM Cloud catalog or CLI.
/// supported serviceInstance identifier in PowerVSResource are Name and ID and that can be obtained from IBM Cloud UI or IBM Cloud cli.
/// More detail about Power VS service instance.
/// https://cloud.ibm.com/docs/power-iaas?topic=power-iaas-creating-power-virtual-server
/// when omitted system will dynamically create the service instance
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSImageServiceInstance {
    /// ID of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Regular expression to match resource,
    /// In case of multiple resources matches the provided regular expression the first matched resource will be selected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

/// IBMPowerVSImageSpec defines the desired state of IBMPowerVSImage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IBMPowerVSImageStorageType {
    #[serde(rename = "tier1")]
    Tier1,
    #[serde(rename = "tier3")]
    Tier3,
}

/// IBMPowerVSImageStatus defines the observed state of IBMPowerVSImage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMPowerVSImageStatus {
    /// Conditions defines current service state of the IBMPowerVSImage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ImageID is the id of the imported image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageID")]
    pub image_id: Option<String>,
    /// ImageState is the status of the imported image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageState")]
    pub image_state: Option<String>,
    /// JobID is the job ID of an import operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobID")]
    pub job_id: Option<String>,
    /// Ready is true when the provider resource is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
}

