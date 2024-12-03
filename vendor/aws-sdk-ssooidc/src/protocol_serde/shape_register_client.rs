// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_register_client_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::register_client::RegisterClientError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::operation::register_client::RegisterClientError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidClientMetadataException" => crate::operation::register_client::RegisterClientError::InvalidClientMetadataException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidClientMetadataExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_client_metadata_exception::de_invalid_client_metadata_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRedirectUriException" => crate::operation::register_client::RegisterClientError::InvalidRedirectUriException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRedirectUriExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_redirect_uri_exception::de_invalid_redirect_uri_exception_json_err(_response_body, output)
                        .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRequestException" => crate::operation::register_client::RegisterClientError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidScopeException" => crate::operation::register_client::RegisterClientError::InvalidScopeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidScopeExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_scope_exception::de_invalid_scope_exception_json_err(_response_body, output)
                    .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedGrantTypeException" => crate::operation::register_client::RegisterClientError::UnsupportedGrantTypeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedGrantTypeExceptionBuilder::default();
                output = crate::protocol_serde::shape_unsupported_grant_type_exception::de_unsupported_grant_type_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::register_client::RegisterClientError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_client_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_client::RegisterClientOutput, crate::operation::register_client::RegisterClientError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::register_client::builders::RegisterClientOutputBuilder::default();
        output = crate::protocol_serde::shape_register_client::de_register_client(_response_body, output)
            .map_err(crate::operation::register_client::RegisterClientError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_register_client_input(
    input: &crate::operation::register_client::RegisterClientInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_register_client_input::ser_register_client_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_register_client(
    value: &[u8],
    mut builder: crate::operation::register_client::builders::RegisterClientOutputBuilder,
) -> Result<crate::operation::register_client::builders::RegisterClientOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "authorizationEndpoint" => {
                    builder = builder.set_authorization_endpoint(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "clientId" => {
                    builder = builder.set_client_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "clientIdIssuedAt" => {
                    builder = builder.set_client_id_issued_at(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "clientSecret" => {
                    builder = builder.set_client_secret(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "clientSecretExpiresAt" => {
                    builder = builder.set_client_secret_expires_at(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "tokenEndpoint" => {
                    builder = builder.set_token_endpoint(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
