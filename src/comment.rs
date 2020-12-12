use super::config::Config;

#[derive(Debug)]
pub struct Comment {
    pub id: String,
    pub reviewers: String,
    pub links: String,
    pub is_bug: bool,
    pub config: Config,
}

impl Comment {
    pub fn new(self) -> Result<Self, String> {
        let branch_name = Comment::branch_name(&self);
        let reviewers = Comment::reviewers(&self)?;
        let links = Comment::links(&self);

        let comment = Self {
            id: branch_name,
            reviewers,
            links,
            config: self.config,
            is_bug: self.is_bug,
        };

        Ok(comment)
    }

    pub fn print(self) -> String {
        let comment = format!(
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
            branch = self.id,
            links = self.links,
            review = self.reviewers,
        );

        comment.trim().to_string()
    }

    fn branch_name(&self) -> String {
        if self.is_bug {
            return format!("bugfix/{}", self.id);
        }
        format!("feature/{}", self.id)
    }

    fn reviewers(&self) -> Result<String, String> {
        if self.reviewers.is_empty() && self.config.default_reviewer.is_empty() {
            return Err(String::from("You haven't provided any reviewer."));
        }

        let mut rs = String::new();
        if self.reviewers.is_empty() {
            rs.push_str(&format!("@{}\n", self.config.default_reviewer));
        }

        let revs: Vec<&str> = self.reviewers.split(',').collect();
        if !revs.is_empty() && !revs[0].is_empty() {
            for rev in revs {
                rs.push_str(&format!("@{}\n", rev));
            }
        }

        Ok(rs)
    }

    fn links(&self) -> String {
        let link_list: Vec<&str> = self.links.split(',').collect();
        let mut s = String::new();

        for link in link_list {
            let parts: Vec<&str> = link.split('/').collect();
            let repo_abbrev = parts[0];
            let pr_id = parts.get(1).unwrap_or(&"");
            if self.config.links.contains_key(repo_abbrev) {
                let val = self.config.links.get(repo_abbrev).unwrap();
                s.push_str(&format!("- {} {}/{}\n", val.repo_name, val.url, pr_id));
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Comment, Config};

    #[test]
    fn should_create_comment() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "example_reviewer".to_string(),
            is_bug: false,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(c.id, "feature/123");
        assert_eq!(c.links, "".to_string());
        assert_eq!(c.reviewers.trim(), "@example_reviewer");
    }

    #[test]
    fn should_create_bugfix() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "".to_string(),
            is_bug: true,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(c.id, "bugfix/123");
    }

    #[test]
    fn should_use_default_reviewer() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "".to_string(),
            is_bug: false,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(c.reviewers.trim(), "@default_reviewer");
    }

    #[test]
    fn should_use_reviewers() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "example_reviewer_one,example_reviewer_two".to_string(),
            is_bug: false,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(
            c.reviewers,
            "@example_reviewer_one\n@example_reviewer_two\n"
        );
    }

    #[test]
    #[should_panic(expected = "You haven\\\'t provided any reviewer.\"")]
    fn should_panic_when_no_reviewers() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "".to_string(),
            is_bug: false,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(c.reviewers, "");
    }

    #[test]
    fn should_print_comment() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment {
            id: "123".to_string(),
            links: "b/1".to_string(),
            reviewers: "".to_string(),
            is_bug: false,
            config,
        };
        let c = Comment::new(c).unwrap();
        assert_eq!(c.print(), "**PULL REQUEST**\nfeature/123\n\n**LINKS**\n\n**REVIEW**\n@default_reviewer\n\n**CHANGES**\n_TODO:_ what you\'ve changed\n\n**TESTING**\n_TODO:_ how to test changes you\'ve made");
    }
}
