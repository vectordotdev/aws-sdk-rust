// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeServers`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`server_name(impl ::std::convert::Into<String>)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::server_name) / [`set_server_name(Option<String>)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::set_server_name): <p>Describes the server with the specified ServerName.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::set_next_token): <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    ///   - [`max_results(i32)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_servers::builders::DescribeServersFluentBuilder::set_max_results): <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    /// - On success, responds with [`DescribeServersOutput`](crate::operation::describe_servers::DescribeServersOutput) with field(s):
    ///   - [`servers(Option<Vec<Server>>)`](crate::operation::describe_servers::DescribeServersOutput::servers): <p>Contains the response to a <code>DescribeServers</code> request.</p>  <p> <i>For Chef Automate servers:</i> If <code>DescribeServersResponse$Servers$EngineAttributes</code> includes CHEF_MAJOR_UPGRADE_AVAILABLE, you can upgrade the Chef Automate server to Chef Automate 2. To be eligible for upgrade, a server running Chef Automate 1 must have had at least one successful maintenance run after November 1, 2019.</p>  <p> <i>For Puppet servers:</i> <code>DescribeServersResponse$Servers$EngineAttributes</code> contains the following two responses:</p>  <ul>   <li> <p> <code>PUPPET_API_CA_CERT</code>, the PEM-encoded CA certificate that is used by the Puppet API over TCP port number 8140. The CA certificate is also used to sign node certificates.</p> </li>   <li> <p> <code>PUPPET_API_CRL</code>, a certificate revocation list. The certificate revocation list is for internal maintenance purposes only. For more information about the Puppet certificate revocation list, see <a href="https://puppet.com/docs/puppet/5.5/man/certificate_revocation_list.html">Man Page: puppet certificate_revocation_list</a> in the Puppet documentation.</p> </li>  </ul>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_servers::DescribeServersOutput::next_token): <p>This is not currently implemented for <code>DescribeServers</code> requests. </p>
    /// - On failure, responds with [`SdkError<DescribeServersError>`](crate::operation::describe_servers::DescribeServersError)
    pub fn describe_servers(
        &self,
    ) -> crate::operation::describe_servers::builders::DescribeServersFluentBuilder {
        crate::operation::describe_servers::builders::DescribeServersFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
