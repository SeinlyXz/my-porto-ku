use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section { id: "about", class: "py-24 px-6",
            div { class: "max-w-6xl mx-auto",
                div { class: "font-mono text-amber-500 text-sm mb-8 tracking-widest",
                    "// 002 — ABOUT"
                }
                div { class: "border-l-2 border-amber-500 pl-8",
                    h2 { class: "font-heading text-3xl font-bold text-white mb-6 tracking-wide uppercase",
                        "Professional Summary"
                    }
                    p { class: "font-body text-gray-300 text-lg leading-relaxed",
                        "Full Stack Developer with over 3 years of experience building reliable, scalable, and production-ready web platforms. Proven track record of delivering end-to-end solutions for paid voting systems, ticketing platforms, and organizational apps. Proficient in Go, SvelteKit, Node.js, PostgreSQL, and Google Cloud Platform. Committed to clean architecture, maintainability, and shipping high-impact features."
                    }
                }
            }
        }
    }
}
