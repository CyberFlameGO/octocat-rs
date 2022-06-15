use crate::model::{prelude::*, repositories::nested::SimpleLicense};

/// Descending is the default.
///
/// -Desc variants: Descending order.
/// -Asc variants: Ascending order.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Sort {
    InteractionsDesc,
    InteractionsAsc,
    ReactionsDesc,
    ReactionsAsc,
    #[serde(rename = "reactions-+1")]
    ReactionsThumbsUp,
    #[serde(rename = "reactions--1")]
    ReactionsThumbsDown,
    ReactionsSmile,
    ReactionsTada,
    ReactionsHeart,
    AuthorDateDesc,
    AuthorDateAsc,
    CommitterDateDesc,
    CommitterDateAsc,
    UpdatedDesc,
    UpdatedAsc,
}

// TODO: Verify this because the docs suck apparently
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoSearchResultItem {
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: usize,
    pub node_id: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub notifications_url: String,
    pub owner: String,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub forks: usize,
    pub forks_count: usize,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub mirror_url: String,
    pub open_issues: usize,
    pub open_issues_count: usize,
    pub license: Option<SimpleLicense>,
    pub pushed_at: String,
    pub size: usize,
    pub ssh_url: String,
    pub stargazers_count: usize,
    pub svn_url: String,
    pub watchers: usize,
    pub watchers_count: usize,
    pub created_at: String,
    pub updated_at: String,
    pub score: usize,
}