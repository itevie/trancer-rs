/// Usage: let val = confirm_action!(ctx, CreateMessage::new().content("ARE U SURE?"));
/// Returns: (bool, Message)
#[macro_export]
macro_rules! confirm_action {
    ($ctx:ident, $msg:expr) => {{
        let row = CreateActionRow::Buttons(vec![
            CreateButton::new("Yes")
                .style(ButtonStyle::Success)
                .label("Yes"),
            CreateButton::new("No")
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
