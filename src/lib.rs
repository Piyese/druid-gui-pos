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

use druid::{
    widget::{
        Flex, // MainAxisAlignment, CrossAxisAlignment, 
        Padding, Label, Container, Split, Align
    }, 
    Widget, WidgetExt, Color
};

pub fn inventory_ui()->impl Widget<u32> {
    let product_reg: Container<u32> = Container::new(
        Flex::column()
            .with_flex_child(
                Split::rows(
                    Align::centered(Label::new("Prod Reg")),
                    Align::centered(Label::new("Prod list"))
                )
                .split_point(0.4), 1.0
            )
    );

    let finshed_prod: Flex<u32> = Flex::column().with_flex_child(
        Align::centered(Label::new("Finished Prod")), 1.
    );

    let side_bar: Padding<u32> = Padding::new(
        5.0, 
        Container::new( 
            Flex::column()
                    .with_flex_child(
                        Split::rows(product_reg, finshed_prod)
                        .split_point(0.55), 1.0
                    )
                    // .main_axis_alignment(MainAxisAlignment::End)
                    // .cross_axis_alignment(CrossAxisAlignment::End)
                    .expand_height()
                    .fix_width(300.)
                ).border(Color::WHITE, 1.0)
    );

    let top_row: Padding<u32> = Padding::new(
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

    let bottom_row: Padding<u32> = Padding::new(
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