use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "py-12 px-6 border-t border-gray-800",
            div { class: "max-w-4xl mx-auto flex justify-between items-center flex-wrap gap-4",
                p { class: "font-mono text-gray-500 text-xs tracking-wide",
                    "// 2026 MUHAMMAD SEINLY ASHVIYA"
                }
                div { class: "flex gap-6",
                    a {
                        href: "mailto:seinly@protonmail.com",
                        class: "font-mono text-xs text-gray-500 hover:text-amber-500 transition-colors tracking-wide",
                        "EMAIL"
                    }
                    a {
                        href: "https://linkedin.com/in/seinlyashviya",
                        target: "_blank",
                        class: "font-mono text-xs text-gray-500 hover:text-amber-500 transition-colors tracking-wide",
                        "LINKEDIN"
                    }
                }
            }
        }
    }
}
