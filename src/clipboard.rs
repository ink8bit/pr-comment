use std::error::Error;

use clipboard::{ClipboardContext, ClipboardProvider};

/// Copies comment to system clipboard
pub fn copy<'a>(need_copy: bool, comment: String) -> Result<&'a str, Box<dyn Error>> {
    if need_copy {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(comment).unwrap();
    }

    Ok("\nYour comment was successfully copied to your clipboard.")
}
