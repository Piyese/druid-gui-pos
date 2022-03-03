// let sidebar = Flex::column()
// .main_axis_alignment(MainAxisAlignment::Start)
// .cross_axis_alignment(CrossAxisAlignment::Start)
// .with_child(axis_picker)
// .with_default_spacer()
// .with_child(cross_picker)
// .with_default_spacer()
// .with_child(transit_picker)
// .with_default_spacer()
// .with_child(extra_tab)
// .with_flex_spacer(1.)
// .fix_width(200.0)
// .lens(AppState::tab_config);

pub mod data;
pub mod views;

use data::inventory::{Product, ProductEdit};
use druid::{
    widget::{
        Flex, // MainAxisAlignment, CrossAxisAlignment, 
        Padding, Label, Container, Split, Align, List, Scroll
    }, 
    Widget, WidgetExt, Color, im::Vector, Data, Lens, EventCtx, Env
};
use views::prod_view::{prod_widget, new_prod_textbox};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    product_editor: ProductEdit,
    products: Vector<Product>
}
impl AppState {
    pub fn dummy_data()->AppState {
        let products = (0..10).into_iter().map(|f|Product{ name: format!("Product{}",f) });
        let products = Vector::from_iter(products);
        Self { products, product_editor: ProductEdit::default() }
    }
    pub fn add_product(&mut self) {
        if !self.product_editor.name.is_empty(){
            self.products.push_front(Product::from(self.product_editor.to_owned()));
            self.product_editor = ProductEdit::default();
        }
    }
    pub fn click_add_new(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_product();
    }
}

pub fn inventory_ui()->impl Widget<AppState> {
    let product_section: Container<AppState> = Container::new(
        Flex::column()
            .with_flex_child(
                Split::rows(
                    Align::centered(new_prod_textbox()),
                    Flex::column()
                        .with_child(Label::new("PRODUCT(S)"))
                        .with_flex_child(
                            Padding::new(
                                10.0, 
                                Scroll::new(
                                    List::new(prod_widget).lens(AppState::products)
                                ).vertical()
                            ), 
                            1.0
                        )
                )
                .split_point(0.4), 1.0
            )
    );

    let finshed_prod: Flex<AppState> = Flex::column().with_flex_child(
        Align::centered(Label::new("Finished Prod")), 1.
    );

    let side_bar: Padding<AppState> = Padding::new(
        5.0, 
        Container::new( 
            Flex::column()
                    .with_flex_child(
                        Split::rows(product_section, finshed_prod)
                        .split_point(0.55), 1.0
                    )
                    // .main_axis_alignment(MainAxisAlignment::End)
                    // .cross_axis_alignment(CrossAxisAlignment::End)
                    .expand_height()
                    .fix_width(300.)
                ).border(Color::WHITE, 1.0)
    );

    let top_row: Padding<AppState> = Padding::new(
        3.,
        Container::new(
            Split::columns(
                Align::centered(Label::new("Packaged Items")),
                Split::columns(
                    Align::centered(Label::new("production log")), 
                    Align::centered(Label::new("more info")) 
                )
                .split_point(0.5)
            )
            .split_point(0.4)
        ).background(Color::SILVER)
        .border(Color::WHITE, 1.0),
    );

    let bottom_row: Padding<AppState> = Padding::new(
        3.0,
        Container::new(
            Split::columns(
                Align::centered(Label::new("Daily Sales")),
                Align::centered(Label::new("Raw Materials")),
            ).split_point(0.5)
        )
        .border(Color::WHITE, 1.0),
    );

    let mut org_col = Flex::row();
    org_col.add_child(side_bar);
    org_col.add_flex_child(
        Container::new(
            Split::rows(top_row, bottom_row)
                .split_point(0.5)
                .bar_size(0.0)
            ),
            1.0
    );
    org_col //.debug_paint_layout()
}