use leptos::prelude::*;
use crate::app::router_base;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-slate-800/50 bg-slate-950">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-2xl mx-auto">
                    {/* Brand */}
                    <div>
                        <div class="flex items-center gap-3 mb-4">
                            <img src="assets/mission-patch.png"
                                 alt="Patch Missão Hades"
                                 class="h-8 w-auto"
                            />
                            <span class="font-bold text-lg text-slate-100">MISSÃO HADES</span>
                        </div>
                        <p class="text-sm text-slate-400 leading-relaxed max-w-xs">
                            "Míssil de defesa anti-aéria de médio alcance"
                        </p>
                    </div>

                    {/* Links */}
                    <div class="md:text-right">
                        <h4 class="text-xs uppercase tracking-widest text-slate-500 font-semibold mb-4">
                            Navegação
                        </h4>
                        <ul class="space-y-2">
                            <li><a href={format!("{}/", router_base())} class="text-sm text-slate-400 hover:text-hades-400 transition-colors">Início</a></li>
                            <li><a href={format!("{}/problema", router_base())} class="text-sm text-slate-400 hover:text-hades-400 transition-colors">O Problema</a></li>
                            <li><a href={format!("{}/solucao", router_base())} class="text-sm text-slate-400 hover:text-hades-400 transition-colors">A Solução</a></li>
                            <li><a href={format!("{}/viabilidade", router_base())} class="text-sm text-slate-400 hover:text-hades-400 transition-colors">Viabilidade</a></li>
                            <li><a href={format!("{}/equipa", router_base())} class="text-sm text-slate-400 hover:text-hades-400 transition-colors">Equipa</a></li>
                        </ul>
                    </div>
                </div>

                <div class="mt-10 pt-6 border-t border-slate-800/50 text-center">
                    <p class="text-xs text-slate-600">
                        "2026 Missão Hades. Projeto académico."
                    </p>
                </div>
            </div>
        </footer>
    }
}
