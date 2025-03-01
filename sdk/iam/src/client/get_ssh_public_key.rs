// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSSHPublicKey`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl ::std::convert::Into<String>)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::set_user_name): <p>The name of the IAM user associated with the SSH public key.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`ssh_public_key_id(impl ::std::convert::Into<String>)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::ssh_public_key_id) / [`set_ssh_public_key_id(Option<String>)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::set_ssh_public_key_id): <p>The unique identifier for the SSH public key.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    ///   - [`encoding(EncodingType)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::encoding) / [`set_encoding(Option<EncodingType>)`](crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::set_encoding): <p>Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use <code>SSH</code>. To retrieve the public key in PEM format, use <code>PEM</code>.</p>
    /// - On success, responds with [`GetSshPublicKeyOutput`](crate::operation::get_ssh_public_key::GetSshPublicKeyOutput) with field(s):
    ///   - [`ssh_public_key(Option<SshPublicKey>)`](crate::operation::get_ssh_public_key::GetSshPublicKeyOutput::ssh_public_key): <p>A structure containing details about the SSH public key.</p>
    /// - On failure, responds with [`SdkError<GetSSHPublicKeyError>`](crate::operation::get_ssh_public_key::GetSSHPublicKeyError)
    pub fn get_ssh_public_key(
        &self,
    ) -> crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder {
        crate::operation::get_ssh_public_key::builders::GetSSHPublicKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
