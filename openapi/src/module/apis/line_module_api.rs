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

use super::{configuration, Error};
use crate::module::apis::ResponseContent;

/// struct for passing parameters to the method [`acquire_chat_control`]
#[derive(Clone, Debug)]
pub struct AcquireChatControlParams {
    /// The `userId`, `roomId`, or `groupId`
    pub chat_id: String,
    pub acquire_chat_control_request: Option<crate::module::models::AcquireChatControlRequest>,
}

/// struct for passing parameters to the method [`detach_module`]
#[derive(Clone, Debug)]
pub struct DetachModuleParams {
    pub detach_module_request: Option<crate::module::models::DetachModuleRequest>,
}

/// struct for passing parameters to the method [`get_modules`]
#[derive(Clone, Debug)]
pub struct GetModulesParams {
    /// Value of the continuation token found in the next property of the JSON object returned in the response. If you can't get all basic information about the bots in one request, include this parameter to get the remaining array.
    pub start: Option<String>,
    /// Specify the maximum number of bots that you get basic information from. The default value is 100. Max value: 100
    pub limit: Option<i32>,
}

/// struct for passing parameters to the method [`release_chat_control`]
#[derive(Clone, Debug)]
pub struct ReleaseChatControlParams {
    /// The `userId`, `roomId`, or `groupId`
    pub chat_id: String,
}

/// struct for typed errors of method [`acquire_chat_control`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcquireChatControlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`detach_module`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachModuleError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_modules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetModulesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`release_chat_control`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReleaseChatControlError {
    UnknownValue(serde_json::Value),
}

/// If the Standby Channel wants to take the initiative (Chat Control), it calls the Acquire Control API. The channel that was previously an Active Channel will automatically switch to a Standby Channel.
pub async fn acquire_chat_control(
    configuration: &configuration::Configuration,
    params: AcquireChatControlParams,
) -> Result<(), Error<AcquireChatControlError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let chat_id = params.chat_id;
    let acquire_chat_control_request = params.acquire_chat_control_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/bot/chat/{chatId}/control/acquire",
        local_var_configuration.base_path,
        chatId = crate::module::apis::urlencode(chat_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&acquire_chat_control_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AcquireChatControlError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The module channel admin calls the Detach API to detach the module channel from a LINE Official Account.
pub async fn detach_module(
    configuration: &configuration::Configuration,
    params: DetachModuleParams,
) -> Result<(), Error<DetachModuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let detach_module_request = params.detach_module_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/bot/channel/detach",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&detach_module_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DetachModuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a list of basic information about the bots of multiple LINE Official Accounts that have attached module channels.
pub async fn get_modules(
    configuration: &configuration::Configuration,
    params: GetModulesParams,
) -> Result<crate::module::models::GetModulesResponse, Error<GetModulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let start = params.start;
    let limit = params.limit;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/bot/list", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = start {
        local_var_req_builder =
            local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetModulesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// To return the initiative (Chat Control) of Active Channel to Primary Channel, call the Release Control API.
pub async fn release_chat_control(
    configuration: &configuration::Configuration,
    params: ReleaseChatControlParams,
) -> Result<(), Error<ReleaseChatControlError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let chat_id = params.chat_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/bot/chat/{chatId}/control/release",
        local_var_configuration.base_path,
        chatId = crate::module::apis::urlencode(chat_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ReleaseChatControlError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}