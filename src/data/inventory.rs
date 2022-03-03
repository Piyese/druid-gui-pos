use druid::{Data, Lens, EventCtx, Env, widget::Controller, Widget, Selector, AppDelegate};

use crate::AppState;

// pub const SAVEPRODUCT: Selector<Product> = Selector::new("add.product");
// pub struct ProductController;
// impl<W:Widget<ProductEdit>> Controller<ProductEdit, W> for ProductController {
//     fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &ProductEdit, data: &ProductEdit, env: &Env) {

//         ctx.submit_command(SAVEPRODUCT.with(Product::from(data.to_owned())));
//         dbg!(&old_data);
//         dbg!(&data);
//         child.update(ctx, old_data, data, env)
//     }
// }
// pub struct Delegate;
// impl AppDelegate<AppState> for Delegate {
//     fn command(
//         &mut self,
//         _ctx: &mut druid::DelegateCtx,
//         _target: druid::Target,
//         cmd: &druid::Command,
//         data: &mut AppState,
//         _env: &Env,
//     ) -> druid::Handled {
//         if let Some(prod) = cmd.get(SAVEPRODUCT) {
//             data.add_product(prod);
//             druid::Handled::Yes
//         }else { dbg!(&data); druid::Handled::No }
//     }
// }

#[derive(Debug, Clone, Data, Lens, Default, PartialEq)]
pub struct Product {
    pub name: String,
}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct ProductEdit {
    pub name: String
}

impl From<ProductEdit> for Product {
    fn from(item: ProductEdit) -> Self {
        Self { name: item.name }
    }
}
