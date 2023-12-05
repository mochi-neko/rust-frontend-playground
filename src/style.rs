pub(crate) const STYLE_CSS: &str = r#"
    html {
        color-scheme: light dark;
    }

    body {
        background-color: var(--mdc-theme-background);
        display: "flex";
        flex-wrap: "wrap";
        gap: 1rem;
        border: 1px solid currentColor;
        padding: 1rem;
        margin: 1rem;
    }
"#;
