// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/stackabletech/zookeeper-operator/zookeeper.stackable.tech/v1alpha1/zookeeperznodes.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// A claim for a single ZooKeeper ZNode tree (filesystem node).
/// 
/// A ConfigMap will automatically be created with the same name, containing the connection string in the field `ZOOKEEPER`. Each ZookeeperZnode gets an isolated ZNode chroot, which the `ZOOKEEPER` automatically contains. All data inside of this chroot will be deleted when the corresponding `ZookeeperZnode` is.
/// 
/// `ZookeeperZnode` is *not* designed to manage the contents of this ZNode. Instead, it should be used to create a chroot for an installation of an application to work inside. Initializing the contents is the responsibility of the application.
/// 
/// You can learn more about this in the [Isolating clients with ZNodes usage guide](https://docs.stackable.tech/home/nightly/zookeeper/usage_guide/isolating_clients_with_znodes).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "zookeeper.stackable.tech", version = "v1alpha1", kind = "ZookeeperZnode", plural = "zookeeperznodes")]
#[kube(namespaced)]
#[kube(status = "ZookeeperZnodeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ZookeeperZnodeSpec {
    /// The reference to the ZookeeperCluster that this ZNode belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterRef")]
    pub cluster_ref: Option<ZookeeperZnodeClusterRef>,
}

/// The reference to the ZookeeperCluster that this ZNode belongs to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ZookeeperZnodeClusterRef {
    /// The name of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The namespace of the cluster
    /// 
    /// This field is optional, and will default to the namespace of the referring object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ZookeeperZnodeStatus {
    /// The absolute ZNode allocated to the ZookeeperZnode. This will typically be set by the operator.
    /// 
    /// This can be set explicitly by an administrator, such as when restoring from a backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "znodePath")]
    pub znode_path: Option<String>,
}
