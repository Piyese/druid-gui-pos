use druid::{Widget, widget::{Label, Flex, TextBox, Button, Align, Padding, Split}, WidgetExt, Env};

use crate::{data::inventory::{RawMaterial}, AppState};

pub fn rm_widget()->impl Widget<RawMaterial> {
    let disp = Flex::row()
        .with_flex_child(
            Split::columns(
                Label::raw().lens(RawMaterial::name).align_left().padding(druid::Insets::uniform(5.)),
                Label::new(|data: &RawMaterial, _env: &Env|{
                    format!("{} Kg",data.quantity)
                }).align_right().padding(druid::Insets::uniform(5.))//.lens(RawMaterial::quantity)
            ).solid_bar(true).split_point(0.8),
            1.
        );

    disp
}
