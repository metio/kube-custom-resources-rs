// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1/clusterdefinitions.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ClusterDefinitionSpec defines the desired state of ClusterDefinition.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1", kind = "ClusterDefinition", plural = "clusterdefinitions")]
#[kube(status = "ClusterDefinitionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterDefinitionSpec {
    /// Topologies defines all possible topologies within the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topologies: Option<Vec<ClusterDefinitionTopologies>>,
}

/// ClusterTopology represents the definition for a specific cluster topology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDefinitionTopologies {
    /// Components specifies the components in the topology.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ClusterDefinitionTopologiesComponents>>,
    /// Default indicates whether this topology serves as the default configuration.
    /// When set to true, this topology is automatically used unless another is explicitly specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// Name is the unique identifier for the cluster topology.
    /// Cannot be updated.
    pub name: String,
    /// Specifies the sequence in which components within a cluster topology are
    /// started, stopped, and upgraded.
    /// This ordering is crucial for maintaining the correct dependencies and operational flow across components.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<ClusterDefinitionTopologiesOrders>,
    /// Shardings specifies the shardings in the topology.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shardings: Option<Vec<ClusterDefinitionTopologiesShardings>>,
}

/// ClusterTopologyComponent defines a Component within a ClusterTopology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDefinitionTopologiesComponents {
    /// Specifies the exact name, name prefix, or regular expression pattern for matching the name of the ComponentDefinition
    /// custom resource (CR) that defines the Component's characteristics and behavior.
    /// 
    /// 
    /// The system selects the ComponentDefinition CR with the latest version that matches the pattern.
    /// This approach allows:
    /// 
    /// 
    /// 1. Precise selection by providing the exact name of a ComponentDefinition CR.
    /// 2. Flexible and automatic selection of the most up-to-date ComponentDefinition CR
    /// 	  by specifying a name prefix or regular expression pattern.
    /// 
    /// 
    /// Cannot be updated once set.
    #[serde(rename = "compDef")]
    pub comp_def: String,
    /// Defines the unique identifier of the component within the cluster topology.
    /// 
    /// 
    /// It follows IANA Service naming rules and is used as part of the Service's DNS name.
    /// The name must start with a lowercase letter, can contain lowercase letters, numbers,
    /// and hyphens, and must end with a lowercase letter or number.
    /// 
    /// 
    /// If the @template field is set to true, the name will be used as a prefix to match the specific components dynamically created.
    /// 
    /// 
    /// Cannot be updated once set.
    pub name: String,
    /// Specifies whether the topology component will be considered as a template for instantiating components upon user requests dynamically.
    /// 
    /// 
    /// Cannot be updated once set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
}

/// Specifies the sequence in which components within a cluster topology are
/// started, stopped, and upgraded.
/// This ordering is crucial for maintaining the correct dependencies and operational flow across components.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDefinitionTopologiesOrders {
    /// Specifies the order for creating and initializing entities.
    /// This is designed for entities that depend on one another. Entities without dependencies can be grouped together.
    /// 
    /// 
    /// Entities that can be provisioned independently or have no dependencies can be listed together in the same stage,
    /// separated by commas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision: Option<Vec<String>>,
    /// Outlines the order for stopping and deleting entities.
    /// This sequence is designed for entities that require a graceful shutdown or have interdependencies.
    /// 
    /// 
    /// Entities that can be terminated independently or have no dependencies can be listed together in the same stage,
    /// separated by commas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminate: Option<Vec<String>>,
    /// Update determines the order for updating entities' specifications, such as image upgrades or resource scaling.
    /// This sequence is designed for entities that have dependencies or require specific update procedures.
    /// 
    /// 
    /// Entities that can be updated independently or have no dependencies can be listed together in the same stage,
    /// separated by commas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<Vec<String>>,
}

/// ClusterTopologySharding defines a sharding within a ClusterTopology.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDefinitionTopologiesShardings {
    /// Defines the unique identifier of the sharding within the cluster topology.
    /// It follows IANA Service naming rules and is used as part of the Service's DNS name.
    /// The name must start with a lowercase letter, can contain lowercase letters, numbers,
    /// and hyphens, and must end with a lowercase letter or number.
    /// 
    /// 
    /// Cannot be updated once set.
    pub name: String,
    /// Specifies the sharding definition that defines the characteristics and behavior of the sharding.
    /// 
    /// 
    /// The system selects the ShardingDefinition CR with the latest version that matches the pattern.
    /// This approach allows:
    /// 
    /// 
    /// 1. Precise selection by providing the exact name of a ShardingDefinition CR.
    /// 2. Flexible and automatic selection of the most up-to-date ShardingDefinition CR
    /// by specifying a regular expression pattern.
    /// 
    /// 
    /// Once set, this field cannot be updated.
    #[serde(rename = "shardingDef")]
    pub sharding_def: String,
}

/// ClusterDefinitionStatus defines the observed state of ClusterDefinition
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDefinitionStatus {
    /// Provides additional information about the current phase.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the most recent generation observed for this ClusterDefinition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Specifies the current phase of the ClusterDefinition. Valid values are `empty`, `Available`, `Unavailable`.
    /// When `Available`, the ClusterDefinition is ready and can be referenced by related objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ClusterDefinitionStatusPhase>,
    /// Topologies this ClusterDefinition supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topologies: Option<String>,
}

/// ClusterDefinitionStatus defines the observed state of ClusterDefinition
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterDefinitionStatusPhase {
    Available,
    Unavailable,
}

