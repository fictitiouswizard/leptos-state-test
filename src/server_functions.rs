use leptos::*;
use crate::app_state::AppState;

#[server(FooBar, "/api")]
pub async fn foo_bar(thing: String) -> Result<String, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::web;

    let data: web::Data<AppState>  = extract().await?;
    println!("{}", &data.foo);
    Ok(data.foo.clone())
}