use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::pages::{home::HomePage, problema::ProblemaPage, solucao::SolucaoPage,
            viabilidade::ViabilidadePage, equipa::EquipaPage};
use crate::components::{navbar::Navbar, footer::Footer};

#[component]
pub fn MissaoHadesApp() -> impl IntoView {
    view! {
        <Router>
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
