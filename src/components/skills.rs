use dioxus::prelude::*;

struct SkillCategory {
    label: &'static str,
    skills: &'static [&'static str],
}

const CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        label: "LANGUAGES",
        skills: &["Go", "JavaScript", "PHP"],
    },
    SkillCategory {
        label: "FRONTEND",
        skills: &["SvelteKit", "Next.js", "React", "Tailwind CSS"],
    },
    SkillCategory {
        label: "BACKEND",
        skills: &["Node.js", "Laravel", "PostgreSQL", "WordPress"],
    },
    SkillCategory {
        label: "CLOUD & DEVOPS",
        skills: &["GCP", "Linux Server", "CI/CD"],
    },
    SkillCategory {
        label: "TOOLS",
        skills: &["REST APIs", "Git", "Clean Architecture"],
    },
    SkillCategory {
        label: "OTHER",
        skills: &["Responsive Design", "UI/UX Optimization"],
    },
];

#[component]
pub fn Skills() -> Element {
    rsx! {
        section { id: "skills", class: "py-24 px-6 bg-[#13151a]",
            div { class: "max-w-4xl mx-auto",
                div { class: "font-mono text-amber-500 text-sm mb-8 tracking-widest",
                    "// 003 — TECHNICAL ARSENAL"
                }
                h2 { class: "font-heading text-3xl font-bold text-white mb-12 tracking-wide uppercase",
                    "Skills & Tools"
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                    for category in CATEGORIES.iter() {
                        div { class: "space-y-3",
                            h3 { class: "font-mono text-xs text-amber-500/70 tracking-widest mb-3",
                                "{category.label}"
                            }
                            div { class: "flex flex-wrap gap-2",
                                for skill in category.skills.iter() {
                                    span { class: "border border-gray-700 px-3 py-1.5 text-sm font-body text-gray-300 hover:border-amber-500 hover:text-amber-500 transition-colors cursor-default",
                                        "{skill}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
