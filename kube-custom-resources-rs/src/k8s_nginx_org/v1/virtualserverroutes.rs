// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/kubernetes-ingress/k8s.nginx.org/v1/virtualserverroutes.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// VirtualServerRouteSpec is the spec of the VirtualServerRoute resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.nginx.org", version = "v1", kind = "VirtualServerRoute", plural = "virtualserverroutes")]
#[kube(namespaced)]
#[kube(status = "VirtualServerRouteStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VirtualServerRouteSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subroutes: Option<Vec<VirtualServerRouteSubroutes>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<VirtualServerRouteUpstreams>>,
}

/// Route defines a route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutes {
    /// Action defines an action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<VirtualServerRouteSubroutesAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dos: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorPages")]
    pub error_pages: Option<Vec<VirtualServerRouteSubroutesErrorPages>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "location-snippets")]
    pub location_snippets: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<VirtualServerRouteSubroutesMatches>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<VirtualServerRouteSubroutesPolicies>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<VirtualServerRouteSubroutesSplits>>,
}

/// Action defines an action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<String>,
    /// ActionProxy defines a proxy in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<VirtualServerRouteSubroutesActionProxy>,
    /// ActionRedirect defines a redirect in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServerRouteSubroutesActionRedirect>,
    /// ActionReturn defines a return in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "return")]
    pub r#return: Option<VirtualServerRouteSubroutesActionReturn>,
}

/// ActionProxy defines a proxy in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionProxy {
    /// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<VirtualServerRouteSubroutesActionProxyRequestHeaders>,
    /// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    pub response_headers: Option<VirtualServerRouteSubroutesActionProxyResponseHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rewritePath")]
    pub rewrite_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionProxyRequestHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<VirtualServerRouteSubroutesActionProxyRequestHeadersSet>>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionProxyRequestHeadersSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionProxyResponseHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<VirtualServerRouteSubroutesActionProxyResponseHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<Vec<String>>,
}

/// AddHeader defines an HTTP Header with an optional Always field to use with the add_header NGINX directive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionProxyResponseHeadersAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ActionRedirect defines a redirect in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ActionReturn defines a return in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteSubroutesActionReturnHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesActionReturnHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ErrorPage defines an ErrorPage in a Route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesErrorPages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<i64>>,
    /// ErrorPageRedirect defines a redirect for an ErrorPage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServerRouteSubroutesErrorPagesRedirect>,
    /// ErrorPageReturn defines a return for an ErrorPage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "return")]
    pub r#return: Option<VirtualServerRouteSubroutesErrorPagesReturn>,
}

/// ErrorPageRedirect defines a redirect for an ErrorPage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesErrorPagesRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ErrorPageReturn defines a return for an ErrorPage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesErrorPagesReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteSubroutesErrorPagesReturnHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesErrorPagesReturnHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Match defines a match.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatches {
    /// Action defines an action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<VirtualServerRouteSubroutesMatchesAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VirtualServerRouteSubroutesMatchesConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<VirtualServerRouteSubroutesMatchesSplits>>,
}

/// Action defines an action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<String>,
    /// ActionProxy defines a proxy in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<VirtualServerRouteSubroutesMatchesActionProxy>,
    /// ActionRedirect defines a redirect in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServerRouteSubroutesMatchesActionRedirect>,
    /// ActionReturn defines a return in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "return")]
    pub r#return: Option<VirtualServerRouteSubroutesMatchesActionReturn>,
}

/// ActionProxy defines a proxy in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionProxy {
    /// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<VirtualServerRouteSubroutesMatchesActionProxyRequestHeaders>,
    /// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    pub response_headers: Option<VirtualServerRouteSubroutesMatchesActionProxyResponseHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rewritePath")]
    pub rewrite_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionProxyRequestHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<VirtualServerRouteSubroutesMatchesActionProxyRequestHeadersSet>>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionProxyRequestHeadersSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionProxyResponseHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<VirtualServerRouteSubroutesMatchesActionProxyResponseHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<Vec<String>>,
}

/// AddHeader defines an HTTP Header with an optional Always field to use with the add_header NGINX directive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionProxyResponseHeadersAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ActionRedirect defines a redirect in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ActionReturn defines a return in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteSubroutesMatchesActionReturnHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesActionReturnHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Condition defines a condition in a MatchRule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub argument: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
}

/// Split defines a split.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplits {
    /// Action defines an action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<VirtualServerRouteSubroutesMatchesSplitsAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Action defines an action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<String>,
    /// ActionProxy defines a proxy in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<VirtualServerRouteSubroutesMatchesSplitsActionProxy>,
    /// ActionRedirect defines a redirect in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServerRouteSubroutesMatchesSplitsActionRedirect>,
    /// ActionReturn defines a return in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "return")]
    pub r#return: Option<VirtualServerRouteSubroutesMatchesSplitsActionReturn>,
}

/// ActionProxy defines a proxy in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionProxy {
    /// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<VirtualServerRouteSubroutesMatchesSplitsActionProxyRequestHeaders>,
    /// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    pub response_headers: Option<VirtualServerRouteSubroutesMatchesSplitsActionProxyResponseHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rewritePath")]
    pub rewrite_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionProxyRequestHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<VirtualServerRouteSubroutesMatchesSplitsActionProxyRequestHeadersSet>>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionProxyRequestHeadersSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionProxyResponseHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<VirtualServerRouteSubroutesMatchesSplitsActionProxyResponseHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<Vec<String>>,
}

/// AddHeader defines an HTTP Header with an optional Always field to use with the add_header NGINX directive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionProxyResponseHeadersAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ActionRedirect defines a redirect in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ActionReturn defines a return in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteSubroutesMatchesSplitsActionReturnHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesMatchesSplitsActionReturnHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// PolicyReference references a policy by name and an optional namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesPolicies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Split defines a split.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplits {
    /// Action defines an action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<VirtualServerRouteSubroutesSplitsAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Action defines an action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<String>,
    /// ActionProxy defines a proxy in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<VirtualServerRouteSubroutesSplitsActionProxy>,
    /// ActionRedirect defines a redirect in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServerRouteSubroutesSplitsActionRedirect>,
    /// ActionReturn defines a return in an Action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "return")]
    pub r#return: Option<VirtualServerRouteSubroutesSplitsActionReturn>,
}

/// ActionProxy defines a proxy in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionProxy {
    /// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<VirtualServerRouteSubroutesSplitsActionProxyRequestHeaders>,
    /// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaders")]
    pub response_headers: Option<VirtualServerRouteSubroutesSplitsActionProxyResponseHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rewritePath")]
    pub rewrite_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
}

/// ProxyRequestHeaders defines the request headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionProxyRequestHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<VirtualServerRouteSubroutesSplitsActionProxyRequestHeadersSet>>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionProxyRequestHeadersSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ProxyResponseHeaders defines the response headers manipulation in an ActionProxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionProxyResponseHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<VirtualServerRouteSubroutesSplitsActionProxyResponseHeadersAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pass: Option<Vec<String>>,
}

/// AddHeader defines an HTTP Header with an optional Always field to use with the add_header NGINX directive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionProxyResponseHeadersAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ActionRedirect defines a redirect in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ActionReturn defines a return in an Action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionReturn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteSubroutesSplitsActionReturnHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteSubroutesSplitsActionReturnHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Upstream defines an upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPort")]
    pub backup_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "buffer-size")]
    pub buffer_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffering: Option<bool>,
    /// UpstreamBuffers defines Buffer Configuration for an Upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffers: Option<VirtualServerRouteUpstreamsBuffers>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "client-max-body-size")]
    pub client_max_body_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connect-timeout")]
    pub connect_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fail-timeout")]
    pub fail_timeout: Option<String>,
    /// HealthCheck defines the parameters for active Upstream HealthChecks.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<VirtualServerRouteUpstreamsHealthCheck>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keepalive: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lb-method")]
    pub lb_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "max-conns")]
    pub max_conns: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "max-fails")]
    pub max_fails: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "next-upstream")]
    pub next_upstream: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "next-upstream-timeout")]
    pub next_upstream_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "next-upstream-tries")]
    pub next_upstream_tries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ntlm: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// UpstreamQueue defines Queue Configuration for an Upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<VirtualServerRouteUpstreamsQueue>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "read-timeout")]
    pub read_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "send-timeout")]
    pub send_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// SessionCookie defines the parameters for session persistence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionCookie")]
    pub session_cookie: Option<VirtualServerRouteUpstreamsSessionCookie>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "slow-start")]
    pub slow_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subselector: Option<BTreeMap<String, String>>,
    /// UpstreamTLS defines a TLS configuration for an Upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualServerRouteUpstreamsTls>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "use-cluster-ip")]
    pub use_cluster_ip: Option<bool>,
}

/// UpstreamBuffers defines Buffer Configuration for an Upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsBuffers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// HealthCheck defines the parameters for active Upstream HealthChecks.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connect-timeout")]
    pub connect_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fails: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcService")]
    pub grpc_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcStatus")]
    pub grpc_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<VirtualServerRouteUpstreamsHealthCheckHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepalive-time")]
    pub keepalive_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "read-timeout")]
    pub read_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "send-timeout")]
    pub send_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMatch")]
    pub status_match: Option<String>,
    /// UpstreamTLS defines a TLS configuration for an Upstream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualServerRouteUpstreamsHealthCheckTls>,
}

/// Header defines an HTTP Header.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsHealthCheckHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// UpstreamTLS defines a TLS configuration for an Upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsHealthCheckTls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// UpstreamQueue defines Queue Configuration for an Upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsQueue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// SessionCookie defines the parameters for session persistence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsSessionCookie {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpOnly")]
    pub http_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub samesite: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}

/// UpstreamTLS defines a TLS configuration for an Upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteUpstreamsTls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// VirtualServerRouteStatus defines the status for the VirtualServerRoute resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalEndpoints")]
    pub external_endpoints: Option<Vec<VirtualServerRouteStatusExternalEndpoints>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "referencedBy")]
    pub referenced_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ExternalEndpoint defines the IP/ Hostname and ports used to connect to this resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualServerRouteStatusExternalEndpoints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<String>,
}

