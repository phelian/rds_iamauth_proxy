// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetRoleCredentialsOutput {
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub role_credentials: ::std::option::Option<crate::types::RoleCredentials>,
    _request_id: Option<String>,
}
impl GetRoleCredentialsOutput {
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub fn role_credentials(&self) -> ::std::option::Option<&crate::types::RoleCredentials> {
        self.role_credentials.as_ref()
    }
}
impl ::std::fmt::Debug for GetRoleCredentialsOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoleCredentialsOutput");
        formatter.field("role_credentials", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for GetRoleCredentialsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetRoleCredentialsOutput {
    /// Creates a new builder-style object to manufacture [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput).
    pub fn builder() -> crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder {
        crate::operation::get_role_credentials::builders::GetRoleCredentialsOutputBuilder::default()
    }
}

/// A builder for [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct GetRoleCredentialsOutputBuilder {
    pub(crate) role_credentials: ::std::option::Option<crate::types::RoleCredentials>,
    _request_id: Option<String>,
}
impl GetRoleCredentialsOutputBuilder {
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub fn role_credentials(mut self, input: crate::types::RoleCredentials) -> Self {
        self.role_credentials = ::std::option::Option::Some(input);
        self
    }
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub fn set_role_credentials(mut self, input: ::std::option::Option<crate::types::RoleCredentials>) -> Self {
        self.role_credentials = input;
        self
    }
    /// <p>The credentials for the role that is assigned to the user.</p>
    pub fn get_role_credentials(&self) -> &::std::option::Option<crate::types::RoleCredentials> {
        &self.role_credentials
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetRoleCredentialsOutput`](crate::operation::get_role_credentials::GetRoleCredentialsOutput).
    pub fn build(self) -> crate::operation::get_role_credentials::GetRoleCredentialsOutput {
        crate::operation::get_role_credentials::GetRoleCredentialsOutput {
            role_credentials: self.role_credentials,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for GetRoleCredentialsOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetRoleCredentialsOutputBuilder");
        formatter.field("role_credentials", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
