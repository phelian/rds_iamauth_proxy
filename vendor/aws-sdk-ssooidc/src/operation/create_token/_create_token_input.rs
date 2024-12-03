// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateTokenInput {
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub client_id: ::std::option::Option<::std::string::String>,
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub client_secret: ::std::option::Option<::std::string::String>,
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub grant_type: ::std::option::Option<::std::string::String>,
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub device_code: ::std::option::Option<::std::string::String>,
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub refresh_token: ::std::option::Option<::std::string::String>,
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub redirect_uri: ::std::option::Option<::std::string::String>,
    /// <p>Used only when calling this API for the Authorization Code grant type. This value is generated by the client and presented to validate the original code challenge value the client passed at authorization time.</p>
    pub code_verifier: ::std::option::Option<::std::string::String>,
}
impl CreateTokenInput {
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn client_secret(&self) -> ::std::option::Option<&str> {
        self.client_secret.as_deref()
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn grant_type(&self) -> ::std::option::Option<&str> {
        self.grant_type.as_deref()
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn device_code(&self) -> ::std::option::Option<&str> {
        self.device_code.as_deref()
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn refresh_token(&self) -> ::std::option::Option<&str> {
        self.refresh_token.as_deref()
    }
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.scope.is_none()`.
    pub fn scope(&self) -> &[::std::string::String] {
        self.scope.as_deref().unwrap_or_default()
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn redirect_uri(&self) -> ::std::option::Option<&str> {
        self.redirect_uri.as_deref()
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value is generated by the client and presented to validate the original code challenge value the client passed at authorization time.</p>
    pub fn code_verifier(&self) -> ::std::option::Option<&str> {
        self.code_verifier.as_deref()
    }
}
impl ::std::fmt::Debug for CreateTokenInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateTokenInput");
        formatter.field("client_id", &self.client_id);
        formatter.field("client_secret", &"*** Sensitive Data Redacted ***");
        formatter.field("grant_type", &self.grant_type);
        formatter.field("device_code", &self.device_code);
        formatter.field("code", &self.code);
        formatter.field("refresh_token", &"*** Sensitive Data Redacted ***");
        formatter.field("scope", &self.scope);
        formatter.field("redirect_uri", &self.redirect_uri);
        formatter.field("code_verifier", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CreateTokenInput {
    /// Creates a new builder-style object to manufacture [`CreateTokenInput`](crate::operation::create_token::CreateTokenInput).
    pub fn builder() -> crate::operation::create_token::builders::CreateTokenInputBuilder {
        crate::operation::create_token::builders::CreateTokenInputBuilder::default()
    }
}

/// A builder for [`CreateTokenInput`](crate::operation::create_token::CreateTokenInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateTokenInputBuilder {
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_secret: ::std::option::Option<::std::string::String>,
    pub(crate) grant_type: ::std::option::Option<::std::string::String>,
    pub(crate) device_code: ::std::option::Option<::std::string::String>,
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) refresh_token: ::std::option::Option<::std::string::String>,
    pub(crate) scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) redirect_uri: ::std::option::Option<::std::string::String>,
    pub(crate) code_verifier: ::std::option::Option<::std::string::String>,
}
impl CreateTokenInputBuilder {
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    /// This field is required.
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_id
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    /// This field is required.
    pub fn client_secret(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_secret = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn set_client_secret(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_secret = input;
        self
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn get_client_secret(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_secret
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    /// This field is required.
    pub fn grant_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grant_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn set_grant_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grant_type = input;
        self
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn get_grant_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.grant_type
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn device_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn set_device_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_code = input;
        self
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn get_device_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.device_code
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.code
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn refresh_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.refresh_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn set_refresh_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.refresh_token = input;
        self
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn get_refresh_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.refresh_token
    }
    /// Appends an item to `scope`.
    ///
    /// To override the contents of this collection use [`set_scope`](Self::set_scope).
    ///
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn scope(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.scope.unwrap_or_default();
        v.push(input.into());
        self.scope = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn set_scope(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.scope = input;
        self
    }
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn get_scope(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.scope
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.redirect_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn set_redirect_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.redirect_uri = input;
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn get_redirect_uri(&self) -> &::std::option::Option<::std::string::String> {
        &self.redirect_uri
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value is generated by the client and presented to validate the original code challenge value the client passed at authorization time.</p>
    pub fn code_verifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code_verifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value is generated by the client and presented to validate the original code challenge value the client passed at authorization time.</p>
    pub fn set_code_verifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code_verifier = input;
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value is generated by the client and presented to validate the original code challenge value the client passed at authorization time.</p>
    pub fn get_code_verifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.code_verifier
    }
    /// Consumes the builder and constructs a [`CreateTokenInput`](crate::operation::create_token::CreateTokenInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_token::CreateTokenInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_token::CreateTokenInput {
            client_id: self.client_id,
            client_secret: self.client_secret,
            grant_type: self.grant_type,
            device_code: self.device_code,
            code: self.code,
            refresh_token: self.refresh_token,
            scope: self.scope,
            redirect_uri: self.redirect_uri,
            code_verifier: self.code_verifier,
        })
    }
}
impl ::std::fmt::Debug for CreateTokenInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateTokenInputBuilder");
        formatter.field("client_id", &self.client_id);
        formatter.field("client_secret", &"*** Sensitive Data Redacted ***");
        formatter.field("grant_type", &self.grant_type);
        formatter.field("device_code", &self.device_code);
        formatter.field("code", &self.code);
        formatter.field("refresh_token", &"*** Sensitive Data Redacted ***");
        formatter.field("scope", &self.scope);
        formatter.field("redirect_uri", &self.redirect_uri);
        formatter.field("code_verifier", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
