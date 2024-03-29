# Rust API client for line_membership

This document describes LINE Official Account Membership API.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.1
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `line_membership` and add the following to `Cargo.toml` under `[dependencies]`:

```
line_membership = { path = "./line_membership" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.line.me*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*MembershipApi* | [**get_membership_list**](docs/MembershipApi.md#get_membership_list) | **Get** /membership/v1/list | 
*MembershipApi* | [**get_membership_subscription**](docs/MembershipApi.md#get_membership_subscription) | **Get** /membership/v1/subscription/{userId} | 


## Documentation For Models

 - [ErrorResponse](docs/ErrorResponse.md)
 - [GetMembershipSubscriptionResponse](docs/GetMembershipSubscriptionResponse.md)
 - [Membership](docs/Membership.md)
 - [MembershipListResponse](docs/MembershipListResponse.md)
 - [MembershipMeta](docs/MembershipMeta.md)
 - [MembershipUser](docs/MembershipUser.md)
 - [Subscription](docs/Subscription.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



