pub mod product;
pub mod notification;

use rocket::fairing::AdHoc;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/product", routes![product::create, product::list,
                product::read,
                product::delete,
                product::publish
            ])
            .mount("/notification", routes![notification::subscribe, notification::unsubscribe])

    })
}


