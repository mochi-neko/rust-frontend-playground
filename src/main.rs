mod firebase;
mod generated;
mod logging;
mod sign_up;

use dioxus::prelude::{
    dioxus_elements, rsx, use_future, use_state, Element, Scope,
};

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    let mail_address = use_state(cx, String::new);
    let password = use_state(cx, String::new);
    let log_in = use_future(cx, (), |_| {
        let mail_address = mail_address.get().clone();
        let password = password.get().clone();

        async move {
            let info = sign_up::SignUpInfo {
                mail_address,
                password,
            };

            log::info!("Sign up: {:?}", info);
            sign_up::sign_up(&info)
                .await
                .unwrap_or_default();
        }
    });

    cx.render(rsx! {
        h1 { "SignUp" }

        label { "e-mail: " }

        input {
            r#type: "email",
            oninput: move |event| {
                log::info!("Input e-mail address: {}", event.value);
                mail_address.with_mut(|address| *address = event.value.clone())
            },
        }

        br {}

        label { "password: " }

        input {
            r#type: "password",
            oninput: move |event| {
                log::info!("Input password: {}", event.value.clone().replace(|_| true, "*"));
                password.with_mut(|password| *password = event.value.clone())
            },
        }

        br {}

        button {
            onclick: move |_| log_in.restart(),
            "Register",
        }
    })
}
