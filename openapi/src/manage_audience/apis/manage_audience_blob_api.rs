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
use crate::manage_audience::apis::ResponseContent;

/// struct for passing parameters to the method [`add_user_ids_to_audience`]
#[derive(Clone, Debug)]
pub struct AddUserIdsToAudienceParams {
    /// A text file with one user ID or IFA entered per line. Specify text/plain as Content-Type. Max file number: 1 Max number: 1,500,000
    pub file: std::path::PathBuf,
    /// The audience ID.
    pub audience_group_id: Option<i64>,
    /// The description to register with the job
    pub upload_description: Option<String>,
}

/// struct for passing parameters to the method [`create_audience_for_uploading_user_ids`]
#[derive(Clone, Debug)]
pub struct CreateAudienceForUploadingUserIdsParams {
    /// A text file with one user ID or IFA entered per line. Specify text/plain as Content-Type. Max file number: 1 Max number: 1,500,000
    pub file: std::path::PathBuf,
    /// The audience's name. This is case-insensitive, meaning AUDIENCE and audience are considered identical. Max character limit: 120
    pub description: Option<String>,
    /// To specify recipients by IFAs: set `true`. To specify recipients by user IDs: set `false` or omit isIfaAudience property.
    pub is_ifa_audience: Option<bool>,
    /// The description to register for the job (in `jobs[].description`).
    pub upload_description: Option<String>,
}

/// struct for typed errors of method [`add_user_ids_to_audience`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserIdsToAudienceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_audience_for_uploading_user_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAudienceForUploadingUserIdsError {
    UnknownValue(serde_json::Value),
}

/// Add user IDs or Identifiers for Advertisers (IFAs) to an audience for uploading user IDs (by file).
pub async fn add_user_ids_to_audience(
    configuration: &configuration::Configuration,
    params: AddUserIdsToAudienceParams,
) -> Result<(), Error<AddUserIdsToAudienceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let _file = params.file;
    let audience_group_id = params.audience_group_id;
    let upload_description = params.upload_description;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/bot/audienceGroup/upload/byFile",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = audience_group_id {
        local_var_form = local_var_form.text("audienceGroupId", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = upload_description {
        local_var_form =
            local_var_form.text("uploadDescription", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'file' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddUserIdsToAudienceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create audience for uploading user IDs (by file).
pub async fn create_audience_for_uploading_user_ids(
    configuration: &configuration::Configuration,
    params: CreateAudienceForUploadingUserIdsParams,
) -> Result<
    crate::manage_audience::models::CreateAudienceGroupResponse,
    Error<CreateAudienceForUploadingUserIdsError>,
> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let _file = params.file;
    let description = params.description;
    let is_ifa_audience = params.is_ifa_audience;
    let upload_description = params.upload_description;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/bot/audienceGroup/upload/byFile",
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
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = description {
        local_var_form = local_var_form.text("description", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = is_ifa_audience {
        local_var_form = local_var_form.text("isIfaAudience", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = upload_description {
        local_var_form =
            local_var_form.text("uploadDescription", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'file' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAudienceForUploadingUserIdsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
