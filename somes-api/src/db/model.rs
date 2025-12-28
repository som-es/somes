use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, PartialEq, Debug, Serialize, Deserialize)]
pub struct CallToOrdersPerPartyDelegates {
    party: String,
    call_to_order_amount: i32,
    delegate_amount: i32,
    ratio: f32,
}

#[derive(ToSchema, PartialEq, Debug, Serialize, Deserialize)]
pub struct DelegateByCallToOrders {
    name: String,
    image_url: Option<String>,
    party: Option<String>,
    call_to_order_amount: i32,
}

#[derive(ToSchema, PartialEq, Debug, Serialize, Deserialize)]
pub struct SpeakerByHours {
    name: String,
    image_url: Option<String>,
    party: Option<String>,
    hours_spoken: f32,
}

#[derive(ToSchema, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_email_hashed: bool,
    pub is_admin: bool,
}
