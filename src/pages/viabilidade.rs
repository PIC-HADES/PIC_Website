use leptos::prelude::*;

#[component]
pub fn ViabilidadePage() -> impl IntoView {
    view! {
        {/* Hero */}
        <section class="py-20 bg-gradient-to-b from-slate-900 to-slate-950">
            <div class="max-w-4xl mx-auto px-4 text-center">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-emerald-500/30 bg-emerald-500/10 text-emerald-400 text-xs uppercase tracking-widest font-semibold mb-6">
                    Economia
                </div>
                <h1 class="text-4xl md:text-5xl font-bold tracking-tight mb-6">
                    <span class="text-emerald-400">Viabilidade</span>" Económica"
                </h1>
                <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                    "Um sistema 10× mais barato que a concorrência, viabilizando aquisições em escala"
                    " e abrindo mercado de exportação. A conta final: " <strong class="text-emerald-400">"€347 000 por míssil"</strong>"."
                </p>
            </div>
        </section>

        {/* Cost Breakdown */}
        <section class="py-16">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-8 text-slate-100">
                    Estrutura de Custos
                </h2>
                <p class="text-center text-slate-400 mb-10 max-w-xl mx-auto">
                    Decomposição do custo unitário por subsistema.
                </p>
                <div class="overflow-x-auto">
                    <table class="w-full border-collapse">
                        <thead>
                            <tr class="border-b border-slate-700">
                                <th class="text-left py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">Subsistema</th>
                                <th class="text-right py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">"Custo (€)"</th>
                                <th class="text-right py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">% Total</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800">
                            <CostRow item="Materiais" cost="13 000" pct=6.1 />
                            <CostRow item="Aviónica" cost="41 000" pct=19.3 />
                            <CostRow item="Subsistemas" cost="133 000" pct=62.7 />
                            <CostRow item="Fabrico" cost="25 000" pct=11.8 />
                        </tbody>
                        <tfoot>
                            <tr class="border-t-2 border-hades-500">
                                <td class="py-4 px-4 font-bold text-slate-100">Total por míssil</td>
                                <td class="py-4 px-4 text-right font-mono font-bold text-hades-400 text-lg">"€212 000"</td>
                                <td class="py-4 px-4 text-right font-mono font-bold text-hades-400 text-lg">100%</td>
                            </tr>
                        </tfoot>
                    </table>
                </div>
            </div>
        </section>

        {/* Competition Comparison */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-12 text-slate-100">
                    Comparação com Concorrência
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <CompetitorCard
                        name="PAC-3 MSE"
                        origin="EUA (Lockheed Martin)"
                        cost="3.5M€"
                        multiple="10× mais caro"
                        color="red"
                        is_ours=false
                    />
                    <CompetitorCard
                        name="IRIS-T SLM"
                        origin="Alemanha (Diehl)"
                        cost="€ 380 000"
                        multiple="1,3× mais caro"
                        color="amber"
                        is_ours=false
                    />
                    <CompetitorCard
                        name="Missão Hades"
                        origin="Portugal"
                        cost="0,38M€"
                        multiple="Referência"
                        color="emerald"
                        is_ours=true
                    />
                </div>
            </div>
        </section>

        {/* Market & Scale */}
        <section class="py-16">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-start">
                    <div>
                        <h2 class="text-2xl md:text-3xl font-bold mb-6 text-slate-100">
                            Mercado Potencial
                        </h2>
                        <div class="space-y-4 text-slate-400 leading-relaxed">
                            <p>
                                "O mercado global de mísseis antiaéreos deverá atingir " <strong class="text-slate-200">"€45 mil milhões"</strong>
                                " anuais até 2035, impulsionado pelo aumento dos orçamentos de defesa na Europa e Ásia."
                            </p>
                            <p>
                                "Com um custo unitário 10× inferior ao PAC-3, a Missão Hades posiciona-se para capturar"
                                " o segmento de média distância — estimado em 15-20% do mercado total."
                            </p>
                            <ul class="space-y-3 pt-4">
                                <li class="flex items-start gap-3">
                                    <span class="text-emerald-400 mt-1 shrink-0">{"→"}</span>
                                    <span><strong class="text-slate-200">"Aquisição nacional:"</strong> " 48 mísseis para dotar 2 baterias — €16,7M"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-emerald-400 mt-1 shrink-0">{"→"}</span>
                                    <span><strong class="text-slate-200">"Mercado europeu (PESCO):"</strong> " framework cooperativo para 8+ nações"</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-emerald-400 mt-1 shrink-0">{"→"}</span>
                                    <span><strong class="text-slate-200">"Curva de aprendizagem:"</strong> " -15% custo por duplicação de produção (100+ unidades)"</span>
                                </li>
                            </ul>
                        </div>
                    </div>

                    {/* EU framework */}
                    <div class="card-highlight">
                        <div class="flex items-center gap-3 mb-6">
                            <div class="w-12 h-12 rounded-xl bg-naval-500/10 border border-naval-500/20 flex items-center justify-center">
                                <svg class="w-6 h-6 text-naval-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                        d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                                </svg>
                            </div>
                            <div>
                                <h3 class="text-lg font-bold text-slate-100">Modelo Cooperado Europeu</h3>
                                <p class="text-sm text-slate-400">Framework PESCO</p>
                            </div>
                        </div>
                        <div class="space-y-3 text-sm text-slate-400 leading-relaxed">
                            <p>
                                A Missão Hades alinha-se com os objetivos da <strong class="text-slate-200">Cooperação Estruturada Permanente (PESCO)</strong>
                                da UE, que promove projetos colaborativos de defesa entre estados-membros.
                            </p>
                            <p>
                                O modelo de desenvolvimento modular permite que cada país parceiro contribua com
                                subsistemas especializados (propulsão, seeker, C2), partilhando custos de I&D e
                                beneficiando de economias de escala na produção.
                            </p>
                            <p class="pt-2 text-slate-300">
                                "Investimento inicial estimado: " <strong class="text-emerald-400">"€28M"</strong> " para I&D, prototipagem e certificação,"
                                " repartido por 4-6 parceiros europeus."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        {/* TCO */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                <h2 class="text-2xl md:text-3xl font-bold mb-8 text-slate-100">
                    Custo Total de Propriedade (10 anos)
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                    <div class="card py-8">
                        <div class="text-xs uppercase tracking-widest text-slate-500 mb-2">Aquisição (48 unidades)</div>
                        <div class="stat-number text-emerald-400">"€16,7M"</div>
                    </div>
                    <div class="card py-8">
                        <div class="text-xs uppercase tracking-widest text-slate-500 mb-2">Operação & Manutenção</div>
                        <div class="stat-number text-emerald-400">"€4,2M"</div>
                    </div>
                    <div class="card py-8">
                        <div class="text-xs uppercase tracking-widest text-slate-500 mb-2">Pessoal & Infraestrutura</div>
                        <div class="stat-number text-emerald-400">"€6,8M"</div>
                    </div>
                </div>
                <div class="card-highlight inline-block py-6 px-10">
                    <span class="text-sm uppercase tracking-widest text-slate-400">TCO total 10 anos</span>
                    <span class="block text-3xl font-mono font-bold text-hades-400 mt-2">"€ 27,7M"</span>
                    <span class="text-sm text-slate-500 mt-1 block">Comparável a ~8 mísseis PAC-3</span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CostRow(item: &'static str, cost: &'static str, pct: f64) -> impl IntoView {
    view! {
        <tr class="hover:bg-slate-800/30 transition-colors">
            <td class="py-3 px-4 text-slate-300">{item}</td>
            <td class="py-3 px-4 text-right font-mono text-slate-200">"€ " {cost}</td>
            <td class="py-3 px-4 text-right font-mono text-slate-400">{pct}%</td>
        </tr>
    }
}

#[component]
fn CompetitorCard(
    name: &'static str,
    origin: &'static str,
    cost: &'static str,
    multiple: &'static str,
    color: &'static str,
    is_ours: bool,
) -> impl IntoView {
    let border = match color {
        "red" => "border-red-500/30 hover:border-red-500/50",
        "amber" => "border-amber-500/30 hover:border-amber-500/50",
        "emerald" => "border-emerald-400/30 hover:border-emerald-400/50",
        _ => "border-slate-700",
    };

    let cost_color = match color {
        "red" => "text-red-400",
        "amber" => "text-amber-400",
        "emerald" => "text-emerald-400",
        _ => "text-slate-200",
    };

    let bg = if is_ours {
        "bg-emerald-500/5"
    } else {
        "bg-slate-900/80"
    };

    view! {
        <div class=format!("{} card {}", bg, border)>
            <div class="text-xs uppercase tracking-widest text-slate-500 mb-1">{origin}</div>
            <h3 class="text-lg font-bold text-slate-100 mb-3">{name}</h3>
            <div class=format!("text-2xl font-mono font-bold {}", cost_color)>
                {cost}
            </div>
            <div class="text-sm text-slate-500 mt-2">{multiple}</div>
        </div>
    }
}
