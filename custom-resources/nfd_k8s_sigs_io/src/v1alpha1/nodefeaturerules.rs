// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/node-feature-discovery-operator/nfd.k8s-sigs.io/v1alpha1/nodefeaturerules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// NodeFeatureRuleSpec describes a NodeFeatureRule.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "nfd.k8s-sigs.io", version = "v1alpha1", kind = "NodeFeatureRule", plural = "nodefeaturerules")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeFeatureRuleSpec {
    /// Rules is a list of node customization rules.
    pub rules: Vec<NodeFeatureRuleRules>,
}

/// Rule defines a rule for node customization such as labeling.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRules {
    /// Annotations to create if the rule matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// ExtendedResources to create if the rule matches.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extendedResources")]
    pub extended_resources: Option<BTreeMap<String, String>>,
    /// Labels to create if the rule matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// LabelsTemplate specifies a template to expand for dynamically generating multiple labels. Data (after template expansion) must be keys with an optional value (<key>[=<value>]) separated by newlines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsTemplate")]
    pub labels_template: Option<String>,
    /// MatchAny specifies a list of matchers one of which must match.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchAny")]
    pub match_any: Option<Vec<NodeFeatureRuleRulesMatchAny>>,
    /// MatchFeatures specifies a set of matcher terms all of which must match.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFeatures")]
    pub match_features: Option<Vec<NodeFeatureRuleRulesMatchFeatures>>,
    /// Name of the rule.
    pub name: String,
    /// Taints to create if the rule matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<NodeFeatureRuleRulesTaints>>,
    /// Vars is the variables to store if the rule matches. Variables do not directly inflict any changes in the node object. However, they can be referenced from other rules enabling more complex rule hierarchies, without exposing intermediary output values as labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vars: Option<BTreeMap<String, String>>,
    /// VarsTemplate specifies a template to expand for dynamically generating multiple variables. Data (after template expansion) must be keys with an optional value (<key>[=<value>]) separated by newlines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "varsTemplate")]
    pub vars_template: Option<String>,
}

/// MatchAnyElem specifies one sub-matcher of MatchAny.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesMatchAny {
    /// MatchFeatures specifies a set of matcher terms all of which must match.
    #[serde(rename = "matchFeatures")]
    pub match_features: Vec<NodeFeatureRuleRulesMatchAnyMatchFeatures>,
}

/// FeatureMatcherTerm defines requirements against one feature set. All requirements (specified as MatchExpressions) are evaluated against each element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesMatchAnyMatchFeatures {
    pub feature: String,
    /// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
    #[serde(rename = "matchExpressions")]
    pub match_expressions: BTreeMap<String, NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchExpressions>,
    /// MatchName in an expression that is matched against the name of each
    /// element in the feature set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchName")]
    pub match_name: Option<NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchName>,
}

/// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchExpressions {
    /// Op is the operator to be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchExpressionsOp>,
    /// Value is the list of values that the operand evaluates the input against. Value should be empty if the operator is Exists, DoesNotExist, IsTrue or IsFalse. Value should contain exactly one element if the operator is Gt or Lt and exactly two elements if the operator is GtLt. In other cases Value should contain at least one element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchExpressionsOp {
    In,
    NotIn,
    InRegexp,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
    GtLt,
    IsTrue,
    IsFalse,
}

/// MatchName in an expression that is matched against the name of each
/// element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchName {
    /// Op is the operator to be applied.
    pub op: NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchNameOp,
    /// Value is the list of values that the operand evaluates the input
    /// against. Value should be empty if the operator is Exists, DoesNotExist,
    /// IsTrue or IsFalse. Value should contain exactly one element if the
    /// operator is Gt or Lt and exactly two elements if the operator is GtLt.
    /// In other cases Value should contain at least one element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// MatchName in an expression that is matched against the name of each
/// element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeFeatureRuleRulesMatchAnyMatchFeaturesMatchNameOp {
    In,
    NotIn,
    InRegexp,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
    GtLt,
    IsTrue,
    IsFalse,
}

/// FeatureMatcherTerm defines requirements against one feature set. All requirements (specified as MatchExpressions) are evaluated against each element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesMatchFeatures {
    pub feature: String,
    /// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
    #[serde(rename = "matchExpressions")]
    pub match_expressions: BTreeMap<String, NodeFeatureRuleRulesMatchFeaturesMatchExpressions>,
    /// MatchName in an expression that is matched against the name of each
    /// element in the feature set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchName")]
    pub match_name: Option<NodeFeatureRuleRulesMatchFeaturesMatchName>,
}

/// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesMatchFeaturesMatchExpressions {
    /// Op is the operator to be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<NodeFeatureRuleRulesMatchFeaturesMatchExpressionsOp>,
    /// Value is the list of values that the operand evaluates the input against. Value should be empty if the operator is Exists, DoesNotExist, IsTrue or IsFalse. Value should contain exactly one element if the operator is Gt or Lt and exactly two elements if the operator is GtLt. In other cases Value should contain at least one element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// MatchExpressionSet contains a set of MatchExpressions, each of which is evaluated against a set of input values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeFeatureRuleRulesMatchFeaturesMatchExpressionsOp {
    In,
    NotIn,
    InRegexp,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
    GtLt,
    IsTrue,
    IsFalse,
}

/// MatchName in an expression that is matched against the name of each
/// element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeFeatureRuleRulesMatchFeaturesMatchName {
    /// Op is the operator to be applied.
    pub op: NodeFeatureRuleRulesMatchFeaturesMatchNameOp,
    /// Value is the list of values that the operand evaluates the input
    /// against. Value should be empty if the operator is Exists, DoesNotExist,
    /// IsTrue or IsFalse. Value should contain exactly one element if the
    /// operator is Gt or Lt and exactly two elements if the operator is GtLt.
    /// In other cases Value should contain at least one element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// MatchName in an expression that is matched against the name of each
/// element in the feature set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeFeatureRuleRulesMatchFeaturesMatchNameOp {
    In,
    NotIn,
    InRegexp,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
    GtLt,
    IsTrue,
    IsFalse,
}

/// The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureRuleRulesTaints {
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

