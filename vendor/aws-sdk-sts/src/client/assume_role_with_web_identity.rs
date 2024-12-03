// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssumeRoleWithWebIdentity`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_arn(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_role_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p><note>  <p>Additional considerations apply to Amazon Cognito identity pools that assume <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies-cross-account-resource-access.html">cross-account IAM roles</a>. The trust policies of these roles must accept the <code>cognito-identity.amazonaws.com</code> service principal and must contain the <code>cognito-identity.amazonaws.com:aud</code> condition key to restrict role assumption to users from your intended identity pools. A policy that trusts Amazon Cognito identity pools without this condition creates a risk that a user from an unintended identity pool can assume the role. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/iam-roles.html#trust-policies"> Trust policies for IAM roles in Basic (Classic) authentication </a> in the <i>Amazon Cognito Developer Guide</i>.</p> </note><br>
    ///   - [`role_session_name(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::role_session_name) / [`set_role_session_name(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_role_session_name):<br>required: **true**<br><p>An identifier for the assumed role session. Typically, you pass the name or identifier that is associated with the user who is using your application. That way, the temporary security credentials that your application will use are associated with that user. This session name is included as part of the ARN and assumed role ID in the <code>AssumedRoleUser</code> response element.</p> <p>For security purposes, administrators can view this field in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/cloudtrail-integration.html#cloudtrail-integration_signin-tempcreds">CloudTrail logs</a> to help identify who performed an action in Amazon Web Services. Your administrator might require that you specify your user name as the session name when you assume the role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_iam-condition-keys.html#ck_rolesessionname"> <code>sts:RoleSessionName</code> </a>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p><br>
    ///   - [`web_identity_token(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::web_identity_token) / [`set_web_identity_token(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_web_identity_token):<br>required: **true**<br><p>The OAuth 2.0 access token or OpenID Connect ID token that is provided by the identity provider. Your application must get this token by authenticating the user who is using your application with a web identity provider before the application makes an <code>AssumeRoleWithWebIdentity</code> call. Timestamps in the token must be formatted as either an integer or a long integer. Only tokens with RSA algorithms (RS256) are supported.</p><br>
    ///   - [`provider_id(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::provider_id) / [`set_provider_id(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_provider_id):<br>required: **false**<br><p>The fully qualified host component of the domain name of the OAuth 2.0 identity provider. Do not specify this value for an OpenID Connect identity provider.</p> <p>Currently <code>www.amazon.com</code> and <code>graph.facebook.com</code> are the only supported identity providers for OAuth 2.0 access tokens. Do not include URL schemes and port numbers.</p> <p>Do not specify this value for OpenID Connect ID tokens.</p><br>
    ///   - [`policy_arns(PolicyDescriptorType)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::policy_arns) / [`set_policy_arns(Option<Vec::<PolicyDescriptorType>>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_policy_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p> <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p><note>  <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p> </note> <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p><br>
    ///   - [`policy(impl Into<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_policy):<br>required: **false**<br><p>An IAM policy in JSON format that you want to use as an inline session policy.</p> <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p> <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <p>For more information about role session permissions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session policies</a>.</p><note>  <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p> </note><br>
    ///   - [`duration_seconds(i32)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::set_duration_seconds):<br>required: **false**<br><p>The duration, in seconds, of the role session. The value can range from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p> <p>By default, the value is set to <code>3600</code> seconds.</p><note>  <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p> </note><br>
    /// - On success, responds with [`AssumeRoleWithWebIdentityOutput`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput) with field(s):
    ///   - [`credentials(Option<Credentials>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::credentials): <p>The temporary security credentials, which include an access key ID, a secret access key, and a security token.</p><note>  <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p> </note>
    ///   - [`subject_from_web_identity_token(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::subject_from_web_identity_token): <p>The unique user identifier that is returned by the identity provider. This identifier is associated with the <code>WebIdentityToken</code> that was submitted with the <code>AssumeRoleWithWebIdentity</code> call. The identifier is typically unique to the user and the application that acquired the <code>WebIdentityToken</code> (pairwise identifier). For OpenID Connect ID tokens, this field contains the value returned by the identity provider as the token's <code>sub</code> (Subject) claim.</p>
    ///   - [`assumed_role_user(Option<AssumedRoleUser>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::assumed_role_user): <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    ///   - [`packed_policy_size(Option<i32>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::packed_policy_size): <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    ///   - [`provider(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::provider): <p>The issuing authority of the web identity token presented. For OpenID Connect ID tokens, this contains the value of the <code>iss</code> field. For OAuth 2.0 access tokens, this contains the value of the <code>ProviderId</code> parameter that was passed in the <code>AssumeRoleWithWebIdentity</code> request.</p>
    ///   - [`audience(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::audience): <p>The intended audience (also known as client ID) of the web identity token. This is traditionally the client identifier issued to the application that requested the web identity token.</p>
    ///   - [`source_identity(Option<String>)`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityOutput::source_identity): <p>The value of the source identity that is returned in the JSON web token (JWT) from the identity provider.</p> <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html#id_roles_terms-and-concepts">chained role</a> sessions. You can configure your identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithWebIdentity</code>. You do this by adding a claim to the JSON web token. To learn more about OIDC tokens and claims, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-using-tokens-with-identity-providers.html">Using Tokens with User Pools</a> in the <i>Amazon Cognito Developer Guide</i>. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    /// - On failure, responds with [`SdkError<AssumeRoleWithWebIdentityError>`](crate::operation::assume_role_with_web_identity::AssumeRoleWithWebIdentityError)
    pub fn assume_role_with_web_identity(&self) -> crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder {
        crate::operation::assume_role_with_web_identity::builders::AssumeRoleWithWebIdentityFluentBuilder::new(self.handle.clone())
    }
}
