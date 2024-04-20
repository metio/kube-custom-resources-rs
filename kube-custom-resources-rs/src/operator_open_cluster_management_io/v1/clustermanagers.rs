// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-cluster-management-io/ocm/operator.open-cluster-management.io/v1/clustermanagers.yaml --derive=PartialEq
// kopium version: 0.18.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// Spec represents a desired deployment configuration of controllers that govern registration and work distribution for attached Klusterlets.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.open-cluster-management.io", version = "v1", kind = "ClusterManager", plural = "clustermanagers")]
#[kube(status = "ClusterManagerStatus")]
#[kube(schema = "disabled")]
pub struct ClusterManagerSpec {
    /// AddOnManagerConfiguration contains the configuration of addon manager
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addOnManagerConfiguration")]
    pub add_on_manager_configuration: Option<ClusterManagerAddOnManagerConfiguration>,
    /// AddOnManagerImagePullSpec represents the desired image configuration of addon manager controller/webhook installed on hub.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addOnManagerImagePullSpec")]
    pub add_on_manager_image_pull_spec: Option<String>,
    /// DeployOption contains the options of deploying a cluster-manager Default mode is used if DeployOption is not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deployOption")]
    pub deploy_option: Option<ClusterManagerDeployOption>,
    /// NodePlacement enables explicit control over the scheduling of the deployed pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodePlacement")]
    pub node_placement: Option<ClusterManagerNodePlacement>,
    /// PlacementImagePullSpec represents the desired image configuration of placement controller/webhook installed on hub.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "placementImagePullSpec")]
    pub placement_image_pull_spec: Option<String>,
    /// RegistrationConfiguration contains the configuration of registration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrationConfiguration")]
    pub registration_configuration: Option<ClusterManagerRegistrationConfiguration>,
    /// RegistrationImagePullSpec represents the desired image of registration controller/webhook installed on hub.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrationImagePullSpec")]
    pub registration_image_pull_spec: Option<String>,
    /// ResourceRequirement specify QoS classes of deployments managed by clustermanager. It applies to all the containers in the deployments.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRequirement")]
    pub resource_requirement: Option<ClusterManagerResourceRequirement>,
    /// WorkConfiguration contains the configuration of work
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workConfiguration")]
    pub work_configuration: Option<ClusterManagerWorkConfiguration>,
    /// WorkImagePullSpec represents the desired image configuration of work controller/webhook installed on hub.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workImagePullSpec")]
    pub work_image_pull_spec: Option<String>,
}

/// AddOnManagerConfiguration contains the configuration of addon manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerAddOnManagerConfiguration {
    /// FeatureGates represents the list of feature gates for addon manager If it is set empty, default feature gates will be used. If it is set, featuregate/Foo is an example of one item in FeatureGates: 1. If featuregate/Foo does not exist, registration-operator will discard it 2. If featuregate/Foo exists and is false by default. It is now possible to set featuregate/Foo=[false|true] 3. If featuregate/Foo exists and is true by default. If a cluster-admin upgrading from 1 to 2 wants to continue having featuregate/Foo=false, he can set featuregate/Foo=false before upgrading. Let's say the cluster-admin wants featuregate/Foo=false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGates")]
    pub feature_gates: Option<Vec<ClusterManagerAddOnManagerConfigurationFeatureGates>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerAddOnManagerConfigurationFeatureGates {
    /// Feature is the key of feature gate. e.g. featuregate/Foo.
    pub feature: String,
    /// Mode is either Enable, Disable, "" where "" is Disable by default. In Enable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=true". In Disable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=false".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ClusterManagerAddOnManagerConfigurationFeatureGatesMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerAddOnManagerConfigurationFeatureGatesMode {
    Enable,
    Disable,
}

/// DeployOption contains the options of deploying a cluster-manager Default mode is used if DeployOption is not set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerDeployOption {
    /// Hosted includes configurations we need for clustermanager in the Hosted mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosted: Option<ClusterManagerDeployOptionHosted>,
    /// Mode can be Default or Hosted. In Default mode, the Hub is installed as a whole and all parts of Hub are deployed in the same cluster. In Hosted mode, only crd and configurations are installed on one cluster(defined as hub-cluster). Controllers run in another cluster (defined as management-cluster) and connect to the hub with the kubeconfig in secret of "external-hub-kubeconfig"(a kubeconfig of hub-cluster with cluster-admin permission). Note: Do not modify the Mode field once it's applied.
    pub mode: ClusterManagerDeployOptionMode,
}

/// Hosted includes configurations we need for clustermanager in the Hosted mode.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerDeployOptionHosted {
    /// RegistrationWebhookConfiguration represents the customized webhook-server configuration of registration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registrationWebhookConfiguration")]
    pub registration_webhook_configuration: Option<ClusterManagerDeployOptionHostedRegistrationWebhookConfiguration>,
    /// WorkWebhookConfiguration represents the customized webhook-server configuration of work.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workWebhookConfiguration")]
    pub work_webhook_configuration: Option<ClusterManagerDeployOptionHostedWorkWebhookConfiguration>,
}

/// RegistrationWebhookConfiguration represents the customized webhook-server configuration of registration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerDeployOptionHostedRegistrationWebhookConfiguration {
    /// Address represents the address of a webhook-server. It could be in IP format or fqdn format. The Address must be reachable by apiserver of the hub cluster.
    pub address: String,
    /// Port represents the port of a webhook-server. The default value of Port is 443.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// WorkWebhookConfiguration represents the customized webhook-server configuration of work.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerDeployOptionHostedWorkWebhookConfiguration {
    /// Address represents the address of a webhook-server. It could be in IP format or fqdn format. The Address must be reachable by apiserver of the hub cluster.
    pub address: String,
    /// Port represents the port of a webhook-server. The default value of Port is 443.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// DeployOption contains the options of deploying a cluster-manager Default mode is used if DeployOption is not set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerDeployOptionMode {
    Default,
    Hosted,
}

/// NodePlacement enables explicit control over the scheduling of the deployed pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerNodePlacement {
    /// NodeSelector defines which Nodes the Pods are scheduled on. The default is an empty list.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// Tolerations are attached by pods to tolerate any taint that matches the triple <key,value,effect> using the matching operator <operator>. The default is an empty list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<ClusterManagerNodePlacementTolerations>>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerNodePlacementTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// RegistrationConfiguration contains the configuration of registration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerRegistrationConfiguration {
    /// AutoApproveUser represents a list of users that can auto approve CSR and accept client. If the credential of the bootstrap-hub-kubeconfig matches to the users, the cluster created by the bootstrap-hub-kubeconfig will be auto-registered into the hub cluster. This takes effect only when ManagedClusterAutoApproval feature gate is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoApproveUsers")]
    pub auto_approve_users: Option<Vec<String>>,
    /// FeatureGates represents the list of feature gates for registration If it is set empty, default feature gates will be used. If it is set, featuregate/Foo is an example of one item in FeatureGates: 1. If featuregate/Foo does not exist, registration-operator will discard it 2. If featuregate/Foo exists and is false by default. It is now possible to set featuregate/Foo=[false|true] 3. If featuregate/Foo exists and is true by default. If a cluster-admin upgrading from 1 to 2 wants to continue having featuregate/Foo=false, he can set featuregate/Foo=false before upgrading. Let's say the cluster-admin wants featuregate/Foo=false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGates")]
    pub feature_gates: Option<Vec<ClusterManagerRegistrationConfigurationFeatureGates>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerRegistrationConfigurationFeatureGates {
    /// Feature is the key of feature gate. e.g. featuregate/Foo.
    pub feature: String,
    /// Mode is either Enable, Disable, "" where "" is Disable by default. In Enable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=true". In Disable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=false".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ClusterManagerRegistrationConfigurationFeatureGatesMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerRegistrationConfigurationFeatureGatesMode {
    Enable,
    Disable,
}

/// ResourceRequirement specify QoS classes of deployments managed by clustermanager. It applies to all the containers in the deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerResourceRequirement {
    /// ResourceRequirements defines resource requests and limits when Type is ResourceQosClassResourceRequirement
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRequirements")]
    pub resource_requirements: Option<ClusterManagerResourceRequirementResourceRequirements>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ClusterManagerResourceRequirementType>,
}

/// ResourceRequirements defines resource requests and limits when Type is ResourceQosClassResourceRequirement
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerResourceRequirementResourceRequirements {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ClusterManagerResourceRequirementResourceRequirementsClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerResourceRequirementResourceRequirementsClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// ResourceRequirement specify QoS classes of deployments managed by clustermanager. It applies to all the containers in the deployments.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerResourceRequirementType {
    Default,
    BestEffort,
    ResourceRequirement,
}

/// WorkConfiguration contains the configuration of work
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerWorkConfiguration {
    /// FeatureGates represents the list of feature gates for work If it is set empty, default feature gates will be used. If it is set, featuregate/Foo is an example of one item in FeatureGates: 1. If featuregate/Foo does not exist, registration-operator will discard it 2. If featuregate/Foo exists and is false by default. It is now possible to set featuregate/Foo=[false|true] 3. If featuregate/Foo exists and is true by default. If a cluster-admin upgrading from 1 to 2 wants to continue having featuregate/Foo=false, he can set featuregate/Foo=false before upgrading. Let's say the cluster-admin wants featuregate/Foo=false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGates")]
    pub feature_gates: Option<Vec<ClusterManagerWorkConfigurationFeatureGates>>,
    /// WorkDriver represents the type of work driver. Possible values are "kube", "mqtt", or "grpc". If not provided, the default value is "kube". If set to non-"kube" drivers, the klusterlet need to use the same driver. and the driver configuration must be provided in a secret named "work-driver-config" in the namespace where the cluster manager is running, adhering to the following structure: config.yaml: | <driver-config-in-yaml> 
    ///  For detailed driver configuration, please refer to the sdk-go documentation: https://github.com/open-cluster-management-io/sdk-go/blob/main/pkg/cloudevents/README.md#supported-protocols-and-drivers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workDriver")]
    pub work_driver: Option<ClusterManagerWorkConfigurationWorkDriver>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerWorkConfigurationFeatureGates {
    /// Feature is the key of feature gate. e.g. featuregate/Foo.
    pub feature: String,
    /// Mode is either Enable, Disable, "" where "" is Disable by default. In Enable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=true". In Disable mode, a valid feature gate `featuregate/Foo` will be set to "--featuregate/Foo=false".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ClusterManagerWorkConfigurationFeatureGatesMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerWorkConfigurationFeatureGatesMode {
    Enable,
    Disable,
}

/// WorkConfiguration contains the configuration of work
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterManagerWorkConfigurationWorkDriver {
    #[serde(rename = "kube")]
    Kube,
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "grpc")]
    Grpc,
}

/// Status represents the current status of controllers that govern the lifecycle of managed clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerStatus {
    /// Conditions contain the different condition statuses for this ClusterManager. Valid condition types are: Applied: Components in hub are applied. Available: Components in hub are available and ready to serve. Progressing: Components in hub are in a transitioning state. Degraded: Components in hub do not match the desired configuration and only provide degraded service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Generations are used to determine when an item needs to be reconciled or has changed in a way that needs a reaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<ClusterManagerStatusGenerations>>,
    /// ObservedGeneration is the last generation change you've dealt with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// RelatedResources are used to track the resources that are related to this ClusterManager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relatedResources")]
    pub related_resources: Option<Vec<ClusterManagerStatusRelatedResources>>,
}

/// GenerationStatus keeps track of the generation for a given resource so that decisions about forced updates can be made. The definition matches the GenerationStatus defined in github.com/openshift/api/v1
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerStatusGenerations {
    /// group is the group of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// lastGeneration is the last generation of the resource that controller applies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastGeneration")]
    pub last_generation: Option<i64>,
    /// name is the name of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the resource that you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// version is the version of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// RelatedResourceMeta represents the resource that is managed by an operator
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterManagerStatusRelatedResources {
    /// group is the group of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// name is the name of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the thing you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the resource that you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// version is the version of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

