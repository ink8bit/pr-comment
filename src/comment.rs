use std::{collections::HashMap, error::Error};

use super::config;
pub struct Comment {
    pub branch_name: String,
    pub reviewers: String,
    pub links: String,
}

pub fn create(c: Comment) -> String {
    format!(
        "
**PULL REQUEST**
{branch}

**LINKS**
{links}

**REVIEW**
{review}

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
",
        branch = c.branch_name,
        links = c.links,
        review = c.reviewers,
    )
}

pub fn branch(id: &str, is_bug: bool) -> String {
    if is_bug {
        return format!("bugfix/{}", id);
    }
    format!("feature/{}", id)
}

pub fn reviewers(reviewer: &str, default_reviewer: String) -> Result<String, Box<dyn Error>> {
    if reviewer.is_empty() && default_reviewer.is_empty() {
        panic!("You haven't provided any reviewer.");
    }

    let mut rs = String::new();
    if reviewer.is_empty() {
        rs.push_str(&format!("@{}\n", default_reviewer));
    }

    let revs: Vec<&str> = reviewer.split(",").collect();
    if revs.len() > 0 && !revs[0].is_empty() {
        for rev in revs {
            rs.push_str(&format!("@{}\n", rev));
        }
    }

    Ok(rs)
}

pub fn links(links: &str, config_links: HashMap<String, config::LinkInfo>) -> String {
    let link_list: Vec<&str> = links.split(",").collect();
    let mut s = String::new();

    for link in link_list {
        let parts: Vec<&str> = link.split("/").collect();
        let repo_abbrev = parts[0];
        let pr_id = parts.get(1).unwrap_or(&"");
        if config_links.contains_key(repo_abbrev) {
            let val = config_links.get(repo_abbrev).unwrap();
            s.push_str(&format!("- {} {}/{}\n", val.repo_name, val.url, pr_id));
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_when_no_reviewers() {
        reviewers("", String::from("")).unwrap();
    }

    #[test]
    fn use_default_reviewer_when_no_one_provided() {
        assert_eq!(reviewers("", String::from("batman")).unwrap(), "@batman\n");
    }

    #[test]
    fn feature_branch_if_bug_flag_is_true() {
        assert_eq!(branch("123", false), "feature/123");
    }

    #[test]
    fn bugfix_branch_if_bug_flag_is_true() {
        assert_eq!(branch("123", true), "bugfix/123");
    }
}
