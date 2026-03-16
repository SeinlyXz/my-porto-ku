use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
struct Job {
    company: &'static str,
    role: &'static str,
    project: &'static str,
    period: &'static str,
    details: &'static [&'static str],
}

const JOBS: &[Job] = &[
    Job {
        company: "Alodia Indonesia",
        role: "Full Stack Developer",
        project: "Alovote",
        period: "May 2025 — Present",
        details: &[
            "Built a large-scale paid voting system for live events",
            "Developed backend and frontend using Go, SvelteKit, Laravel, PostgreSQL",
            "Ensured high performance for real-time voting transactions",
        ],
    },
    Job {
        company: "PBNU",
        role: "Frontend Developer",
        project: "Siskader",
        period: "Sep 2025 — Nov 2025",
        details: &[
            "Fixed legacy issues and resolved critical bugs",
            "Implemented CI/CD pipelines",
            "Improved stability and user experience",
        ],
    },
    Job {
        company: "BeyondLabs",
        role: "Full Stack Developer",
        project: "Tiketix",
        period: "Sep 2025 — Oct 2025",
        details: &[
            "Improved responsiveness and UI design",
            "Built ticket validation backend system",
        ],
    },
    Job {
        company: "Rajawali Indonesia",
        role: "Full Stack Developer",
        project: "Bakoel Karcis",
        period: "Mar 2025 — Jul 2025",
        details: &[
            "Developed ticket and member verification systems",
            "Enhanced UI readability and accessibility",
        ],
    },
    Job {
        company: "NU CARE LAZISNU",
        role: "Frontend Developer",
        project: "Intern",
        period: "Jan 2025 — Feb 2025",
        details: &[
            "Improved layout stability and responsiveness",
            "Optimized performance across devices",
        ],
    },
    Job {
        company: "Rajawali Indonesia",
        role: "WordPress Developer",
        project: "Intern",
        period: "Jul 2022 — Dec 2022",
        details: &[
            "Built JogjaRockArtFestival 2022 official website",
            "Developed custom themes and plugins",
        ],
    },
];

#[component]
pub fn Experience() -> Element {
    rsx! {
        section { id: "experience", class: "py-24 px-6",
            div { class: "max-w-6xl mx-auto",
                div { class: "font-mono text-amber-500 text-sm mb-8 tracking-widest",
                    "// 004 — WORK HISTORY"
                }
                h2 { class: "font-heading text-3xl font-bold text-white mb-12 tracking-wide uppercase",
                    "Experience"
                }
                div { class: "space-y-6",
                    for (i , job) in JOBS.iter().enumerate() {
                        ExperienceCard { i: i as i32, props: job.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn ExperienceCard(i: i32, props: Job) -> Element {
    rsx! {
        div { class: "group border border-gray-800 bg-[#13151a] p-6 hover:border-amber-500/50 transition-colors",
            div { class: "flex justify-between items-start mb-4 flex-wrap gap-2",
                div {
                    div { class: "flex items-center gap-3 mb-1",
                        span { class: "font-mono text-amber-500/50 text-xs", {format!("{:02}", i + 1)} }
                        h3 { class: "font-heading text-xl font-bold text-white", "{props.company}" }
                    }
                    p { class: "font-body text-gray-400 text-sm ml-7",
                        "{props.role} — {props.project}"
                    }
                }
                span { class: "font-mono text-xs text-gray-500 shrink-0", "{props.period}" }
            }
            ul { class: "space-y-2 border-l border-gray-700 pl-4 ml-3",
                for detail in props.details.iter() {
                    li { class: "font-body text-gray-300 text-sm", "{detail}" }
                }
            }
        }
    }
}
