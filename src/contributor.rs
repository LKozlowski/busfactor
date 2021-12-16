use octocrab::{Octocrab, Page, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub contributions: u64,
}

#[derive(Debug, Serialize)]
pub struct ContributorParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl ContributorParameters {
    pub fn new() -> Self {
        Self {
            per_page: None,
            page: None,
        }
    }

    pub fn per_page(mut self, value: u8) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn page(mut self, value: u32) -> Self {
        self.page = Some(value);
        self
    }
}

#[async_trait::async_trait]
pub trait ContributorsExt {
    async fn list_repo_contributors(
        &self,
        owner: &str,
        repository: &str,
        parameters: &ContributorParameters,
    ) -> Result<Page<Contributor>>;
}

#[async_trait::async_trait]
impl ContributorsExt for Octocrab {
    async fn list_repo_contributors(
        &self,
        owner: &str,
        repository: &str,
        parameters: &ContributorParameters,
    ) -> Result<Page<Contributor>> {
        self.get(
            format!("/repos/{}/{}/contributors", owner, repository),
            Some(parameters),
        )
        .await
    }
}
