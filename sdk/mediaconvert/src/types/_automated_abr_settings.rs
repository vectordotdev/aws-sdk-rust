// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Use automated ABR to have MediaConvert set up the renditions in your ABR package for you automatically, based on characteristics of your input video. This feature optimizes video quality while minimizing the overall size of your ABR package.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutomatedAbrSettings {
    /// Optional. The maximum target bit rate used in your automated ABR stack. Use this value to set an upper limit on the bandwidth consumed by the highest-quality rendition. This is the rendition that is delivered to viewers with the fastest internet connections. If you don't specify a value, MediaConvert uses 8,000,000 (8 mb/s) by default.
    #[doc(hidden)]
    pub max_abr_bitrate: ::std::option::Option<i32>,
    /// Optional. The maximum number of renditions that MediaConvert will create in your automated ABR stack. The number of renditions is determined automatically, based on analysis of each job, but will never exceed this limit. When you set this to Auto in the console, which is equivalent to excluding it from your JSON job specification, MediaConvert defaults to a limit of 15.
    #[doc(hidden)]
    pub max_renditions: ::std::option::Option<i32>,
    /// Optional. The minimum target bitrate used in your automated ABR stack. Use this value to set a lower limit on the bitrate of video delivered to viewers with slow internet connections. If you don't specify a value, MediaConvert uses 600,000 (600 kb/s) by default.
    #[doc(hidden)]
    pub min_abr_bitrate: ::std::option::Option<i32>,
    /// Optional. Use Automated ABR rules to specify restrictions for the rendition sizes MediaConvert will create in your ABR stack. You can use these rules if your ABR workflow has specific rendition size requirements, but you still want MediaConvert to optimize for video quality and overall file size.
    #[doc(hidden)]
    pub rules: ::std::option::Option<::std::vec::Vec<crate::types::AutomatedAbrRule>>,
}
impl AutomatedAbrSettings {
    /// Optional. The maximum target bit rate used in your automated ABR stack. Use this value to set an upper limit on the bandwidth consumed by the highest-quality rendition. This is the rendition that is delivered to viewers with the fastest internet connections. If you don't specify a value, MediaConvert uses 8,000,000 (8 mb/s) by default.
    pub fn max_abr_bitrate(&self) -> ::std::option::Option<i32> {
        self.max_abr_bitrate
    }
    /// Optional. The maximum number of renditions that MediaConvert will create in your automated ABR stack. The number of renditions is determined automatically, based on analysis of each job, but will never exceed this limit. When you set this to Auto in the console, which is equivalent to excluding it from your JSON job specification, MediaConvert defaults to a limit of 15.
    pub fn max_renditions(&self) -> ::std::option::Option<i32> {
        self.max_renditions
    }
    /// Optional. The minimum target bitrate used in your automated ABR stack. Use this value to set a lower limit on the bitrate of video delivered to viewers with slow internet connections. If you don't specify a value, MediaConvert uses 600,000 (600 kb/s) by default.
    pub fn min_abr_bitrate(&self) -> ::std::option::Option<i32> {
        self.min_abr_bitrate
    }
    /// Optional. Use Automated ABR rules to specify restrictions for the rendition sizes MediaConvert will create in your ABR stack. You can use these rules if your ABR workflow has specific rendition size requirements, but you still want MediaConvert to optimize for video quality and overall file size.
    pub fn rules(&self) -> ::std::option::Option<&[crate::types::AutomatedAbrRule]> {
        self.rules.as_deref()
    }
}
impl AutomatedAbrSettings {
    /// Creates a new builder-style object to manufacture [`AutomatedAbrSettings`](crate::types::AutomatedAbrSettings).
    pub fn builder() -> crate::types::builders::AutomatedAbrSettingsBuilder {
        crate::types::builders::AutomatedAbrSettingsBuilder::default()
    }
}

/// A builder for [`AutomatedAbrSettings`](crate::types::AutomatedAbrSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AutomatedAbrSettingsBuilder {
    pub(crate) max_abr_bitrate: ::std::option::Option<i32>,
    pub(crate) max_renditions: ::std::option::Option<i32>,
    pub(crate) min_abr_bitrate: ::std::option::Option<i32>,
    pub(crate) rules: ::std::option::Option<::std::vec::Vec<crate::types::AutomatedAbrRule>>,
}
impl AutomatedAbrSettingsBuilder {
    /// Optional. The maximum target bit rate used in your automated ABR stack. Use this value to set an upper limit on the bandwidth consumed by the highest-quality rendition. This is the rendition that is delivered to viewers with the fastest internet connections. If you don't specify a value, MediaConvert uses 8,000,000 (8 mb/s) by default.
    pub fn max_abr_bitrate(mut self, input: i32) -> Self {
        self.max_abr_bitrate = ::std::option::Option::Some(input);
        self
    }
    /// Optional. The maximum target bit rate used in your automated ABR stack. Use this value to set an upper limit on the bandwidth consumed by the highest-quality rendition. This is the rendition that is delivered to viewers with the fastest internet connections. If you don't specify a value, MediaConvert uses 8,000,000 (8 mb/s) by default.
    pub fn set_max_abr_bitrate(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_abr_bitrate = input;
        self
    }
    /// Optional. The maximum number of renditions that MediaConvert will create in your automated ABR stack. The number of renditions is determined automatically, based on analysis of each job, but will never exceed this limit. When you set this to Auto in the console, which is equivalent to excluding it from your JSON job specification, MediaConvert defaults to a limit of 15.
    pub fn max_renditions(mut self, input: i32) -> Self {
        self.max_renditions = ::std::option::Option::Some(input);
        self
    }
    /// Optional. The maximum number of renditions that MediaConvert will create in your automated ABR stack. The number of renditions is determined automatically, based on analysis of each job, but will never exceed this limit. When you set this to Auto in the console, which is equivalent to excluding it from your JSON job specification, MediaConvert defaults to a limit of 15.
    pub fn set_max_renditions(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_renditions = input;
        self
    }
    /// Optional. The minimum target bitrate used in your automated ABR stack. Use this value to set a lower limit on the bitrate of video delivered to viewers with slow internet connections. If you don't specify a value, MediaConvert uses 600,000 (600 kb/s) by default.
    pub fn min_abr_bitrate(mut self, input: i32) -> Self {
        self.min_abr_bitrate = ::std::option::Option::Some(input);
        self
    }
    /// Optional. The minimum target bitrate used in your automated ABR stack. Use this value to set a lower limit on the bitrate of video delivered to viewers with slow internet connections. If you don't specify a value, MediaConvert uses 600,000 (600 kb/s) by default.
    pub fn set_min_abr_bitrate(mut self, input: ::std::option::Option<i32>) -> Self {
        self.min_abr_bitrate = input;
        self
    }
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// Optional. Use Automated ABR rules to specify restrictions for the rendition sizes MediaConvert will create in your ABR stack. You can use these rules if your ABR workflow has specific rendition size requirements, but you still want MediaConvert to optimize for video quality and overall file size.
    pub fn rules(mut self, input: crate::types::AutomatedAbrRule) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = ::std::option::Option::Some(v);
        self
    }
    /// Optional. Use Automated ABR rules to specify restrictions for the rendition sizes MediaConvert will create in your ABR stack. You can use these rules if your ABR workflow has specific rendition size requirements, but you still want MediaConvert to optimize for video quality and overall file size.
    pub fn set_rules(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AutomatedAbrRule>>,
    ) -> Self {
        self.rules = input;
        self
    }
    /// Consumes the builder and constructs a [`AutomatedAbrSettings`](crate::types::AutomatedAbrSettings).
    pub fn build(self) -> crate::types::AutomatedAbrSettings {
        crate::types::AutomatedAbrSettings {
            max_abr_bitrate: self.max_abr_bitrate,
            max_renditions: self.max_renditions,
            min_abr_bitrate: self.min_abr_bitrate,
            rules: self.rules,
        }
    }
}
