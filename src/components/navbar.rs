use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    const NAV_LINK: &str = "font-mono text-xs text-gray-400 hover:text-amber-500 transition-colors tracking-widest uppercase";

    rsx! {
        nav { class: "fixed top-0 left-0 right-0 z-50 bg-[#0f1116]/80 backdrop-blur-md border-b border-gray-800/50",
            div { class: "max-w-4xl mx-auto py-4 flex justify-between items-center",
                a {
                    href: "#",
                    class: "font-heading text-lg font-bold tracking-[0.2em] text-white hover:text-amber-500 transition-colors",
                    "SEINLY."
                }
                div { class: "hidden md:flex gap-8",
                    a { href: "#about", class: NAV_LINK, "About" }
                    a { href: "#skills", class: NAV_LINK, "Skills" }
                    a { href: "#experience", class: NAV_LINK, "Work" }
                    a { href: "#education", class: NAV_LINK, "Education" }
                }
            }
        }

        Outlet::<Route> {}
    }
}
