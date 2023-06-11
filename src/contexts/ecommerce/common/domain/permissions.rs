use derive_more::Display;

#[derive(Debug, Display)]
pub enum Permissions {
    #[display(fmt = "ecommerce.product:read")]
    EcommerceReadProduct,

    #[display(fmt = "ecommerce.product:create")]
    EcommerceCreateProduct,
}
