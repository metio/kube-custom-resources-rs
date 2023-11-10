// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/cert-manager/cert-manager/acme.cert-manager.io/v1/orders.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "acme.cert-manager.io", version = "v1", kind = "Order", plural = "orders")]
#[kube(namespaced)]
#[kube(status = "OrderStatus")]
#[kube(schema = "disabled")]
pub struct OrderSpec {
    /// CommonName is the common name as specified on the DER encoded CSR. If specified, this value must also be present in `dnsNames` or `ipAddresses`. This field must match the corresponding field on the DER encoded CSR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    /// DNSNames is a list of DNS names that should be included as part of the Order validation process. This field must match the corresponding field on the DER encoded CSR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsNames")]
    pub dns_names: Option<Vec<String>>,
    /// Duration is the duration for the not after date for the requested certificate. this is set on order creation as pe the ACME spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// IPAddresses is a list of IP addresses that should be included as part of the Order validation process. This field must match the corresponding field on the DER encoded CSR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddresses")]
    pub ip_addresses: Option<Vec<String>>,
    /// IssuerRef references a properly configured ACME-type Issuer which should be used to create this Order. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Order will be marked as failed.
    #[serde(rename = "issuerRef")]
    pub issuer_ref: OrderIssuerRef,
    /// Certificate signing request bytes in DER encoding. This will be used when finalizing the order. This field must be set on the order.
    pub request: String,
}

/// IssuerRef references a properly configured ACME-type Issuer which should be used to create this Order. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Order will be marked as failed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderIssuerRef {
    /// Group of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource being referred to.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderStatus {
    /// Authorizations contains data returned from the ACME server on what authorizations must be completed in order to validate the DNS names specified on the Order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizations: Option<Vec<OrderStatusAuthorizations>>,
    /// Certificate is a copy of the PEM encoded certificate for this Order. This field will be populated after the order has been successfully finalized with the ACME server, and the order has transitioned to the 'valid' state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// FailureTime stores the time that this order failed. This is used to influence garbage collection and back-off.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureTime")]
    pub failure_time: Option<String>,
    /// FinalizeURL of the Order. This is used to obtain certificates for this order once it has been completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizeURL")]
    pub finalize_url: Option<String>,
    /// Reason optionally provides more information about a why the order is in the current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// State contains the current state of this Order resource. States 'success' and 'expired' are 'final'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderStatusState>,
    /// URL of the Order. This will initially be empty when the resource is first created. The Order controller will populate this field when the Order is first processed. This field will be immutable after it is initially set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// ACMEAuthorization contains data returned from the ACME server on an authorization that must be completed in order validate a DNS name on an ACME Order resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderStatusAuthorizations {
    /// Challenges specifies the challenge types offered by the ACME server. One of these challenge types will be selected when validating the DNS name and an appropriate Challenge resource will be created to perform the ACME challenge process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Vec<OrderStatusAuthorizationsChallenges>>,
    /// Identifier is the DNS name to be validated as part of this authorization
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// InitialState is the initial state of the ACME authorization when first fetched from the ACME server. If an Authorization is already 'valid', the Order controller will not create a Challenge resource for the authorization. This will occur when working with an ACME server that enables 'authz reuse' (such as Let's Encrypt's production endpoint). If not set and 'identifier' is set, the state is assumed to be pending and a Challenge will be created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialState")]
    pub initial_state: Option<OrderStatusAuthorizationsInitialState>,
    /// URL is the URL of the Authorization that must be completed
    pub url: String,
    /// Wildcard will be true if this authorization is for a wildcard DNS name. If this is true, the identifier will be the *non-wildcard* version of the DNS name. For example, if '*.example.com' is the DNS name being validated, this field will be 'true' and the 'identifier' field will be 'example.com'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<bool>,
}

/// Challenge specifies a challenge offered by the ACME server for an Order. An appropriate Challenge resource can be created to perform the ACME challenge process.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderStatusAuthorizationsChallenges {
    /// Token is the token that must be presented for this challenge. This is used to compute the 'key' that must also be presented.
    pub token: String,
    /// Type is the type of challenge being offered, e.g. 'http-01', 'dns-01', 'tls-sni-01', etc. This is the raw value retrieved from the ACME server. Only 'http-01' and 'dns-01' are supported by cert-manager, other values will be ignored.
    #[serde(rename = "type")]
    pub r#type: String,
    /// URL is the URL of this challenge. It can be used to retrieve additional metadata about the Challenge from the ACME server.
    pub url: String,
}

/// ACMEAuthorization contains data returned from the ACME server on an authorization that must be completed in order validate a DNS name on an ACME Order resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderStatusAuthorizationsInitialState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "errored")]
    Errored,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OrderStatusState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "errored")]
    Errored,
}

