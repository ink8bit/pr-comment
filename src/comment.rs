use std::{collections::HashMap, error::Error};

use super::config;
pub struct Comment {
    pub id: String,
    pub reviewers: String,
    pub links: String,
}

pub fn create(c: Comment) -> String {
    format!(
        "
**PR**
`feature/{}`

**LINKS**
{}

**REVIEW**
{}

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
",
        c.id, c.links, c.reviewers,
    )
}

pub fn reviewers(r_flag_value: &str, default_reviewer: String) -> Result<String, Box<dyn Error>> {
    if r_flag_value.is_empty() && default_reviewer.is_empty() {
        panic!("you haven't provided any reviewer.");
    }

    let mut rs = String::new();
    if r_flag_value.is_empty() {
        rs.push_str(&format!("@{}\n", default_reviewer));
    }

    let revs: Vec<&str> = r_flag_value.split(",").collect();
    if revs.len() > 0 && !revs[0].is_empty() {
        for rev in revs {
            rs.push_str(&format!("@{}\n", rev));
        }
    }

    Ok(rs)
}

pub fn links(l_flag_value: &str, config_links: HashMap<String, config::LinkInfo>) -> String {
    let links: Vec<&str> = l_flag_value.split(",").collect();
    let mut s = String::new();

    for link in links {
        let link_parts: Vec<&str> = link.split("/").collect();
        let repo_abbrev = link_parts[0];
        let pr_id = link_parts[1];
        if config_links.contains_key(repo_abbrev) {
            let val = config_links.get(repo_abbrev).unwrap();
            s.push_str(&format!("- [{}]({}/{})\n", val.description, val.url, pr_id));
        }
    }

    s
}
