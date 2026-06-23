use leptos::prelude::*;
use crate::app::router_base;

const NAV_ITEMS: &[(&str, &str)] = &[
    ("/",          "Início"),
    ("/problema",  "O Problema"),
    ("/solucao",   "A Solução"),
    ("/viabilidade", "Viabilidade"),
];

#[component]
pub fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 bg-slate-950/90 backdrop-blur-md border-b border-slate-800/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    {/* Logo */}
                    <a href={format!("{}/", router_base())} class="flex items-center gap-3 group shrink-0">
                        <img src={format!("{}/assets/mission-patch.png", router_base())}
                             alt="Patch Missão Hades"
                             class="h-7 w-auto"
                        />
                        <span class="font-bold text-lg tracking-tight text-slate-100 group-hover:text-hades-400 transition-colors">
                            "MISSÃO HADES"
                        </span>
                    </a>

                    {/* Desktop Nav */}
                    <div class="hidden md:flex items-center gap-8">
                        {NAV_ITEMS.iter().map(|(path, label)| {
                            let href = format!("{}{}", router_base(), path);
                            view! {
                                <a href=href class="text-slate-400 hover:text-hades-400 transition-colors duration-150 text-sm font-medium">
                                    {*label}
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    {/* Mobile Hamburger */}
                    <button
                        on:click=move |_| set_menu_open.update(|v| *v = !*v)
                        class="md:hidden p-2 rounded-lg text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition-colors"
                        aria-label="Menu"
                    >
                        {move || {
                            if menu_open.get() {
                                view! { <XIcon/> }.into_any()
                            } else {
                                view! { <MenuIcon/> }.into_any()
                            }
                        }}
                    </button>
                </div>
            </div>

            {/* Mobile Menu */}
            {move || {
                if menu_open.get() {
                    view! {
                        <div class="md:hidden border-t border-slate-800 bg-slate-950/95 backdrop-blur-md">
                            <div class="px-4 py-4 space-y-2">
                                {NAV_ITEMS.iter().map(|(path, label)| {
                                    let href = format!("{}{}", router_base(), path);
                                    view! {
                                        <a href=href
                                            class="block px-4 py-3 rounded-lg text-slate-300 hover:bg-slate-800 hover:text-hades-400 transition-colors text-base font-medium"
                                        >
                                            {*label}
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </nav>
        {/* Spacer for fixed navbar */}
        <div class="h-16"></div>
    }
}

#[component]
fn MenuIcon() -> impl IntoView {
    view! {
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M4 6h16M4 12h16M4 18h16"/>
        </svg>
    }
}

#[component]
fn XIcon() -> impl IntoView {
    view! {
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"/>
        </svg>
    }
}
