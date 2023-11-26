mod logging;

use dioxus::prelude::{Element, Scope, use_state, rsx, dioxus_elements};

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    let mail_address = use_state(&cx, || String::new());
    let password = use_state(&cx, || String::new());

    cx.render(rsx! {
        h1 { "SignUp" }

        input {
            r#type: "text",
            oninput: move |event| {
                log::info!("Input mail_address: {}", event.value);
                mail_address.with_mut(|address| *address = event.value.clone())
            },
        }

        input {
            r#type: "password",
            oninput: move |event| {
                log::info!("Input password: {}", event.value.clone().replace(|_| true, "*"));
                password.with_mut(|password| *password = event.value.clone())
            },
        }

        button {
            onclick: move |_| {
                log::info!("Register mail_address: {}, password: {}", mail_address.get(), password.get().replace(|_| true, "*"));
                register(mail_address.get(), password.get()).unwrap()
            },
            "Register",
        }
    })
}

fn register(
    _mail_address: &String,
    _password: &String) -> anyhow::Result<()> {
    Ok(())
}