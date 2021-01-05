use std::error::Error;

use clipboard::{ClipboardContext, ClipboardProvider};

/// Copies comment to system clipboard
pub fn copy<'a>(need_copy: bool, comment: String) -> Result<&'a str, Box<dyn Error>> {
    if !need_copy {
        return Ok("");
    }
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(comment).unwrap();
    Ok("\nYour comment was successfully copied to your clipboard.")
}

#[cfg(test)]
mod tests {
    use super::copy;
    use clipboard::{ClipboardContext, ClipboardProvider};

    #[test]
    fn should_copy_string_value_to_clipboard() {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let fake_text = "fake text";
        let msg = copy(true, fake_text.to_string()).unwrap();

        assert_eq!(
            msg,
            "\nYour comment was successfully copied to your clipboard."
        );
        assert_eq!(ctx.get_contents().unwrap(), fake_text);
    }

    #[test]
    fn should_copy_if_flag_provided() {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let fake_text = "fake text";
        let _msg = copy(false, fake_text.to_string()).unwrap();

        assert_ne!(ctx.get_contents().unwrap_or_default(), fake_text);
    }
}
