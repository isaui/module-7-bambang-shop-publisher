use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;
use  crate::service::notification::NotificationService;
use crate::service::product::ProductService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Result<Created<Json<Subscriber>>>{
    return match NotificationService::subscribe(product_type, subscriber.into_inner() ) {
        Ok(f) => Ok(Created::new("/").body(Json::from(f))),
        Err(e) => Err(e)
    }
}
