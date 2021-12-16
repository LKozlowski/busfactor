use crate::contributor::{Contributor, ContributorParameters, ContributorsExt};
use octocrab::Octocrab;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::mpsc;

const BUS_FACTOR_THRESHOLD: f64 = 0.75;

#[derive(Debug)]
struct RepositoryBusFactor {
    project: String,
    user: String,
    bus_factor: f64,
}

fn calculate_bus_factor(contributors: Vec<Contributor>) -> f64 {
    if contributors.is_empty() {
        return 0.0;
    }
    let all_contribiutions: u64 = contributors.iter().map(|c| c.contributions).sum();
    contributors[0].contributions as f64 / all_contribiutions as f64
}

async fn get_bus_factor(
    client: Arc<Octocrab>,
    options: Arc<ContributorParameters>,
    owner: &str,
    repo: &str,
    results: mpsc::Sender<RepositoryBusFactor>,
) {
    let contributors = match client.list_repo_contributors(owner, repo, &options).await {
        Ok(mut result) => result.take_items(),
        Err(err) => {
            log::error!(
                "cannot fetch contributor list for: {}/{} | reason: {}",
                owner,
                repo,
                err
            );
            Vec::new()
        }
    };

    let result = RepositoryBusFactor {
        project: repo.to_string(),
        user: owner.to_string(),
        bus_factor: calculate_bus_factor(contributors),
    };

    if let Err(err) = results.send(result).await {
        log::error!("cannot send results {:?}", err);
    }
}

pub async fn bus_factor_command(
    client: Arc<Octocrab>,
    language: &str,
    project_count: u32,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel(project_count as usize);
    let contributors_options = Arc::new(ContributorParameters::new().per_page(25).page(1));

    let mut page = match client
        .search()
        .repositories(&format!("language:{}", language))
        .sort("stars")
        .order("desc")
        .per_page(50)
        .send()
        .await
    {
        Ok(projects) => projects,
        Err(err) => {
            return Err(anyhow::anyhow!(
                "cannot fetch projects from github API | reason: {}",
                err
            ));
        }
    };

    let mut repositories = Vec::with_capacity(project_count as usize);
    let mut number = 0;

    'outer: loop {
        let current_items = page.take_items();
        for repository in current_items {
            repositories.push(repository);
            number += 1;
            if number == project_count {
                break 'outer;
            }
        }

        page = match client.get_page(&page.next).await? {
            Some(next_page) => next_page,
            None => break 'outer,
        }
    }

    for repo in repositories {
        let c = client.clone();
        let co = contributors_options.clone();
        let t = tx.clone();
        spawn(async move {
            get_bus_factor(c, co, &repo.owner.login, &repo.name, t).await;
        });
    }

    drop(tx);

    while let Some(result) = rx.recv().await {
        if result.bus_factor >= BUS_FACTOR_THRESHOLD {
            println!(
                "project: {:<30}user: {:<30}percentage: {:<30.2}",
                result.project, result.user, result.bus_factor
            );
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bus_factor() {
        let empty = Vec::new();
        assert_eq!(calculate_bus_factor(empty), 0.0);

        let mut results = Vec::new();
        results.push(Contributor {
            login: String::from("example1"),
            contributions: 75,
        });
        results.push(Contributor {
            login: String::from("example2"),
            contributions: 25,
        });
        assert_eq!(calculate_bus_factor(results), 0.75);
    }
}
