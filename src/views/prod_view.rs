use druid::{Widget, widget::{Label, Flex, TextBox, Button, Align, Padding, Switch, Either, ValueTextBox, Container}, WidgetExt, Env, Color, EventCtx};

use crate::{data::inventory::{Product, ProductEdit, FinishedProd}, AppState};


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

    Flex::column()
        .with_child(Padding::new(5.,Label::new("PRODUCT REGISTRATION")))
        .with_flex_child(Padding::new(5.,textbox.lens(AppState::product_editor)), 1.)
        .with_child(Padding::new(5.,Align::right(add_btn)))
        // .controller(ProductController)
}

pub fn fp_widget()-> impl Widget<FinishedProd> {
    let name_label = Flex::row()
        .with_flex_child(Label::raw().lens(Product::name).lens(FinishedProd::product), 1.);
    let qty_label = Either::new(|data: &FinishedProd, _env: &Env|data.which,
        add_fp(),
        Label::new(|data: &FinishedProd, _env: &Env|{ format!("{} Kg",data.quantity) })
    );

    let disp = Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    name_label.padding(5.).align_left(), 
                    1.
                )
                .with_flex_child(
                    Align::right(Switch::new().lens(FinishedProd::which)), 1.
                )
    ).with_child(qty_label.align_right().padding(10.));

    Padding::new(5.,Container::new(disp).border(Color::WHITE, 0.5))
        
}

pub fn add_fp()-> impl Widget<FinishedProd> {
    let textbox = TextBox::new().with_placeholder("edit quantity").lens(FinishedProd::edit_qty);
    let btn = Button::new("ADD").on_click(FinishedProd::click_add_new);
    let disp = Flex::column()
      .with_child(Label::new("add quantity"))
      .with_child(Flex::row()
        .with_flex_child(textbox, 1.).with_child(btn));
    Container::new(disp)
}