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

//! # LINE Bot SDK for Rust
//!
//! A Rust SDK for the [LINE Messaging API](https://developers.line.biz/en/docs/messaging-api/overview/).
//!
//! This crate provides a high-level client ([`client::LINE`]) that wraps all LINE API modules,
//! along with webhook signature validation and web framework integrations.
//!
//! ## Quick Start
//!
//! ```no_run
//! use line_bot_sdk_rust::client::LINE;
//!
//! let line = LINE::new("YOUR_CHANNEL_ACCESS_TOKEN".to_string());
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `rocket_support` | Enables [`support::rocket::Signature`] extractor for the Rocket framework |
//! | `actix_support` | Enables [`support::actix::Signature`] extractor for the actix-web framework |
//! | `axum_support` | Enables [`support::axum::Signature`] extractor for the axum framework |
//!
//! ## Modules
//!
//! - [`client`] - LINE API client that bundles all API modules
//! - [`parser`] - Webhook signature validation
//! - [`support`] - Web framework integrations (feature-gated)
//!
//! The following LINE API modules are re-exported from their respective crates:
//!
//! - [`line_messaging_api`] - Send messages, manage rich menus, etc.
//! - [`line_webhook`] - Webhook event types and models
//! - [`line_channel_access_token`] - Issue and revoke channel access tokens
//! - [`line_insight`] - Retrieve message delivery and friend statistics
//! - [`line_liff`] - Manage LIFF apps
//! - [`line_manage_audience`] - Create and manage audiences
//! - [`line_module`] - LINE module operations
//! - [`line_module_attach`] - LINE module attach operations
//! - [`line_shop`] - LINE Shop API

// line-openapi modules
pub mod line_channel_access_token;
pub mod line_insight;
pub mod line_liff;
pub mod line_manage_audience;
pub mod line_messaging_api;
pub mod line_module;
pub mod line_module_attach;
pub mod line_shop;
pub mod line_webhook;

pub mod client;
pub mod parser;
pub mod support;
