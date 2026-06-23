use leptos::prelude::*;
use crate::app::router_base;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="relative min-h-[90vh] flex items-center justify-center overflow-hidden">
            {/* Background gradient */}
            <div class="absolute inset-0 bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950"></div>
            <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-hades-950/40 via-transparent to-transparent"></div>

            {/* Grid overlay */}
            <div class="absolute inset-0 opacity-[0.03]"
                 style="background-image: linear-gradient(rgba(255,255,255,.1) 1px, transparent 1px),
                        linear-gradient(90deg, rgba(255,255,255,.1) 1px, transparent 1px);
                        background-size: 60px 60px;">
            </div>

            <div class="relative z-10 max-w-5xl mx-auto px-4 text-center">
                {/* Mission Patch */}
                <div class="mb-8 flex justify-center">
                    <img src="assets/mission-patch.png"
                         alt="Patch da Missão Hades"
                         class="w-48 md:w-56 lg:w-64 h-auto drop-shadow-[0_0_30px_rgba(173,147,71,0.2)]"
                    />
                </div>

                {/* Badge */}
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-hades-500/30 bg-hades-500/10 text-hades-400 text-xs uppercase tracking-widest font-semibold mb-8">
                    <span class="w-1.5 h-1.5 rounded-full bg-hades-400 animate-pulse"></span>
                    Projeto Integrador de Curso
                </div>

                {/* Title */}
                <h1 class="text-5xl md:text-7xl lg:text-8xl font-extrabold tracking-tight mb-6">
                    <span class="text-slate-100">MISSÃO</span>
                    <span class="text-hades-400 block mt-2">HADES</span>
                </h1>

                {/* Subtitle */}
                <p class="text-lg md:text-xl text-slate-400 max-w-2xl mx-auto mb-10 leading-relaxed">
                    "O primeiro sistema de defesa anti-aérea de médio alcance
                    inteiramente projetado em Portugal."
                    <span class="block mt-2 text-slate-500">"Soberania. Autonomia. Viabilidade."</span>
                </p>

                {/* CTA Buttons */}
                <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
                    <a href={format!("{}/solucao", router_base())} class="btn-primary text-base px-8 py-4">
                        Explorar a solução
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                        </svg>
                    </a>
                    <a href={format!("{}/viabilidade", router_base())} class="btn-outline text-base px-8 py-4">
                        Ver viabilidade económica
                    </a>
                </div>

                {/* Stats Row */}
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 max-w-3xl mx-auto">
                    <div class="card py-8">
                        <div class="stat-number">50 km</div>
                        <div class="stat-label">Alcance efetivo</div>
                    </div>
                    <div class="card py-8">
                        <div class="stat-number">Mach 4</div>
                        <div class="stat-label">Velocidade máxima</div>
                    </div>
                    <div class="card py-8">
                        <div class="stat-number">"0,38M€"</div>
                        <div class="stat-label">Custo por unidade</div>
                    </div>
                </div>
            </div>
        </section>

        {/* Value Props */}
        <section class="py-20 bg-slate-900/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="section-title">Porquê a Missão Hades?</h2>
                    <p class="section-subtitle mx-auto">
                        "Três pilares que distinguem este projeto no panorama da defesa nacional."
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    {/* Pillar 1 */}
                    <div class="card text-center">
                        <div class="w-14 h-14 rounded-xl bg-hades-500/10 border border-hades-500/20 flex items-center justify-center mx-auto mb-6">
                            <svg class="w-7 h-7 text-hades-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-3">Soberania Nacional</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            Capacidade de defesa aérea projetada e produzida em Portugal, eliminando
                            dependências externas e garantindo controlo total sobre o ciclo de vida do sistema.
                        </p>
                    </div>

                    {/* Pillar 2 */}
                    <div class="card text-center">
                        <div class="w-14 h-14 rounded-xl bg-naval-500/10 border border-naval-500/20 flex items-center justify-center mx-auto mb-6">
                            <svg class="w-7 h-7 text-naval-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-3">Tecnologia Modular</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            "Arquitetura baseada em componentes COTS com software custom, permitindo
                            atualizações incrementais e adaptação a diferentes perfis de missão sem redesenvolver o sistema."
                        </p>
                    </div>

                    {/* Pillar 3 */}
                    <div class="card text-center">
                        <div class="w-14 h-14 rounded-xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center mx-auto mb-6">
                            <svg class="w-7 h-7 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-3">Viabilidade Económica</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            "Custo por unidade 10 vezes inferior a sistemas equivalentes no mercado,
                            viabilizando aquisições em escala e abrindo mercado de exportação para forças aliadas."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
