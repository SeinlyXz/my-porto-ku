use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "min-h-screen flex items-center px-6 relative overflow-hidden",
            // Decorative ghost number
            div { class: "absolute -right-10 top-1/2 -translate-y-1/2 font-heading text-[18rem] md:text-[28rem] font-bold text-white/[0.03] select-none pointer-events-none leading-none tracking-tighter",
                "01"
            }

            div { class: "max-w-6xl mx-auto w-full relative z-10 flex flex-col md:flex-row items-center gap-12",
                // Left: profile image with animation
                div { class: "shrink-0 animate-float relative",
                    // Ambient glow
                    div { class: "absolute inset-0 bg-amber-500/15 blur-2xl animate-pulse-glow rounded-2xl" }
                    // Offset wireframe behind image
                    div { class: "absolute inset-0 border border-amber-500/30 rounded-2xl translate-x-3 translate-y-3 hero-image-clip" }
                    // Corner bracket accents
                    div { class: "absolute -top-2.5 -left-2.5 w-5 h-5 border-t-2 border-l-2 border-amber-500 z-20" }
                    div { class: "absolute -bottom-2.5 -right-2.5 w-5 h-5 border-b-2 border-r-2 border-amber-500 z-20" }
                    // Main image
                    img {
                        src: asset!("/assets/profile.png"),
                        alt: "Profile photo",
                        class: "w-48 h-48 md:w-64 md:h-64 rounded-2xl object-cover relative z-10 hero-image-clip",
                    }
                }

                // Right: text content
                div { class: "flex-1",
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
}
