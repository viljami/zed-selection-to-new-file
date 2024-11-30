use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct SelectionToNewFile {
    // ... state
}

impl zed::Extension for SelectionToNewFile {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if let Some(worktree) = worktree {
            println!("Worktree: {worktree:?}");
        }

        match command.name.as_str() {
            "move" => {
                if !args.is_empty() {
                    return Err("Move accepts no arguments".to_string());
                }

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..10_usize).into(),
                        label: "Move".to_string(),
                    }],
                    text: "Selection to new file".to_string(),
                })
            }

            command => Err(format!("Unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(SelectionToNewFile);
