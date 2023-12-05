mod firebase;
mod generated;
mod logging;
mod sign_up;
mod style;

use dioxus::{
    hooks::to_owned,
    html::GlobalAttributes,
    prelude::{
        dioxus_elements, fc_to_builder, rsx, use_future, use_state, Element,
        Scope,
    },
};
use material_dioxus::{theming::Colors, MatButton, MatTextField, MatTheme};

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
        style {
            // NOTE: Failed to load style.css then use inline style
            dangerous_inner_html: crate::style::STYLE_CSS,
        }

        MatTheme {
            theme: Colors {
                ..Colors::DEFAULT_LIGHT
            },
            dark_theme: Some(Colors {
                ..Colors::DEFAULT_DARK
            }),
        }

        h1 { "SignUp" }

        div {
            MatTextField {
                label: "e-mail",
                value: mail_address.get(),
                _oninput: {
                    to_owned![mail_address];
                    move |event :String| {
                        log::info!("Input e-mail address: {}", event);
                        mail_address.set(event)
                    }
                }
            }
        }

        div {
            MatTextField {
                label: "password",
                value: password_field(password.get().clone()),
                _oninput: {
                    to_owned![password];
                    move |event: String| {
                        // NOTE: Hide password
                        // log::info!("Input password: {}", event);
                        password.set(event)
                    }
                }
            }
        }

        div {
            span {
                onclick: move |_| {
                    log::info!("Sign up");
                    log_in.restart();
                },
                MatButton{
                    label: "Sign Up",
                    outlined: true,
                }
            }
        }
    })
}

fn password_field(password: String) -> String {
    let count = password.chars().count();

    password
        .chars()
        .enumerate()
        .map(|(index, character)| {
            if index != count - 1 {
                '*'
            } else {
                character
            }
        })
        .collect::<String>()
}
