use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "h".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("h").description("Responds with h.")
}
