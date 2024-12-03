// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the provided context. This includes the signed and encrypted trusted context assertion and the context provider ARN from which the trusted context assertion was generated.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvidedContext {
    /// <p>The context provider ARN from which the trusted context assertion was generated.</p>
    pub provider_arn: ::std::option::Option<::std::string::String>,
    /// <p>The signed and encrypted trusted context assertion generated by the context provider. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
    pub context_assertion: ::std::option::Option<::std::string::String>,
}
impl ProvidedContext {
    /// <p>The context provider ARN from which the trusted context assertion was generated.</p>
    pub fn provider_arn(&self) -> ::std::option::Option<&str> {
        self.provider_arn.as_deref()
    }
    /// <p>The signed and encrypted trusted context assertion generated by the context provider. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
    pub fn context_assertion(&self) -> ::std::option::Option<&str> {
        self.context_assertion.as_deref()
    }
}
impl ProvidedContext {
    /// Creates a new builder-style object to manufacture [`ProvidedContext`](crate::types::ProvidedContext).
    pub fn builder() -> crate::types::builders::ProvidedContextBuilder {
        crate::types::builders::ProvidedContextBuilder::default()
    }
}

/// A builder for [`ProvidedContext`](crate::types::ProvidedContext).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProvidedContextBuilder {
    pub(crate) provider_arn: ::std::option::Option<::std::string::String>,
    pub(crate) context_assertion: ::std::option::Option<::std::string::String>,
}
impl ProvidedContextBuilder {
    /// <p>The context provider ARN from which the trusted context assertion was generated.</p>
    pub fn provider_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.provider_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The context provider ARN from which the trusted context assertion was generated.</p>
    pub fn set_provider_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.provider_arn = input;
        self
    }
    /// <p>The context provider ARN from which the trusted context assertion was generated.</p>
    pub fn get_provider_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.provider_arn
    }
    /// <p>The signed and encrypted trusted context assertion generated by the context provider. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
    pub fn context_assertion(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.context_assertion = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The signed and encrypted trusted context assertion generated by the context provider. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
    pub fn set_context_assertion(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.context_assertion = input;
        self
    }
    /// <p>The signed and encrypted trusted context assertion generated by the context provider. The trusted context assertion is signed and encrypted by Amazon Web Services STS.</p>
    pub fn get_context_assertion(&self) -> &::std::option::Option<::std::string::String> {
        &self.context_assertion
    }
    /// Consumes the builder and constructs a [`ProvidedContext`](crate::types::ProvidedContext).
    pub fn build(self) -> crate::types::ProvidedContext {
        crate::types::ProvidedContext {
            provider_arn: self.provider_arn,
            context_assertion: self.context_assertion,
        }
    }
}
