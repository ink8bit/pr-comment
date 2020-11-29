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
