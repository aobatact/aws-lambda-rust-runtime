use http::{HeaderMap, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::custom_serde::{
    deserialize_headers, deserialize_lambda_map, http_method, serialize_headers, serialize_multi_value_headers,
};

/// `VpcLambdaRequest` contains data coming from VPC Lattice.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLambdaRequestV2 {
    /// Version is expected to be `"2.0"`
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(with = "http_method")]
    pub http_method: Method,
    #[serde(deserialize_with = "deserialize_headers", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub query_string_parameters: HashMap<String, String>,
    pub request_context: VpcLambdaRequestContext,
    pub body: Option<String>,
    pub is_base64_encoded: bool,
}

/// `VpcLambdaRequestContext` contains request context.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLambdaRequestContext {
    #[serde(default)]
    pub service_netork_arn: Option<String>,
    #[serde(default)]
    pub service_arn: Option<String>,
    #[serde(default)]
    pub target_group_arn: Option<String>,
    pub identity: VpcLambdaRequestIdentity,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub time_epoch: Option<String>,
}

/// `VpcLambdaRequestIdentity` contains the identity information.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VpcLambdaRequestIdentity {
    #[serde(default)]
    pub source_vpc_arn: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub principal: Option<String>,
    #[serde(default)]
    pub principal_org_id: Option<String>,
    #[serde(default)]
    pub session_name: Option<String>,
    #[serde(default)]
    pub x509_issuer_ou: Option<String>,
    #[serde(default)]
    pub x509_san_dns: Option<String>,
    #[serde(default)]
    pub x509_san_name_cn: Option<String>,
    #[serde(default)]
    pub x509_san_uri: Option<String>,
    #[serde(default)]
    pub x509_subject_cn: Option<String>,
}

/// `VpcLambdaResponse` configures the response to be returned to VPC Lattice service.
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlResponse {
    pub is_base64_encoded: bool,
    pub status_code: i64,
    #[serde(default)]
    pub status_description: Option<String>,
    #[serde(deserialize_with = "deserialize_headers", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    pub headers: HeaderMap,
    #[serde(default)]
    pub body: Option<String>,
}
