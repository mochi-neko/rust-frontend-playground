use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let list = use_state(&cx, || Vec::<String>::new());

    cx.render(rsx! {
        h1 { "ToDo List" }

        input {
            r#type: "text",
            oninput: move |event| list.with_mut(|list| list.push(event.value.clone())),
        }

        button {
            onclick: move |_| list.with_mut(|list| list.clear()),
            "Clear All"
        }

        ul {
            list.iter().map(|task| rsx!(
                li { "{task}" }
            ))
        }
    })
}