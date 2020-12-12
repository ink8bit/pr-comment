use super::config::Config;
pub struct Comment {
    pub id: String,
    pub reviewers: String,
    pub links: String,
    pub is_bug: bool,
    pub config: Config,
}

impl Comment {
    pub fn new(self) -> Result<String, String> {
        let branch_name = Comment::branch_name(&self);
        let reviewers = Comment::reviewers(&self)?;
        let links = Comment::links(&self);

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
            branch = branch_name,
            links = links,
            review = reviewers,
        );

        Ok(comment)
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
