use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Thing {
    pub that: String,
}

#[server(FooBar, "/api")]
pub async fn foo_bar(_thing: String) -> Result<Thing, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web;
    use crate::app_state::AppState;

    let data: web::Data<AppState>  = extract().await?;
    println!("{}", &data.foo);
    Ok(Thing {
        that: String::from("greg")
    })
}