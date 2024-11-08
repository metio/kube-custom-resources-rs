// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/stackabletech/listener-operator/listeners.stackable.tech/v1alpha1/listenerclasses.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Defines a policy for how [Listeners](https://docs.stackable.tech/home/nightly/listener-operator/listener) should be exposed. Read the [ListenerClass documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass) for more information.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "listeners.stackable.tech", version = "v1alpha1", kind = "ListenerClass", plural = "listenerclasses")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ListenerClassSpec {
    /// Whether addresses should prefer using the IP address (`IP`) or the hostname (`Hostname`). Can also be set to `HostnameConservative`, which will use `IP` for `NodePort` service types, but `Hostname` for everything else.
    /// 
    /// The other type will be used if the preferred type is not available.
    /// 
    /// Defaults to `HostnameConservative`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredAddressType")]
    pub preferred_address_type: Option<ListenerClassPreferredAddressType>,
    /// Annotations that should be added to the Service object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAnnotations")]
    pub service_annotations: Option<BTreeMap<String, String>>,
    /// `externalTrafficPolicy` that should be set on the created [`Service`] objects.
    /// 
    /// The default is `Local` (in contrast to `Cluster`), as we aim to direct traffic to a node running the workload and we should keep testing that as the primary configuration. Cluster is a fallback option for providers that break Local mode (IONOS so far).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceExternalTrafficPolicy")]
    pub service_external_traffic_policy: Option<ListenerClassServiceExternalTrafficPolicy>,
    /// The method used to access the services.
    #[serde(rename = "serviceType")]
    pub service_type: ListenerClassServiceType,
}

/// Defines a policy for how [Listeners](https://docs.stackable.tech/home/nightly/listener-operator/listener) should be exposed. Read the [ListenerClass documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass) for more information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerClassPreferredAddressType {
    Hostname,
    #[serde(rename = "IP")]
    Ip,
    HostnameConservative,
}

/// Defines a policy for how [Listeners](https://docs.stackable.tech/home/nightly/listener-operator/listener) should be exposed. Read the [ListenerClass documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass) for more information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerClassServiceExternalTrafficPolicy {
    Cluster,
    Local,
}

/// Defines a policy for how [Listeners](https://docs.stackable.tech/home/nightly/listener-operator/listener) should be exposed. Read the [ListenerClass documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass) for more information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ListenerClassServiceType {
    NodePort,
    LoadBalancer,
    #[serde(rename = "ClusterIP")]
    ClusterIp,
}

