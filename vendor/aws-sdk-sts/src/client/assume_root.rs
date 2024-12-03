// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssumeRoot`](crate::operation::assume_root::builders::AssumeRootFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_principal(impl Into<String>)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::target_principal) / [`set_target_principal(Option<String>)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::set_target_principal):<br>required: **true**<br><p>The member account principal ARN or account ID.</p><br>
    ///   - [`task_policy_arn(PolicyDescriptorType)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::task_policy_arn) / [`set_task_policy_arn(Option<PolicyDescriptorType>)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::set_task_policy_arn):<br>required: **true**<br><p>The identity based policy that scopes the session to the privileged tasks that can be performed. You can use one of following Amazon Web Services managed policies to scope root session actions. You can add additional customer managed policies to further limit the permissions for the root session.</p> <ul>  <li>   <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/security-iam-awsmanpol.html#security-iam-awsmanpol-IAMAuditRootUserCredentials">IAMAuditRootUserCredentials</a></p></li>  <li>   <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/security-iam-awsmanpol.html#security-iam-awsmanpol-IAMCreateRootUserPassword">IAMCreateRootUserPassword</a></p></li>  <li>   <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/security-iam-awsmanpol.html#security-iam-awsmanpol-IAMDeleteRootUserCredentials">IAMDeleteRootUserCredentials</a></p></li>  <li>   <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/security-iam-awsmanpol.html#security-iam-awsmanpol-S3UnlockBucketPolicy">S3UnlockBucketPolicy</a></p></li>  <li>   <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/security-iam-awsmanpol.html#security-iam-awsmanpol-SQSUnlockQueuePolicy">SQSUnlockQueuePolicy</a></p></li> </ul><br>
    ///   - [`duration_seconds(i32)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::operation::assume_root::builders::AssumeRootFluentBuilder::set_duration_seconds):<br>required: **false**<br><p>The duration, in seconds, of the privileged session. The value can range from 0 seconds up to the maximum session duration of 900 seconds (15 minutes). If you specify a value higher than this setting, the operation fails.</p> <p>By default, the value is set to <code>900</code> seconds.</p><br>
    /// - On success, responds with [`AssumeRootOutput`](crate::operation::assume_root::AssumeRootOutput) with field(s):
    ///   - [`credentials(Option<Credentials>)`](crate::operation::assume_root::AssumeRootOutput::credentials): <p>The temporary security credentials, which include an access key ID, a secret access key, and a security token.</p><note>  <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p> </note>
    ///   - [`source_identity(Option<String>)`](crate::operation::assume_root::AssumeRootOutput::source_identity): <p>The source identity specified by the principal that is calling the <code>AssumeRoot</code> operation.</p> <p>You can use the <code>aws:SourceIdentity</code> condition key to control access based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p> <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    /// - On failure, responds with [`SdkError<AssumeRootError>`](crate::operation::assume_root::AssumeRootError)
    pub fn assume_root(&self) -> crate::operation::assume_root::builders::AssumeRootFluentBuilder {
        crate::operation::assume_root::builders::AssumeRootFluentBuilder::new(self.handle.clone())
    }
}