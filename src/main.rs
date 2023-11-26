mod logging;
mod signup;

use dioxus::prelude::{Element, Scope, rsx, dioxus_elements, use_ref, use_state};

fn main() -> anyhow::Result<()> {
    logging::initialize()?;

    dioxus_web::launch(app);

    Ok(())
}

fn app(cx: Scope) -> Element {
    let mail_address = use_state(&cx, || String::new());
    let password = use_state(&cx, || String::new());
    let signup = use_state(&cx, || signup::SignupInfo {
        mail_address: String::new(),
        password: String::new(),
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
            onclick: move |_| {
                log::info!("Register mail_address: {}, password: {}", mail_address.get(), password.get().replace(|_| true, "*"));
                signup.set(signup::SignupInfo {
                    mail_address: mail_address.get().clone(),
                    password: password.get().clone(),
                });

                signup::register(signup.get()).unwrap()
            },
            "Register",
        }
    })
}

