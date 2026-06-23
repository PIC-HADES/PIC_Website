use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use std::sync::OnceLock;

use crate::pages::{home::HomePage, problema::ProblemaPage, solucao::SolucaoPage,
            viabilidade::ViabilidadePage, equipa::EquipaPage};
use crate::components::{navbar::Navbar, footer::Footer};

/// Detect the base URL path at runtime.
/// Production (GitHub Pages): /PIC_Website
/// Local dev (trunk serve): "" (empty = root)
fn router_base() -> &'static str {
    static BASE: OnceLock<&'static str> = OnceLock::new();
    BASE.get_or_init(|| {
        let pathname = leptos::web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        if pathname == "/PIC_Website" || pathname.starts_with("/PIC_Website/") {
            "/PIC_Website"
        } else {
            ""
        }
    })
}

#[component]
pub fn MissaoHadesApp() -> impl IntoView {
    view! {
        <Router base=router_base()>
            <div class="flex flex-col min-h-screen">
                <Navbar/>
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/problema") view=ProblemaPage/>
                        <Route path=path!("/solucao") view=SolucaoPage/>
                        <Route path=path!("/viabilidade") view=ViabilidadePage/>
                        <Route path=path!("/equipa") view=EquipaPage/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="min-h-screen flex items-center justify-center px-4">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-slate-600 mb-4">404</h1>
                <p class="text-xl text-slate-400 mb-8">Página não encontrada</p>
                <a href="/" class="btn-primary">Voltar ao início</a>
            </div>
        </section>
    }
}
