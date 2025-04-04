// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/apigatewayv2-controller/apigatewayv2.services.k8s.aws/v1alpha1/routes.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RouteSpec defines the desired state of Route.
/// 
/// Represents a route.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apigatewayv2.services.k8s.aws", version = "v1alpha1", kind = "Route", plural = "routes")]
#[kube(namespaced)]
#[kube(status = "RouteStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RouteSpec {
    /// The API identifier.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiID")]
    pub api_id: Option<String>,
    /// Specifies whether an API key is required for the route. Supported only for
    /// WebSocket APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiKeyRequired")]
    pub api_key_required: Option<bool>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiRef")]
    pub api_ref: Option<RouteApiRef>,
    /// The authorization scopes supported by this route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizationScopes")]
    pub authorization_scopes: Option<Vec<String>>,
    /// The authorization type for the route. For WebSocket APIs, valid values are
    /// NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for
    /// using a Lambda authorizer For HTTP APIs, valid values are NONE for open access,
    /// JWT for using JSON Web Tokens, AWS_IAM for using AWS IAM permissions, and
    /// CUSTOM for using a Lambda authorizer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizationType")]
    pub authorization_type: Option<String>,
    /// The identifier of the Authorizer resource to be associated with this route.
    /// The authorizer identifier is generated by API Gateway when you created the
    /// authorizer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizerID")]
    pub authorizer_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authorizerRef")]
    pub authorizer_ref: Option<RouteAuthorizerRef>,
    /// The model selection expression for the route. Supported only for WebSocket
    /// APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelSelectionExpression")]
    pub model_selection_expression: Option<String>,
    /// The operation name for the route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationName")]
    pub operation_name: Option<String>,
    /// The request models for the route. Supported only for WebSocket APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestModels")]
    pub request_models: Option<BTreeMap<String, String>>,
    /// The request parameters for the route. Supported only for WebSocket APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestParameters")]
    pub request_parameters: Option<BTreeMap<String, RouteRequestParameters>>,
    /// The route key for the route.
    #[serde(rename = "routeKey")]
    pub route_key: String,
    /// The route response selection expression for the route. Supported only for
    /// WebSocket APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeResponseSelectionExpression")]
    pub route_response_selection_expression: Option<String>,
    /// The target for the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<RouteTargetRef>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteApiRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RouteApiRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteApiRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteAuthorizerRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RouteAuthorizerRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteAuthorizerRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The request parameters for the route. Supported only for WebSocket APIs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteRequestParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteTargetRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<RouteTargetRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteTargetRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// RouteStatus defines the observed state of Route
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<RouteStatusAckResourceMetadata>,
    /// Specifies whether a route is managed by API Gateway. If you created an API
    /// using quick create, the $default route is managed by API Gateway. You can't
    /// modify the $default route key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGatewayManaged")]
    pub api_gateway_managed: Option<bool>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The route ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeID")]
    pub route_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RouteStatusAckResourceMetadata {
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

