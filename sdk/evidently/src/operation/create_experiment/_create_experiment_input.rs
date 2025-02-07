// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateExperimentInput {
    /// <p>The name or ARN of the project that you want to create the new experiment in.</p>
    #[doc(hidden)]
    pub project: ::std::option::Option<::std::string::String>,
    /// <p>A name for the new experiment.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>An optional description of the experiment.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>An array of structures that describe the configuration of each feature variation used in the experiment.</p>
    #[doc(hidden)]
    pub treatments: ::std::option::Option<::std::vec::Vec<crate::types::TreatmentConfig>>,
    /// <p>An array of structures that defines the metrics used for the experiment, and whether a higher or lower value for each metric is the goal.</p>
    #[doc(hidden)]
    pub metric_goals: ::std::option::Option<::std::vec::Vec<crate::types::MetricGoalConfig>>,
    /// <p>When Evidently assigns a particular user session to an experiment, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the experiment name as the <code>randomizationSalt</code>.</p>
    #[doc(hidden)]
    pub randomization_salt: ::std::option::Option<::std::string::String>,
    /// <p>The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent. The available audience is the total audience minus the audience that you have allocated to overrides or current launches of this feature.</p>
    /// <p>This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the available audience.</p>
    #[doc(hidden)]
    pub sampling_rate: ::std::option::Option<i64>,
    /// <p>A structure that contains the configuration of which variation to use as the "control" version. tThe "control" version is used for comparison with other variations. This structure also specifies how much experiment traffic is allocated to each variation.</p>
    #[doc(hidden)]
    pub online_ab_config: ::std::option::Option<crate::types::OnlineAbConfig>,
    /// <p>Specifies an audience <i>segment</i> to use in the experiment. When a segment is used in an experiment, only user sessions that match the segment pattern are used in the experiment.</p>
    #[doc(hidden)]
    pub segment: ::std::option::Option<::std::string::String>,
    /// <p>Assigns one or more tags (key-value pairs) to the experiment.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with an experiment.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateExperimentInput {
    /// <p>The name or ARN of the project that you want to create the new experiment in.</p>
    pub fn project(&self) -> ::std::option::Option<&str> {
        self.project.as_deref()
    }
    /// <p>A name for the new experiment.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>An optional description of the experiment.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>An array of structures that describe the configuration of each feature variation used in the experiment.</p>
    pub fn treatments(&self) -> ::std::option::Option<&[crate::types::TreatmentConfig]> {
        self.treatments.as_deref()
    }
    /// <p>An array of structures that defines the metrics used for the experiment, and whether a higher or lower value for each metric is the goal.</p>
    pub fn metric_goals(&self) -> ::std::option::Option<&[crate::types::MetricGoalConfig]> {
        self.metric_goals.as_deref()
    }
    /// <p>When Evidently assigns a particular user session to an experiment, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the experiment name as the <code>randomizationSalt</code>.</p>
    pub fn randomization_salt(&self) -> ::std::option::Option<&str> {
        self.randomization_salt.as_deref()
    }
    /// <p>The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent. The available audience is the total audience minus the audience that you have allocated to overrides or current launches of this feature.</p>
    /// <p>This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the available audience.</p>
    pub fn sampling_rate(&self) -> ::std::option::Option<i64> {
        self.sampling_rate
    }
    /// <p>A structure that contains the configuration of which variation to use as the "control" version. tThe "control" version is used for comparison with other variations. This structure also specifies how much experiment traffic is allocated to each variation.</p>
    pub fn online_ab_config(&self) -> ::std::option::Option<&crate::types::OnlineAbConfig> {
        self.online_ab_config.as_ref()
    }
    /// <p>Specifies an audience <i>segment</i> to use in the experiment. When a segment is used in an experiment, only user sessions that match the segment pattern are used in the experiment.</p>
    pub fn segment(&self) -> ::std::option::Option<&str> {
        self.segment.as_deref()
    }
    /// <p>Assigns one or more tags (key-value pairs) to the experiment.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with an experiment.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl CreateExperimentInput {
    /// Creates a new builder-style object to manufacture [`CreateExperimentInput`](crate::operation::create_experiment::CreateExperimentInput).
    pub fn builder() -> crate::operation::create_experiment::builders::CreateExperimentInputBuilder
    {
        crate::operation::create_experiment::builders::CreateExperimentInputBuilder::default()
    }
}

/// A builder for [`CreateExperimentInput`](crate::operation::create_experiment::CreateExperimentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateExperimentInputBuilder {
    pub(crate) project: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) treatments: ::std::option::Option<::std::vec::Vec<crate::types::TreatmentConfig>>,
    pub(crate) metric_goals: ::std::option::Option<::std::vec::Vec<crate::types::MetricGoalConfig>>,
    pub(crate) randomization_salt: ::std::option::Option<::std::string::String>,
    pub(crate) sampling_rate: ::std::option::Option<i64>,
    pub(crate) online_ab_config: ::std::option::Option<crate::types::OnlineAbConfig>,
    pub(crate) segment: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateExperimentInputBuilder {
    /// <p>The name or ARN of the project that you want to create the new experiment in.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ARN of the project that you want to create the new experiment in.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project = input;
        self
    }
    /// <p>A name for the new experiment.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the new experiment.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>An optional description of the experiment.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional description of the experiment.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `treatments`.
    ///
    /// To override the contents of this collection use [`set_treatments`](Self::set_treatments).
    ///
    /// <p>An array of structures that describe the configuration of each feature variation used in the experiment.</p>
    pub fn treatments(mut self, input: crate::types::TreatmentConfig) -> Self {
        let mut v = self.treatments.unwrap_or_default();
        v.push(input);
        self.treatments = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of structures that describe the configuration of each feature variation used in the experiment.</p>
    pub fn set_treatments(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TreatmentConfig>>,
    ) -> Self {
        self.treatments = input;
        self
    }
    /// Appends an item to `metric_goals`.
    ///
    /// To override the contents of this collection use [`set_metric_goals`](Self::set_metric_goals).
    ///
    /// <p>An array of structures that defines the metrics used for the experiment, and whether a higher or lower value for each metric is the goal.</p>
    pub fn metric_goals(mut self, input: crate::types::MetricGoalConfig) -> Self {
        let mut v = self.metric_goals.unwrap_or_default();
        v.push(input);
        self.metric_goals = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of structures that defines the metrics used for the experiment, and whether a higher or lower value for each metric is the goal.</p>
    pub fn set_metric_goals(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MetricGoalConfig>>,
    ) -> Self {
        self.metric_goals = input;
        self
    }
    /// <p>When Evidently assigns a particular user session to an experiment, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the experiment name as the <code>randomizationSalt</code>.</p>
    pub fn randomization_salt(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.randomization_salt = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When Evidently assigns a particular user session to an experiment, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the experiment name as the <code>randomizationSalt</code>.</p>
    pub fn set_randomization_salt(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.randomization_salt = input;
        self
    }
    /// <p>The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent. The available audience is the total audience minus the audience that you have allocated to overrides or current launches of this feature.</p>
    /// <p>This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the available audience.</p>
    pub fn sampling_rate(mut self, input: i64) -> Self {
        self.sampling_rate = ::std::option::Option::Some(input);
        self
    }
    /// <p>The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent. The available audience is the total audience minus the audience that you have allocated to overrides or current launches of this feature.</p>
    /// <p>This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the available audience.</p>
    pub fn set_sampling_rate(mut self, input: ::std::option::Option<i64>) -> Self {
        self.sampling_rate = input;
        self
    }
    /// <p>A structure that contains the configuration of which variation to use as the "control" version. tThe "control" version is used for comparison with other variations. This structure also specifies how much experiment traffic is allocated to each variation.</p>
    pub fn online_ab_config(mut self, input: crate::types::OnlineAbConfig) -> Self {
        self.online_ab_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that contains the configuration of which variation to use as the "control" version. tThe "control" version is used for comparison with other variations. This structure also specifies how much experiment traffic is allocated to each variation.</p>
    pub fn set_online_ab_config(
        mut self,
        input: ::std::option::Option<crate::types::OnlineAbConfig>,
    ) -> Self {
        self.online_ab_config = input;
        self
    }
    /// <p>Specifies an audience <i>segment</i> to use in the experiment. When a segment is used in an experiment, only user sessions that match the segment pattern are used in the experiment.</p>
    pub fn segment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.segment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies an audience <i>segment</i> to use in the experiment. When a segment is used in an experiment, only user sessions that match the segment pattern are used in the experiment.</p>
    pub fn set_segment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.segment = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Assigns one or more tags (key-value pairs) to the experiment.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with an experiment.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Assigns one or more tags (key-value pairs) to the experiment.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>
    /// <p>You can associate as many as 50 tags with an experiment.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateExperimentInput`](crate::operation::create_experiment::CreateExperimentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_experiment::CreateExperimentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_experiment::CreateExperimentInput {
            project: self.project,
            name: self.name,
            description: self.description,
            treatments: self.treatments,
            metric_goals: self.metric_goals,
            randomization_salt: self.randomization_salt,
            sampling_rate: self.sampling_rate,
            online_ab_config: self.online_ab_config,
            segment: self.segment,
            tags: self.tags,
        })
    }
}
