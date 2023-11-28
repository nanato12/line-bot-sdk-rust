/*
* Copyright 2023 nanato12
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::module_attach::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`attach_module`]
#[derive(Clone, Debug)]
pub struct AttachModuleParams {
    /// authorization_code
    pub grant_type: Option<String>,
    /// Authorization code received from the LINE Platform.
    pub code: Option<String>,
    /// Specify the redirect_uri specified in the URL for authentication and authorization.
    pub redirect_uri: Option<String>,
    /// Specify when using PKCE (Proof Key for Code Exchange) defined in the OAuth 2.0 extension specification as a countermeasure against authorization code interception attacks.
    pub code_verifier: Option<String>,
    /// Instead of using Authorization header, you can use this parameter to specify the channel ID of the module channel. You can find the channel ID of the module channel in the LINE Developers Console. 
    pub client_id: Option<String>,
    /// Instead of using Authorization header, you can use this parameter to specify the channel secret of the module channel. You can find the channel secret of the module channel in the LINE Developers Console. 
    pub client_secret: Option<String>,
    /// If you specified a value for region in the URL for authentication and authorization, specify the same value. 
    pub region: Option<String>,
    /// If you specified a value for basic_search_id in the URL for authentication and authorization, specify the same value.
    pub basic_search_id: Option<String>,
    /// If you specified a value for scope in the URL for authentication and authorization, specify the same value.
    pub scope: Option<String>,
    /// If you specified a value for brand_type in the URL for authentication and authorization, specify the same value.
    pub brand_type: Option<String>
}


/// struct for typed errors of method [`attach_module`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachModuleError {
    UnknownValue(serde_json::Value),
}


/// Attach by operation of the module channel provider
pub async fn attach_module(configuration: &configuration::Configuration, params: AttachModuleParams) -> Result<crate::module_attach::models::AttachModuleResponse, Error<AttachModuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let grant_type = params.grant_type;
    let code = params.code;
    let redirect_uri = params.redirect_uri;
    let code_verifier = params.code_verifier;
    let client_id = params.client_id;
    let client_secret = params.client_secret;
    let region = params.region;
    let basic_search_id = params.basic_search_id;
    let scope = params.scope;
    let brand_type = params.brand_type;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/module/auth/v1/token", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = grant_type {
        local_var_form_params.insert("grant_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code {
        local_var_form_params.insert("code", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = redirect_uri {
        local_var_form_params.insert("redirect_uri", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = code_verifier {
        local_var_form_params.insert("code_verifier", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_id {
        local_var_form_params.insert("client_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = client_secret {
        local_var_form_params.insert("client_secret", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = region {
        local_var_form_params.insert("region", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = basic_search_id {
        local_var_form_params.insert("basic_search_id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = scope {
        local_var_form_params.insert("scope", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = brand_type {
        local_var_form_params.insert("brand_type", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AttachModuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

