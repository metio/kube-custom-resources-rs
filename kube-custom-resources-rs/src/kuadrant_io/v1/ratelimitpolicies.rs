// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Kuadrant/kuadrant-operator/kuadrant.io/v1/ratelimitpolicies.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuadrant.io", version = "v1", kind = "RateLimitPolicy", plural = "ratelimitpolicies")]
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
    /// Overall conditions for the policy to be enforced.
    /// If omitted, the policy will be enforced at all requests to the protected routes.
    /// If present, all conditions must match for the policy to be enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyWhen>>,
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
    /// Overall conditions for the policy to be enforced.
    /// If omitted, the policy will be enforced at all requests to the protected routes.
    /// If present, all conditions must match for the policy to be enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyDefaultsWhen>>,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsLimits {
    /// Counters defines additional rate limit counters based on CEL expressions which can reference well known selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<RateLimitPolicyDefaultsLimitsCounters>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyDefaultsLimitsRates>>,
    /// When holds a list of "limit-level" `Predicate`s
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyDefaultsLimitsWhen>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsLimitsCounters {
    /// Expression defines one CEL expression
    /// Expression can use well known attributes
    /// Attributes: https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/advanced/attributes
    /// Well-known selectors: https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    /// They are named by a dot-separated path (e.g. request.path)
    /// Example: "request.path" -> The path portion of the URL
    pub expression: String,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsLimitsRates {
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Window defines the time period for which the Limit specified above applies.
    pub window: String,
}

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsLimitsWhen {
    pub predicate: String,
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

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyDefaultsWhen {
    pub predicate: String,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyLimits {
    /// Counters defines additional rate limit counters based on CEL expressions which can reference well known selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<RateLimitPolicyLimitsCounters>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyLimitsRates>>,
    /// When holds a list of "limit-level" `Predicate`s
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyLimitsWhen>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyLimitsCounters {
    /// Expression defines one CEL expression
    /// Expression can use well known attributes
    /// Attributes: https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/advanced/attributes
    /// Well-known selectors: https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    /// They are named by a dot-separated path (e.g. request.path)
    /// Example: "request.path" -> The path portion of the URL
    pub expression: String,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyLimitsRates {
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Window defines the time period for which the Limit specified above applies.
    pub window: String,
}

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyLimitsWhen {
    pub predicate: String,
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
    /// Overall conditions for the policy to be enforced.
    /// If omitted, the policy will be enforced at all requests to the protected routes.
    /// If present, all conditions must match for the policy to be enforced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyOverridesWhen>>,
}

/// Limits holds the struct of limits indexed by a unique name
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesLimits {
    /// Counters defines additional rate limit counters based on CEL expressions which can reference well known selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<RateLimitPolicyOverridesLimitsCounters>>,
    /// Rates holds the list of limit rates
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rates: Option<Vec<RateLimitPolicyOverridesLimitsRates>>,
    /// When holds a list of "limit-level" `Predicate`s
    /// Called also "soft" conditions as route selectors must also match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<RateLimitPolicyOverridesLimitsWhen>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesLimitsCounters {
    /// Expression defines one CEL expression
    /// Expression can use well known attributes
    /// Attributes: https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/advanced/attributes
    /// Well-known selectors: https://github.com/Kuadrant/architecture/blob/main/rfcs/0001-rlp-v2.md#well-known-selectors
    /// They are named by a dot-separated path (e.g. request.path)
    /// Example: "request.path" -> The path portion of the URL
    pub expression: String,
}

/// Rate defines the actual rate limit that will be used when there is a match
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesLimitsRates {
    /// Limit defines the max value allowed for a given period of time
    pub limit: i64,
    /// Window defines the time period for which the Limit specified above applies.
    pub window: String,
}

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesLimitsWhen {
    pub predicate: String,
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

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyOverridesWhen {
    pub predicate: String,
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
    /// * Gateway: Listener name
    /// * HTTPRoute: HTTPRouteRule name
    /// * Service: Port name
    /// 
    /// If a SectionName is specified, but does not exist on the targeted object,
    /// the Policy must fail to attach, and the policy implementation should record
    /// a `ResolvedRefs` or similar Condition in the Policy's status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

/// Predicate defines one CEL expression that must be evaluated to bool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RateLimitPolicyWhen {
    pub predicate: String,
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

