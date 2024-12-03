// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct LogoutInput {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub access_token: ::std::option::Option<::std::string::String>,
}
impl LogoutInput {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn access_token(&self) -> ::std::option::Option<&str> {
        self.access_token.as_deref()
    }
}
impl ::std::fmt::Debug for LogoutInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LogoutInput");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl LogoutInput {
    /// Creates a new builder-style object to manufacture [`LogoutInput`](crate::operation::logout::LogoutInput).
    pub fn builder() -> crate::operation::logout::builders::LogoutInputBuilder {
        crate::operation::logout::builders::LogoutInputBuilder::default()
    }
}

/// A builder for [`LogoutInput`](crate::operation::logout::LogoutInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct LogoutInputBuilder {
    pub(crate) access_token: ::std::option::Option<::std::string::String>,
}
impl LogoutInputBuilder {
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    /// This field is required.
    pub fn access_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.access_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn set_access_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.access_token = input;
        self
    }
    /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    pub fn get_access_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.access_token
    }
    /// Consumes the builder and constructs a [`LogoutInput`](crate::operation::logout::LogoutInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::logout::LogoutInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::logout::LogoutInput {
            access_token: self.access_token,
        })
    }
}
impl ::std::fmt::Debug for LogoutInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("LogoutInputBuilder");
        formatter.field("access_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
