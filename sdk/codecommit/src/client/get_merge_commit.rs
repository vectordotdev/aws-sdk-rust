// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMergeCommit`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`repository_name(impl ::std::convert::Into<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::set_repository_name): <p>The name of the repository that contains the merge commit about which you want to get information.</p>
    ///   - [`source_commit_specifier(impl ::std::convert::Into<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::source_commit_specifier) / [`set_source_commit_specifier(Option<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::set_source_commit_specifier): <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    ///   - [`destination_commit_specifier(impl ::std::convert::Into<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::destination_commit_specifier) / [`set_destination_commit_specifier(Option<String>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::set_destination_commit_specifier): <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    ///   - [`conflict_detail_level(ConflictDetailLevelTypeEnum)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::conflict_detail_level) / [`set_conflict_detail_level(Option<ConflictDetailLevelTypeEnum>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::set_conflict_detail_level): <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    ///   - [`conflict_resolution_strategy(ConflictResolutionStrategyTypeEnum)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::conflict_resolution_strategy) / [`set_conflict_resolution_strategy(Option<ConflictResolutionStrategyTypeEnum>)`](crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::set_conflict_resolution_strategy): <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    /// - On success, responds with [`GetMergeCommitOutput`](crate::operation::get_merge_commit::GetMergeCommitOutput) with field(s):
    ///   - [`source_commit_id(Option<String>)`](crate::operation::get_merge_commit::GetMergeCommitOutput::source_commit_id): <p>The commit ID of the source commit specifier that was used in the merge evaluation.</p>
    ///   - [`destination_commit_id(Option<String>)`](crate::operation::get_merge_commit::GetMergeCommitOutput::destination_commit_id): <p>The commit ID of the destination commit specifier that was used in the merge evaluation.</p>
    ///   - [`base_commit_id(Option<String>)`](crate::operation::get_merge_commit::GetMergeCommitOutput::base_commit_id): <p>The commit ID of the merge base.</p>
    ///   - [`merged_commit_id(Option<String>)`](crate::operation::get_merge_commit::GetMergeCommitOutput::merged_commit_id): <p>The commit ID for the merge commit created when the source branch was merged into the destination branch. If the fast-forward merge strategy was used, there is no merge commit.</p>
    /// - On failure, responds with [`SdkError<GetMergeCommitError>`](crate::operation::get_merge_commit::GetMergeCommitError)
    pub fn get_merge_commit(
        &self,
    ) -> crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder {
        crate::operation::get_merge_commit::builders::GetMergeCommitFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
