/// Usage: let val = confirm_action!(ctx, CreateMessage::new().content("ARE U SURE?"));
/// Returns: (bool, Message)
/// Needs:
/// use crate::commands::error;
/// use crate::{reply};
/// use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, EditMessage};
#[macro_export]
macro_rules! confirm_action {
    ($ctx:ident, $msg:expr) => {{
        let row = CreateActionRow::Buttons(vec![
            CreateButton::new("yes")
                .style(ButtonStyle::Success)
                .label("Yes"),
            CreateButton::new("no")
                .style(ButtonStyle::Danger)
                .label("No"),
        ]);
        let mut msg = reply!($ctx, $msg.components(vec![row.clone()]))?;
        let result = msg
            .await_component_interaction(&$ctx.sy)
            .timeout(Duration::from_secs(60))
            .await;
        msg.edit(&$ctx.sy, EditMessage::new().components(vec![]))
            .await?;

        (
            match result {
                Some(some) => some.data.custom_id == "yes",
                None => false,
            },
            msg.clone(),
        )
    }};
}
