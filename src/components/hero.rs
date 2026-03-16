use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "min-h-screen flex items-center px-6 relative overflow-hidden",
            // Decorative ghost number
            div { class: "absolute -right-10 top-1/2 -translate-y-1/2 font-heading text-[18rem] md:text-[28rem] font-bold text-white/[0.03] select-none pointer-events-none leading-none tracking-tighter",
                "01"
            }

            div { class: "max-w-6xl mx-auto w-full relative z-10",
                // Section label with blinking cursor
                div { class: "font-mono text-amber-500 text-sm mb-10 tracking-widest",
                    "// 001 — INTRODUCTION"
                    span { class: "animate-blink ml-1", "▌" }
                }

                // Name
                h1 { class: "font-heading text-5xl md:text-7xl lg:text-8xl font-bold tracking-tight text-white leading-[0.95] mb-6",
                    "MUHAMMAD"
                    br {}
                    span { class: "text-gray-500", "SEINLY " }
                    span { class: "text-amber-500", "ASHVIYA" }
                }

                // Divider
                div { class: "w-20 h-0.5 bg-amber-500 my-8" }

                // Subtitle
                p { class: "font-body text-xl md:text-2xl text-gray-300 mb-2 tracking-wide",
                    "Full Stack Website Developer"
                }
                p { class: "font-body text-gray-500 mb-10", "Sleman Regency, Yogyakarta, Indonesia" }

                // Contact links
                div { class: "flex gap-3 flex-wrap",
                    a {
                        href: "mailto:seinly@protonmail.com",
                        class: "border border-gray-700 px-4 py-2.5 font-mono text-xs text-gray-300 hover:border-amber-500 hover:text-amber-500 transition-all tracking-wide",
                        "seinly@protonmail.com"
                    }
                    a {
                        href: "https://linkedin.com/in/seinlyashviya",
                        target: "_blank",
                        class: "border border-gray-700 px-4 py-2.5 font-mono text-xs text-gray-300 hover:border-amber-500 hover:text-amber-500 transition-all tracking-wide",
                        "LinkedIn"
                    }
                    a {
                        href: "tel:+6282242873216",
                        class: "border border-gray-700 px-4 py-2.5 font-mono text-xs text-gray-300 hover:border-amber-500 hover:text-amber-500 transition-all tracking-wide",
                        "+62 822-4287-3216"
                    }
                }
            }
        }
    }
}
