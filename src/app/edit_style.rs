use super::{Message, utils::with_background};
use crate::notes::NoteStyle;
use cosmic::prelude::*;
use cosmic::widget;

pub fn edit_style_dialog(style: &NoteStyle) -> Element<'_, Message> {
    widget::dialog()
        .title("Edit style")
        .body("Edit style body")
        .control(with_background(
            build_edit_style_control(style),
            style.bgcolor,
        ))
        .primary_action(widget::button::text("Ok").on_press(Message::EditStyleUpdate))
        .secondary_action(widget::button::text("Cancel").on_press(Message::EditStyleCancel))
        .into()
}

fn build_edit_style_control(style: &NoteStyle) -> Element<'_, Message> {
    widget::column::with_capacity(3)
        .push(
            widget::row::with_capacity(2)
                .push(widget::text("Name:"))
                .push(widget::text(&style.name)),
        )
        .push(
            widget::row::with_capacity(2)
                .push(widget::text("Font:"))
                .push(widget::text(&style.font_name)),
        )
        .push(
            widget::row::with_capacity(2)
                .push(widget::text("Background:"))
                .push(widget::text(format!("{:?}", &style.bgcolor))),
        )
        .into()
}
