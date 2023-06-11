use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "db error: {}", _0)]
    Persistence(String),

    #[display(fmt = "product already exists")]
    ProductAlreadyExists,

    #[display(fmt = "invalid product timestamp relation")]
    InvalidProductTimeStampRelation,

    #[display(fmt = "invalid product id")]
    InvalidProductId,
    #[display(fmt = "invalid product name")]
    InvalidProductName,
    #[display(fmt = "invalid product price")]
    InvalidProductPrice,
    #[display(fmt = "invalid product currency")]
    InvalidProductCurrency,

    #[display(fmt = "invalid permission")]
    InvalidPermission,
}
