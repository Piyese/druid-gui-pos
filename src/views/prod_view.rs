use druid::{Widget, widget::{Label, Flex, TextBox, Button}, WidgetExt};

use crate::{data::inventory::{Product, ProductEdit}, AppState};


// create a widget to represent a single product
pub fn prod_widget()-> impl Widget<Product> {
    let label = Label::raw().lens(Product::name);
    Flex::row().with_flex_child(label, 1.0)
}

pub fn new_prod_textbox()-> impl Widget<AppState> {
    let textbox = TextBox::new()
        .with_placeholder("add a new product")
        .expand_width()
        .lens(ProductEdit::name);
    let add_btn = Button::new("ADD").on_click(AppState::click_add_new);

    Flex::row()
        .with_flex_child(textbox.lens(AppState::product_editor), 1.)
        .with_child(add_btn)
        // .controller(ProductController)
}