use leptos::prelude::*;

#[component]
pub fn ProblemaPage() -> impl IntoView {
    view! {
        {/* Hero Section */}
        <section class="py-20 bg-gradient-to-b from-slate-900 to-slate-950">
            <div class="max-w-4xl mx-auto px-4 text-center">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-red-500/30 bg-red-500/10 text-red-400 text-xs uppercase tracking-widest font-semibold mb-6">
                    Contexto
                </div>
                <h1 class="text-4xl md:text-5xl font-bold tracking-tight mb-6">
                    O <span class="text-red-400">Problema</span> da Defesa Aérea
                </h1>
                <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                    Portugal enfrenta uma lacuna crítica na sua capacidade de defesa aérea de médio alcance,
                    num contexto geopolítico europeu cada vez mais exigente.
                </p>
            </div>
        </section>

        {/* The Gap */}
        <section class="py-16">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div>
                        <h2 class="text-2xl md:text-3xl font-bold mb-6 text-slate-100">
                            A lacuna portuguesa
                        </h2>
                        <div class="space-y-4 text-slate-400 leading-relaxed">
                            <p>
                                Desde a desativação dos últimos sistemas de defesa aérea de médio alcance,
                                Portugal não dispõe de capacidade autónoma para proteger o seu espaço aéreo
                                além do alcance visual (BVR).
                            </p>
                            <p>
                                As opções atuais resumem-se a:
                            </p>
                            <ul class="space-y-3 pl-5">
                                <li class="flex items-start gap-3">
                                    <span class="text-red-400 mt-1 shrink-0">{"•"}</span>
                                    <span><strong class="text-slate-200">"Dependência de parceiros NATO"</strong>
                                        " — sem garantia de disponibilidade em cenários de crise simultânea."</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-red-400 mt-1 shrink-0">{"•"}</span>
                                    <span><strong class="text-slate-200">"Sistemas de curto alcance"</strong>
                                        " — insuficientes para cobrir áreas estratégicas como o Atlântico ou o Algarve."</span>
                                </li>
                                <li class="flex items-start gap-3">
                                    <span class="text-red-400 mt-1 shrink-0">{"•"}</span>
                                    <span><strong class="text-slate-200">"Aquisição externa a custos proibitivos"</strong>
                                        " — PAC-3 a €3,5M/unidade, IRIS-T SLM a €450k mas com dependência alemã."</span>
                                </li>
                            </ul>
                        </div>
                    </div>

                    {/* Cost Comparison Cards */}
                    <div class="space-y-6">
                        <h3 class="text-sm uppercase tracking-widest text-slate-500 font-semibold text-center mb-6">
                            Comparação de custos por míssil
                        </h3>
                        <ComparisonBar
                            name="PAC-3 (EUA)"
                            cost="€ 3 500 000"
                            percentage=100u32
                            color="red"
                            is_ours=false
                        />
                        <ComparisonBar
                            name="IRIS-T SLM (Alemanha)"
                            cost="€ 450 000"
                            percentage=13u32
                            color="amber"
                            is_ours=false
                        />
                        <ComparisonBar
                            name="Missão Hades"
                            cost="€ 347 000"
                            percentage=10u32
                            color="emerald"
                            is_ours=true
                        />
                    </div>
                </div>
            </div>
        </section>

        {/* Consequences */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-12 text-slate-100">
                    Consequências da dependência
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="card">
                        <div class="w-12 h-12 rounded-lg bg-red-500/10 border border-red-500/20 flex items-center justify-center mb-5">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                    d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-2">Risco Geopolítico</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            "Numa crise europeia alargada, a disponibilidade de sistemas estrangeiros é incerta —"
                            "cada país prioriza a sua própria defesa."
                        </p>
                    </div>
                    <div class="card">
                        <div class="w-12 h-12 rounded-lg bg-red-500/10 border border-red-500/20 flex items-center justify-center mb-5">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-2">Custo de Aquisição</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            "Os sistemas de médio alcance no mercado internacional custam entre €3,5M e €10M por"
                            "unidade, incomportáveis para aquisição em escala."
                        </p>
                    </div>
                    <div class="card">
                        <div class="w-12 h-12 rounded-lg bg-red-500/10 border border-red-500/20 flex items-center justify-center mb-5">
                            <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                    d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
                            </svg>
                        </div>
                        <h3 class="text-lg font-bold text-slate-100 mb-2">Dependência Logística</h3>
                        <p class="text-sm text-slate-400 leading-relaxed">
                            Manutenção, peças e atualizações dependem de fornecedores estrangeiros, criando
                            vulnerabilidades estratégicas a longo prazo.
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ComparisonBar(
    name: &'static str,
    cost: &'static str,
    percentage: u32,
    color: &'static str,
    is_ours: bool,
) -> impl IntoView {
    let bar_color = match color {
        "red" => "bg-red-500",
        "amber" => "bg-amber-500",
        "emerald" => "bg-emerald-400",
        _ => "bg-slate-500",
    };

    let bg_class = if is_ours { "card-highlight" } else { "card" };

    view! {
        <div class=bg_class>
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-semibold text-slate-200">{name}</span>
                <span class="text-sm font-mono text-slate-300">{cost}</span>
            </div>
            <div class="w-full h-3 bg-slate-800 rounded-full overflow-hidden">
                <div
                    class=format!("h-full {} rounded-full transition-all duration-1000", bar_color)
                    style=format!("width: {}%", percentage)
                ></div>
            </div>
        </div>
    }
}
