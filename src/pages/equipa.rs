use leptos::prelude::*;

#[component]
pub fn EquipaPage() -> impl IntoView {
    view! {
        {/* Hero */}
        <section class="py-20 bg-gradient-to-b from-slate-900 to-slate-950">
            <div class="max-w-4xl mx-auto px-4 text-center">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-hades-500/30 bg-hades-500/10 text-hades-400 text-xs uppercase tracking-widest font-semibold mb-6">
                    Equipa
                </div>
                <h1 class="text-4xl md:text-5xl font-bold tracking-tight mb-6">
                    A <span class="text-hades-400">Equipa</span>
                </h1>
                <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                    "Projeto Integrador de Curso — um grupo multidisciplinar de engenharia dedicado"
                    " a tornar Portugal soberano na defesa aérea de médio alcance."
                </p>
            </div>
        </section>

        {/* Team */}
        <section class="py-16">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <TeamMemberCard role="Sistemas de Aviónica" desc="Arquitetura de hardware embarcado, seleção de sensores e atuadores, integração Jetson + STM32, link budget e comunicações.">
                        <svg class="w-8 h-8 text-hades-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                d="M10.5 1.5H8.25A2.25 2.25 0 006 3.75v16.5a2.25 2.25 0 002.25 2.25h7.5A2.25 2.25 0 0018 20.25V3.75a2.25 2.25 0 00-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3"/>
                        </svg>
                    </TeamMemberCard>
                    <TeamMemberCard role="Propulsão & Aerodinâmica" desc="Motor de 2 estágios sólidos, perfil de voo supersónico, otimização aerodinâmica BVR e envelope de desempenho.">
                        <svg class="w-8 h-8 text-hades-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                d="M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z"/>
                        </svg>
                    </TeamMemberCard>
                    <TeamMemberCard role="Sistemas de Armas & Estrutura" desc="Carga útil de fragmentação, estrutura do míssil, sistema de fuzagem segura/armada e integração do lançador.">
                        <svg class="w-8 h-8 text-hades-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                d="M9 3.75H6.912a2.25 2.25 0 00-2.15 1.588L2.35 13.177a2.25 2.25 0 00-.1.661V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 00-2.15-1.588H15M2.25 13.5h3.86a2.25 2.25 0 012.012 1.244l.256.512a2.25 2.25 0 002.013 1.244h3.218a2.25 2.25 0 002.013-1.244l.256-.512a2.25 2.25 0 012.013-1.244h3.859"/>
                        </svg>
                    </TeamMemberCard>
                </div>
            </div>
        </section>

        {/* Why Invest */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-8 text-slate-100">
                    Porquê Investir?
                </h2>
                <div class="space-y-6 max-w-3xl mx-auto">
                    <InvestmentPoint
                        number="01"
                        title="Lacuna de Mercado"
                        desc="Portugal e vários aliados NATO não dispõem de defesa aérea de médio alcance — um segmento negligenciado entre sistemas portáteis (MANPADS) e sistemas de longo alcance (PAC-3, THAAD)."
                    />
                    <InvestmentPoint
                        number="02"
                        title="Vantagem Competitiva"
                        desc="Custo 10× inferior ao PAC-3 com desempenho equivalente para 90% dos cenários táticos. Arquitetura modular permite atualizações sem redesenvolvimento."
                    />
                    <InvestmentPoint
                        number="03"
                        title="Enquadramento Europeu"
                        desc="Alinhado com a PESCO e o Fundo Europeu de Defesa (EDF), com acesso a financiamento comunitário para I&D e prototipagem colaborativa."
                    />
                    <InvestmentPoint
                        number="04"
                        title="Spin-off Comercial"
                        desc="Tecnologia de guiamento INS+GNSS e comunicações FHSS aplicável a drones, UGV e robótica industrial — mercado endereçável de €2MM+."
                    />
                </div>
            </div>
        </section>


    }
}

#[component]
fn TeamMemberCard(role: &'static str, desc: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="card text-center">
            <div class="w-16 h-16 rounded-xl bg-hades-500/10 border border-hades-500/20 flex items-center justify-center mx-auto mb-5">
                {children()}
            </div>
            <h3 class="text-lg font-bold text-slate-100 mb-3">{role}</h3>
            <p class="text-sm text-slate-400 leading-relaxed">{desc}</p>
        </div>
    }
}

#[component]
fn InvestmentPoint(number: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="card flex items-start gap-4">
            <div class="text-2xl font-mono font-bold text-hades-500 shrink-0 w-10">{number}</div>
            <div>
                <h3 class="text-base font-bold text-slate-100 mb-1">{title}</h3>
                <p class="text-sm text-slate-400 leading-relaxed">{desc}</p>
            </div>
        </div>
    }
}
