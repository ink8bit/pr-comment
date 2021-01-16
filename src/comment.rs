//! Comment struct and methods to construct a valid comment

use super::config::Config;

use std::error::Error;
use std::process::Command;
use std::str;

/// Comment struct that defines a comment
///
/// - `reviewers` - reviewer or list of reviewers
/// - `links` - repo links
/// - `config` - configuration options
#[derive(Debug)]
pub struct Comment {
    pub reviewers: String,
    pub links: String,
    pub config: Config,
}

impl Comment {
    /// Returns a new comment with provided list of reviewers, links and config values.
    /// Otherwise - an error with some description
    ///
    /// # Arguments
    ///
    /// * `reviewers` - reviewer or list of reviewers
    ///
    /// * `links` - repo links in a short form.
    /// For example, `b/1`, where `b` is a key from configuration file and `1` is a pull request ID value.
    ///
    /// * `config` - configuration settings
    pub fn new(reviewers: &str, links: &str, config: Config) -> Result<Self, String> {
        let revs = Comment::check_reviewers(reviewers, &config)?;

        let comment = Self {
            reviewers: String::from(revs),
            links: String::from(links),
            config,
        };

        Ok(comment)
    }

    /// Prints an output comment to stdout
    pub fn print(self) -> String {
        let branch = Comment::branch_name().unwrap_or_else(|_| String::from("no branch name"));
        let reviewers = Comment::format_reviewers(&self);
        let links = Comment::format_links(&self);

        let comment = format!(
            "
{branch}
{links}
{review}
**CHANGES**
TODO: what you've changed

**TESTING**
TODO: how to test changes you've made
    ",
            branch = branch,
            links = links,
            review = reviewers,
        );

        comment.trim().to_string()
    }

    /// Returns a current git branch name
    fn branch_name() -> Result<String, Box<dyn Error>> {
        let out = Command::new("git")
            .args(&["branch", "--show-current"])
            .output()?;
        let name = str::from_utf8(&out.stdout)?;
        let mut formatted = String::from("**PULL REQUEST**\n");
        formatted.push_str(name);

        Ok(formatted)
    }

    /// Returns an error if there are no reviewers provided in config file or using args
    fn check_reviewers<'a>(reviewers: &'a str, config: &Config) -> Result<&'a str, &'a str> {
        if reviewers.is_empty() && config.default_reviewer.is_empty() {
            return Err("You haven't provided any reviewer.");
        }
        Ok(reviewers)
    }

    /// Returns a formatted string with a list of reviewers
    ///
    /// An output should look like:
    ///
    /// @reviewer_1
    /// @reviewer_2
    /// ...
    /// @reviewer_N
    fn format_reviewers(&self) -> String {
        let mut rs = String::from("**REVIEW**\n");
        if self.reviewers.is_empty() {
            rs.push_str(&format!("@{}\n", self.config.default_reviewer));
            return rs;
        }

        let revs: Vec<&str> = self.reviewers.split(',').collect();
        if !revs.is_empty() && !revs[0].is_empty() {
            for rev in revs {
                rs.push_str(&format!("@{}\n", rev));
            }
        }

        rs
    }

    /// Returns a formatted string of repo links values
    ///
    /// An output should look like (GitHub as an example here, you can provide any link via config file):
    ///
    /// - <repo_name> https://github.com/<user>/<repo_name>/pull/<pull_request_id>
    fn format_links(&self) -> String {
        let link_list: Vec<&str> = self.links.split(',').collect();
        let mut s = String::from("**LINKS**\n");

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
    fn should_create_comment_with_proper_field_values() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "".to_string(),
            links,
        };
        let c = Comment::new("example_reviewer", "b/1", config).unwrap();
        let reviewers = Comment::format_reviewers(&c);
        let links = Comment::format_links(&c);

        assert_eq!(links, "**LINKS**\n".to_string());
        assert_eq!(reviewers.trim(), "**REVIEW**\n@example_reviewer");
    }

    #[test]
    fn should_use_default_reviewer() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment::new("", "b/1", config).unwrap();
        let reviewer = Comment::format_reviewers(&c);

        assert_eq!(reviewer.trim(), "**REVIEW**\n@default_reviewer");
    }

    #[test]
    fn should_use_reviewers() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment::new("example_reviewer_one,example_reviewer_two", "b/1", config).unwrap();

        let reviewers = Comment::format_reviewers(&c);

        assert_eq!(
            reviewers,
            "**REVIEW**\n@example_reviewer_one\n@example_reviewer_two\n"
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
        let reviewers = Comment::check_reviewers("", &config).unwrap();

        assert_eq!(reviewers, "");
    }

    #[test]
    #[ignore]
    fn should_print_comment() {
        let links = HashMap::new();
        let config = Config {
            default_reviewer: "default_reviewer".to_string(),
            links,
        };
        let c = Comment::new("", "b/1", config).unwrap();

        assert_eq!(c.print(), "**PULL REQUEST**\nfeature/123\n\n**LINKS**\n\n**REVIEW**\n@default_reviewer\n\n**CHANGES**\n_TODO:_ what you\'ve changed\n\n**TESTING**\n_TODO:_ how to test changes you\'ve made");
    }
}
