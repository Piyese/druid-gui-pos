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

#[derive(Debug, Clone, Data, Lens, Default, PartialEq)]
pub struct RawMaterial {
    pub name: String,
    pub quantity: f32,
    // materials in store will have this as None
    // when construting an instance for transaction purposes, then it has to be Some(thing)
    pub price_per_kg: Option<f32>,
}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct RawMaterialEditor {
    pub name: String,
    pub quantity: String,
    pub price_per_kg: Option<f32>,
}
#[derive(Debug, Clone, Data, Lens, Default)]
pub struct FinishedProd {
    pub product: Product,
    pub quantity: f32,
    pub edit_qty: String,
    pub which: bool
}
impl FinishedProd {
    pub fn new(product: Product)->Self {
        Self { 
            product,
            quantity: 0.0,
            edit_qty: String::default(),
            which: false, 
        }
    }
    pub fn click_add_new(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_qty();
    }
    fn add_qty(&mut self) {
        if let Ok(edit) = self.edit_qty.parse::<f32>() {
            self.quantity += edit;
            self.which = false;
        }else {
            self.edit_qty = String::from("invalid!!");
        }
    }
}

impl From<FinishedProdEditor> for FinishedProd {
    fn from(item: FinishedProdEditor) -> Self {
        Self { product: item.product, quantity: item.quantity, edit_qty: 0.0.to_string(), which: false }
    }
}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct FinishedProdEditor {
    pub product: Product,
    pub quantity: f32
}
