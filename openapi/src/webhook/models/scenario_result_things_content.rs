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
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenarioResultThingsContent {
    /// Type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Device ID of the device that has been linked with LINE.
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "result")]
    pub result: Box<crate::webhook::models::ScenarioResult>,
}

impl ScenarioResultThingsContent {
    pub fn new(r#type: String, device_id: String, result: crate::webhook::models::ScenarioResult) -> ScenarioResultThingsContent {
        ScenarioResultThingsContent {
            r#type,
            device_id,
            result: Box::new(result),
        }
    }
}


