use dioxus::prelude::*;

#[component]
pub fn Education() -> Element {
    rsx! {
        section { id: "education", class: "py-24 px-6 bg-[#13151a]",
            div { class: "max-w-6xl mx-auto",
                div { class: "font-mono text-amber-500 text-sm mb-8 tracking-widest",
                    "// 005 — CREDENTIALS"
                }
                h2 { class: "font-heading text-3xl font-bold text-white mb-12 tracking-wide uppercase",
                    "Education & Certifications"
                }

                // Education
                div { class: "border-l-2 border-amber-500 pl-8 mb-12",
                    h3 { class: "font-heading text-xl font-bold text-white mb-1",
                        "Universitas AMIKOM Yogyakarta"
                    }
                    p { class: "font-body text-gray-400 mb-3",
                        "Bachelor's Degree in Informatika (2022 — 2026)"
                    }
                    div { class: "flex items-baseline gap-2",
                        span { class: "font-mono text-amber-500 text-sm", "GPA" }
                        span { class: "font-heading text-3xl font-bold text-white", "3.97" }
                        span { class: "font-body text-gray-500 text-sm", "/ 4.0" }
                    }
                }

                // Certifications
                div {
                    h3 { class: "font-mono text-xs text-amber-500/70 tracking-widest mb-4",
                        "CERTIFICATIONS"
                    }
                    div { class: "flex flex-wrap gap-4",
                        div { class: "border border-gray-700 p-4 hover:border-amber-500/50 transition-colors",
                            p { class: "font-heading font-bold text-white text-sm",
                                "Junior Network Administrator"
                            }
                            p { class: "font-mono text-gray-500 text-xs mt-1", "BNSP — Oct 2023" }
                        }
                        div { class: "border border-gray-700 p-4 hover:border-amber-500/50 transition-colors",
                            p { class: "font-heading font-bold text-white text-sm",
                                "Certified Basic Programmer"
                            }
                            p { class: "font-mono text-gray-500 text-xs mt-1",
                                "HackerRank — Jul 2023"
                            }
                        }
                    }
                }

                div { class: "",
                    p { "Mini Game" }
                    button {
                        onclick: move |_| {
                            println!("Button clicked!");
                        },
                        "Click Me"
                    }
                }
            }
        }
    }
}
