// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutLexicon`](crate::operation::put_lexicon::builders::PutLexiconFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::put_lexicon::builders::PutLexiconFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_lexicon::builders::PutLexiconFluentBuilder::set_name): <p>Name of the lexicon. The name must follow the regular express format [0-9A-Za-z]{1,20}. That is, the name is a case-sensitive alphanumeric string up to 20 characters long. </p>
    ///   - [`content(impl ::std::convert::Into<String>)`](crate::operation::put_lexicon::builders::PutLexiconFluentBuilder::content) / [`set_content(Option<String>)`](crate::operation::put_lexicon::builders::PutLexiconFluentBuilder::set_content): <p>Content of the PLS lexicon as string data.</p>
    /// - On success, responds with [`PutLexiconOutput`](crate::operation::put_lexicon::PutLexiconOutput)
    /// - On failure, responds with [`SdkError<PutLexiconError>`](crate::operation::put_lexicon::PutLexiconError)
    pub fn put_lexicon(&self) -> crate::operation::put_lexicon::builders::PutLexiconFluentBuilder {
        crate::operation::put_lexicon::builders::PutLexiconFluentBuilder::new(self.handle.clone())
    }
}
