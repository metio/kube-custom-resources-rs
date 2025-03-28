// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/networkfirewall-controller/networkfirewall.services.k8s.aws/v1alpha1/firewallpolicies.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FirewallPolicySpec defines the desired state of FirewallPolicy.
/// 
/// The firewall policy defines the behavior of a firewall using a collection
/// of stateless and stateful rule groups and other settings. You can use one
/// firewall policy for multiple firewalls.
/// 
/// This, along with FirewallPolicyResponse, define the policy. You can retrieve
/// all objects for a firewall policy by calling DescribeFirewallPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networkfirewall.services.k8s.aws", version = "v1alpha1", kind = "FirewallPolicy", plural = "firewallpolicies")]
#[kube(namespaced)]
#[kube(status = "FirewallPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FirewallPolicySpec {
    /// A description of the firewall policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A complex type that contains settings for encryption of your firewall policy
    /// resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<FirewallPolicyEncryptionConfiguration>,
    /// The rule groups and policy actions to use in the firewall policy.
    #[serde(rename = "firewallPolicy")]
    pub firewall_policy: FirewallPolicyFirewallPolicy,
    /// The descriptive name of the firewall policy. You can't change the name of
    /// a firewall policy after you create it.
    #[serde(rename = "firewallPolicyName")]
    pub firewall_policy_name: String,
    /// The key:value pairs to associate with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FirewallPolicyTags>>,
}

/// A complex type that contains settings for encryption of your firewall policy
/// resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// The rule groups and policy actions to use in the firewall policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicy {
    /// Contains variables that you can use to override default Suricata settings
    /// in your firewall policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyVariables")]
    pub policy_variables: Option<FirewallPolicyFirewallPolicyPolicyVariables>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulDefaultActions")]
    pub stateful_default_actions: Option<Vec<String>>,
    /// Configuration settings for the handling of the stateful rule groups in a
    /// firewall policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulEngineOptions")]
    pub stateful_engine_options: Option<FirewallPolicyFirewallPolicyStatefulEngineOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulRuleGroupReferences")]
    pub stateful_rule_group_references: Option<Vec<FirewallPolicyFirewallPolicyStatefulRuleGroupReferences>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessCustomActions")]
    pub stateless_custom_actions: Option<Vec<FirewallPolicyFirewallPolicyStatelessCustomActions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessDefaultActions")]
    pub stateless_default_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessFragmentDefaultActions")]
    pub stateless_fragment_default_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessRuleGroupReferences")]
    pub stateless_rule_group_references: Option<Vec<FirewallPolicyFirewallPolicyStatelessRuleGroupReferences>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsInspectionConfigurationARN")]
    pub tls_inspection_configuration_arn: Option<String>,
}

/// Contains variables that you can use to override default Suricata settings
/// in your firewall policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyPolicyVariables {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleVariables")]
    pub rule_variables: Option<BTreeMap<String, FirewallPolicyFirewallPolicyPolicyVariablesRuleVariables>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyPolicyVariablesRuleVariables {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

/// Configuration settings for the handling of the stateful rule groups in a
/// firewall policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatefulEngineOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleOrder")]
    pub rule_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamExceptionPolicy")]
    pub stream_exception_policy: Option<String>,
}

/// Identifier for a single stateful rule group, used in a firewall policy to
/// refer to a rule group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatefulRuleGroupReferences {
    /// The setting that allows the policy owner to change the behavior of the rule
    /// group within a policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<FirewallPolicyFirewallPolicyStatefulRuleGroupReferencesOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceARN")]
    pub resource_arn: Option<String>,
}

/// The setting that allows the policy owner to change the behavior of the rule
/// group within a policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatefulRuleGroupReferencesOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// An optional, non-standard action to use for stateless packet handling. You
/// can define this in addition to the standard action that you must specify.
/// 
/// You define and name the custom actions that you want to be able to use, and
/// then you reference them by name in your actions settings.
/// 
/// You can use custom actions in the following places:
/// 
///    * In a rule group's StatelessRulesAndCustomActions specification. The
///    custom actions are available for use by name inside the StatelessRulesAndCustomActions
///    where you define them. You can use them for your stateless rule actions
///    to specify what to do with a packet that matches the rule's match attributes.
/// 
///    * In a FirewallPolicy specification, in StatelessCustomActions. The custom
///    actions are available for use inside the policy where you define them.
///    You can use them for the policy's default stateless actions settings to
///    specify what to do with packets that don't match any of the policy's stateless
///    rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatelessCustomActions {
    /// A custom action to use in stateless rule actions settings. This is used in
    /// CustomAction.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionDefinition")]
    pub action_definition: Option<FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionName")]
    pub action_name: Option<String>,
}

/// A custom action to use in stateless rule actions settings. This is used in
/// CustomAction.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinition {
    /// Stateless inspection criteria that publishes the specified metrics to Amazon
    /// CloudWatch for the matching packet. This setting defines a CloudWatch dimension
    /// value to be published.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publishMetricAction")]
    pub publish_metric_action: Option<FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinitionPublishMetricAction>,
}

/// Stateless inspection criteria that publishes the specified metrics to Amazon
/// CloudWatch for the matching packet. This setting defines a CloudWatch dimension
/// value to be published.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinitionPublishMetricAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinitionPublishMetricActionDimensions>>,
}

/// The value to use in an Amazon CloudWatch custom metric dimension. This is
/// used in the PublishMetrics CustomAction. A CloudWatch custom metric dimension
/// is a name/value pair that's part of the identity of a metric.
/// 
/// Network Firewall sets the dimension name to CustomAction and you provide
/// the dimension value.
/// 
/// For more information about CloudWatch custom metric dimensions, see Publishing
/// Custom Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html#usingDimensions)
/// in the Amazon CloudWatch User Guide (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatelessCustomActionsActionDefinitionPublishMetricActionDimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Identifier for a single stateless rule group, used in a firewall policy to
/// refer to the rule group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyFirewallPolicyStatelessRuleGroupReferences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceARN")]
    pub resource_arn: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// FirewallPolicyStatus defines the observed state of FirewallPolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FirewallPolicyStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The high-level properties of a firewall policy. This, along with the FirewallPolicy,
    /// define the policy. You can retrieve all objects for a firewall policy by
    /// calling DescribeFirewallPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyResponse")]
    pub firewall_policy_response: Option<FirewallPolicyStatusFirewallPolicyResponse>,
    /// A token used for optimistic locking. Network Firewall returns a token to
    /// your requests that access the firewall policy. The token marks the state
    /// of the policy resource at the time of the request.
    /// 
    /// To make changes to the policy, you provide the token in your request. Network
    /// Firewall uses the token to ensure that the policy hasn't changed since you
    /// last retrieved it. If it has changed, the operation fails with an InvalidTokenException.
    /// If this happens, retrieve the firewall policy again to get a current copy
    /// of it with current token. Reapply your changes as needed, then try the operation
    /// again using the new token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateToken")]
    pub update_token: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// The high-level properties of a firewall policy. This, along with the FirewallPolicy,
/// define the policy. You can retrieve all objects for a firewall policy by
/// calling DescribeFirewallPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyStatusFirewallPolicyResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumedStatefulRuleCapacity")]
    pub consumed_stateful_rule_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumedStatelessRuleCapacity")]
    pub consumed_stateless_rule_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A complex type that contains optional Amazon Web Services Key Management
    /// Service (KMS) encryption settings for your Network Firewall resources. Your
    /// data is encrypted by default with an Amazon Web Services owned key that Amazon
    /// Web Services owns and manages for you. You can use either the Amazon Web
    /// Services owned key, or provide your own customer managed key. To learn more
    /// about KMS encryption of your Network Firewall resources, see Encryption at
    /// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
    /// in the Network Firewall Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<FirewallPolicyStatusFirewallPolicyResponseEncryptionConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyARN")]
    pub firewall_policy_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyID")]
    pub firewall_policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyName")]
    pub firewall_policy_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firewallPolicyStatus")]
    pub firewall_policy_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfAssociations")]
    pub number_of_associations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<FirewallPolicyStatusFirewallPolicyResponseTags>>,
}

/// A complex type that contains optional Amazon Web Services Key Management
/// Service (KMS) encryption settings for your Network Firewall resources. Your
/// data is encrypted by default with an Amazon Web Services owned key that Amazon
/// Web Services owns and manages for you. You can use either the Amazon Web
/// Services owned key, or provide your own customer managed key. To learn more
/// about KMS encryption of your Network Firewall resources, see Encryption at
/// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
/// in the Network Firewall Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyStatusFirewallPolicyResponseEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FirewallPolicyStatusFirewallPolicyResponseTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

