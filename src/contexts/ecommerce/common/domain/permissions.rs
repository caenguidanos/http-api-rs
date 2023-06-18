use derive_more::Display;

#[derive(Debug, Display)]
pub enum Permissions {
    #[display(fmt = "ecommerce.backoffice.product:read")]
    EcommerceBackofficeProductRead,

    #[display(fmt = "ecommerce.backoffice.product:create")]
    EcommerceBackofficeProductCreate,
}
