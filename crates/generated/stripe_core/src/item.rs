// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::{
    ids::CheckoutSessionItemId,
    params::{Expandable, Object},
};
use serde::{Deserialize, Serialize};
use stripe::resources::{Currency, Discount, Price, Product, TaxRate};

/// The resource representing a Stripe "LineItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSessionItem {
    /// Unique identifier for the object.
    pub id: CheckoutSessionItemId,

    /// Total discount amount applied.
    ///
    /// If no discounts were applied, defaults to 0.
    pub amount_discount: i64,

    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total after discounts and taxes.
    pub amount_total: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,

    /// The discounts applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,

    /// The price used to generate the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// The ID of the product for this line item.
    ///
    /// This will always be the same as `price.product`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Expandable<Product>>,

    /// The quantity of products being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

impl Object for CheckoutSessionItem {
    type Id = CheckoutSessionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "item"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,

    pub discount: Discount,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}
