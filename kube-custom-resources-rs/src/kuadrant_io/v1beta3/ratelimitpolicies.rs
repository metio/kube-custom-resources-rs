// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Kuadrant/kuadrant-operator/kuadrant.io/v1beta3/ratelimitpolicies.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuadrant.io", version = "v1beta3", kind = "RateLimitPolicy", plural = "ratelimitpolicies")]
#[kube(namespaced)]
#[kube(status = "RateLimitPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RateLimitPolicySpec {
    /// Rules to apply as defaults. Can be overridden by more specific policiy rules lower in the hierarchy and by less specific policy overrides.
    /// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<RateLimitPolicyDefaults>,
    /// Limits holds the struct of limits indexed by a unique name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, RateLimitPolicyLimits>>,
    /// Rules to apply as overrides. Override all policy rules lower in the hierarchy. Can be overridden by less specific policy overrides.
    /// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<RateLimitPolicyOverrides>,
    /// Reference to the object to which this policy applies.
    #[serde(rename = "targetRef")]
    pub target_ref: RateLimitPolicyTargetRef,
}

/// Rules to apply as defaults. Can be overridden by more specific policiy rules lower in the hierarchy and by less specific policy overrides.
/// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaults {
    /// Limits holds the struct of limits indexed by a unique name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, RateLimitPolicyDefaultsLimits>>,
    /// Strategy defines the merge strategy to apply when merging this policy with other policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<RateLimitPolicyDefaultsStrategy>,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsLimits {
    /// Counters defines additional rate limit counters based on context qualifiers and well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<String>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyDefaultsLimitsRates>>,
    /// When holds the list of conditions for the policy to be enforced.
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyDefaultsLimitsWhen>>,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyDefaultsLimitsRates {
    /// Duration defines the time period for which the Limit specified above applies.
    pub duration: i64,
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Duration defines the time uni
    /// Possible values are: "second", "minute", "hour", "day"
    pub unit: RateLimitPolicyDefaultsLimitsRatesUnit,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyDefaultsLimitsRatesUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyDefaultsLimitsWhen {
    /// The binary operator to be applied to the content fetched from the selector
    /// Possible values are: "eq" (equal to), "neq" (not equal to)
    pub operator: RateLimitPolicyDefaultsLimitsWhenOperator,
    /// Selector defines one item from the well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    pub selector: String,
    /// The value of reference for the comparison.
    pub value: String,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyDefaultsLimitsWhenOperator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
    #[serde(rename = "startswith")]
    Startswith,
    #[serde(rename = "endswith")]
    Endswith,
    #[serde(rename = "incl")]
    Incl,
    #[serde(rename = "excl")]
    Excl,
    #[serde(rename = "matches")]
    Matches,
}

/// Rules to apply as defaults. Can be overridden by more specific policiy rules lower in the hierarchy and by less specific policy overrides.
/// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyDefaultsStrategy {
    #[serde(rename = "atomic")]
    Atomic,
    #[serde(rename = "merge")]
    Merge,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyLimits {
    /// Counters defines additional rate limit counters based on context qualifiers and well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<String>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyLimitsRates>>,
    /// When holds the list of conditions for the policy to be enforced.
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyLimitsWhen>>,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyLimitsRates {
    /// Duration defines the time period for which the Limit specified above applies.
    pub duration: i64,
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Duration defines the time uni
    /// Possible values are: "second", "minute", "hour", "day"
    pub unit: RateLimitPolicyLimitsRatesUnit,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyLimitsRatesUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyLimitsWhen {
    /// The binary operator to be applied to the content fetched from the selector
    /// Possible values are: "eq" (equal to), "neq" (not equal to)
    pub operator: RateLimitPolicyLimitsWhenOperator,
    /// Selector defines one item from the well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    pub selector: String,
    /// The value of reference for the comparison.
    pub value: String,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyLimitsWhenOperator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
    #[serde(rename = "startswith")]
    Startswith,
    #[serde(rename = "endswith")]
    Endswith,
    #[serde(rename = "incl")]
    Incl,
    #[serde(rename = "excl")]
    Excl,
    #[serde(rename = "matches")]
    Matches,
}

/// Rules to apply as overrides. Override all policy rules lower in the hierarchy. Can be overridden by less specific policy overrides.
/// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverrides {
    /// Limits holds the struct of limits indexed by a unique name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, RateLimitPolicyOverridesLimits>>,
    /// Strategy defines the merge strategy to apply when merging this policy with other policies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<RateLimitPolicyOverridesStrategy>,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesLimits {
    /// Counters defines additional rate limit counters based on context qualifiers and well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<String>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyOverridesLimitsRates>>,
    /// When holds the list of conditions for the policy to be enforced.
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyOverridesLimitsWhen>>,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyOverridesLimitsRates {
    /// Duration defines the time period for which the Limit specified above applies.
    pub duration: i64,
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Duration defines the time uni
    /// Possible values are: "second", "minute", "hour", "day"
    pub unit: RateLimitPolicyOverridesLimitsRatesUnit,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyOverridesLimitsRatesUnit {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "day")]
    Day,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RateLimitPolicyOverridesLimitsWhen {
    /// The binary operator to be applied to the content fetched from the selector
    /// Possible values are: "eq" (equal to), "neq" (not equal to)
    pub operator: RateLimitPolicyOverridesLimitsWhenOperator,
    /// Selector defines one item from the well known selectors
    /// TODO Document properly "Well-known selector" https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    pub selector: String,
    /// The value of reference for the comparison.
    pub value: String,
}

/// WhenCondition defines semantics for matching an HTTP request based on conditions
/// https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io/v1.HTTPRouteSpec
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyOverridesLimitsWhenOperator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
    #[serde(rename = "startswith")]
    Startswith,
    #[serde(rename = "endswith")]
    Endswith,
    #[serde(rename = "incl")]
    Incl,
    #[serde(rename = "excl")]
    Excl,
    #[serde(rename = "matches")]
    Matches,
}

/// Rules to apply as overrides. Override all policy rules lower in the hierarchy. Can be overridden by less specific policy overrides.
/// Use one of: defaults, overrides, or bare set of policy rules (implicit defaults).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RateLimitPolicyOverridesStrategy {
    #[serde(rename = "atomic")]
    Atomic,
    #[serde(rename = "merge")]
    Merge,
}

/// Reference to the object to which this policy applies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyTargetRef {
    /// Group is the group of the target resource.
    pub group: String,
    /// Kind is kind of the target resource.
    pub kind: String,
    /// Name is the name of the target resource.
    pub name: String,
    /// SectionName is the name of a section within the target resource. When
    /// unspecified, this targetRef targets the entire resource. In the following
    /// resources, SectionName is interpreted as the following:
    /// 
    /// 
    /// * Gateway: Listener name
    /// * HTTPRoute: HTTPRouteRule name
    /// * Service: Port name
    /// 
    /// 
    /// If a SectionName is specified, but does not exist on the targeted object,
    /// the Policy must fail to attach, and the policy implementation should record
    /// a `ResolvedRefs` or similar Condition in the Policy's status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyStatus {
    /// Represents the observations of a foo's current state.
    /// Known .status.conditions.type are: "Available"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration reflects the generation of the most recently observed spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

