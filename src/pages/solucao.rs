use leptos::prelude::*;

#[component]
pub fn SolucaoPage() -> impl IntoView {
    view! {
        {/* Hero */}
        <section class="py-20 bg-gradient-to-b from-slate-900 to-slate-950">
            <div class="max-w-4xl mx-auto px-4 text-center">
                <div class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-naval-500/30 bg-naval-500/10 text-naval-400 text-xs uppercase tracking-widest font-semibold mb-6">
                    Arquitetura
                </div>
                <h1 class="text-4xl md:text-5xl font-bold tracking-tight mb-6">
                    A <span class="text-naval-400">Solução</span> Técnica
                </h1>
                <p class="text-lg text-slate-400 max-w-2xl mx-auto leading-relaxed">
                    Arquitetura modular baseada em componentes COTS com software custom,
                    concebida para evolução incremental e adaptação a múltiplos perfis de missão.
                </p>
            </div>
        </section>

        {/* Specs Table */}
        <section class="py-16">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-8 text-slate-100">
                    Especificações do Sistema
                </h2>
                <div class="overflow-x-auto">
                    <table class="w-full border-collapse">
                        <thead>
                            <tr class="border-b border-slate-700">
                                <th class="text-left py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">Parâmetro</th>
                                <th class="text-left py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">Valor</th>
                                <th class="text-left py-3 px-4 text-sm uppercase tracking-widest text-slate-500 font-semibold">Observações</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-800">
                            <SpecRow param="Alcance" value="50 km" note="Alcance efetivo contra alvos aéreos"/>
                            <SpecRow param="Velocidade" value="Mach 4" note="Sustentado em voo de cruzeiro"/>
                            <SpecRow param="Teto" value="12 000 m" note="Altitude operacional máxima"/>
                            <SpecRow param="Carga útil" value="15 kg" note="Fragmentação direcional"/>
                            <SpecRow param="Sistema de guiamento" value="INS+GNSS + IIR" note="Atualizável para SAR"/>
                            <SpecRow param="Comprimento" value="3,2 m" note="Lançamento tubular"/>
                            <SpecRow param="Diâmetro" value="180 mm" note="Compatível lançadores standard"/>
                            <SpecRow param="Peso lançamento" value="~90 kg" note="Inclui propelente"/>
                            <SpecRow param="Propulsão" value="2 estágios sólidos" note="Boost + sustentação"/>
                            <SpecRow param="Link de dados" value="FHSS 2,4 GHz" note="MAVLink encriptado, 50 km"/>
                        </tbody>
                    </table>
                </div>
            </div>
        </section>

        {/* Architecture Overview */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-12 text-slate-100">
                    Arquitetura Modular
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <ArchCard
                        title="Guiamento & Navegação"
                        items=vec![
                            "IMU VectorNav VN-210T (2× redundantes)",
                            "GNSS u-blox ZED-X20D (2×)",
                            "Filtro Kalman assimétrico",
                            "Fusão sensorial INS+GNSS",
                        ]
                        color="naval"
                    />
                    <ArchCard
                        title="Seeker & Guiamento Terminal"
                        items=vec![
                            "FLIR Boson 640 LWIR",
                            "Processamento IIR a bordo",
                            "Correlação de template",
                            "Handover mid-course→terminal",
                        ]
                        color="naval"
                    />
                    <ArchCard
                        title="Atuação & Controlo"
                        items=vec![
                            "Servos Hitec SR33BL-CAN (4×)",
                            "CAN-FD redundante",
                            "Controlo PID adaptativo",
                            "Atuadores de superfície + TVC",
                        ]
                        color="hades"
                    />
                    <ArchCard
                        title="Computação & Comunicação"
                        items=vec![
                            "Jetson Orin NX (16 GB)",
                            "STM32H753XI para controlo crítico",
                            "Rádio FHSS 2,4 GHz",
                            "MAVLink encriptado",
                        ]
                        color="hades"
                    />
                </div>
            </div>
        </section>

        {/* Block Diagram */}
        <section class="py-16">
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                <h2 class="text-2xl md:text-3xl font-bold mb-6 text-slate-100">
                    Diagrama de Blocos - Arquitetura de Aviónica
                </h2>
                <p class="text-slate-400 mb-8 max-w-2xl mx-auto">
                    Arquitetura dual-bus com barramento de controlo (CAN-FD) e barramento táctico (Ethernet),
                    isolados galvanicamente para prevenção de falhas em cascata.
                </p>
                <div class="card p-8 inline-block">
                    <svg class="w-full max-w-3xl mx-auto" viewBox="0 0 800 500" fill="none" xmlns="http://www.w3.org/2000/svg">
                        {/* Background */}
                        <rect width="800" height="500" rx="12" fill="#0f172a"/>

                        {/* Title */}
                        <text x="400" y="30" text-anchor="middle" fill="#94a3b8" font-family="JetBrains Mono, monospace" font-size="11" font-weight="600" letter-spacing="2">ARQUITETURA DE AVIONICA - MISSAO HADES</text>

                        {/* Dual bus */}
                        <line x1="60" y1="240" x2="740" y2="240" stroke="#475569" stroke-width="2" stroke-dasharray="6,3"/>
                        <text x="400" y="235" text-anchor="middle" fill="#64748b" font-family="monospace" font-size="9">--- Tactical Bus (Ethernet, 1 Gbps) ---</text>
                        <line x1="60" y1="370" x2="740" y2="370" stroke="#475569" stroke-width="2"/>
                        <text x="400" y="385" text-anchor="middle" fill="#64748b" font-family="monospace" font-size="9">--- Control Bus (CAN-FD, 8 Mbps) ---</text>

                        {/* Jetson */}
                        <rect x="300" y="55" width="200" height="65" rx="8" fill="#1e293b" stroke="#f59e0b" stroke-width="1.5"/>
                        <text x="400" y="80" text-anchor="middle" fill="#fbbf24" font-family="monospace" font-size="12" font-weight="bold">Jetson Orin NX</text>
                        <text x="400" y="97" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">Perception & Guidance</text>
                        <text x="400" y="110" text-anchor="middle" fill="#64748b" font-family="monospace" font-size="8">Linux, 16 GB, 100 TOPS</text>

                        {/* STM32 */}
                        <rect x="60" y="395" width="180" height="50" rx="8" fill="#1e293b" stroke="#3b82f6" stroke-width="1.5"/>
                        <text x="150" y="420" text-anchor="middle" fill="#60a5fa" font-family="monospace" font-size="11" font-weight="bold">STM32H753XI</text>
                        <text x="150" y="436" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">Control Crítico (FSM, Atuadores)</text>

                        {/* CAN connection STM32 → Control Bus */}
                        <line x1="150" y1="395" x2="150" y2="370" stroke="#475569" stroke-width="1.5"/>

                        {/* Servos */}
                        <rect x="560" y="395" width="180" height="50" rx="8" fill="#1e293b" stroke="#f59e0b" stroke-width="1.5"/>
                        <text x="650" y="420" text-anchor="middle" fill="#fbbf24" font-family="monospace" font-size="11" font-weight="bold">4x Hitec SR33BL-CAN</text>
                        <text x="650" y="436" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">Atuadores (PID, CAN-FD)</text>

                        {/* CAN connection Servos → Control Bus */}
                        <line x1="650" y1="395" x2="650" y2="370" stroke="#475569" stroke-width="1.5"/>

                        {/* FLIR Boson */}
                        <rect x="60" y="55" width="200" height="65" rx="8" fill="#1e293b" stroke="#ef4444" stroke-width="1.5"/>
                        <text x="160" y="80" text-anchor="middle" fill="#f87171" font-family="monospace" font-size="12" font-weight="bold">FLIR Boson 640</text>
                        <text x="160" y="97" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">LWIR Seeker</text>
                        <text x="160" y="110" text-anchor="middle" fill="#64748b" font-family="monospace" font-size="8">640x512, 60 Hz</text>

                        {/* IMU */}
                        <rect x="540" y="55" width="200" height="65" rx="8" fill="#1e293b" stroke="#22c55e" stroke-width="1.5"/>
                        <text x="640" y="80" text-anchor="middle" fill="#4ade80" font-family="monospace" font-size="11" font-weight="bold">VN-210T x 2</text>
                        <text x="640" y="97" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">IMU Redundante</text>
                        <text x="640" y="110" text-anchor="middle" fill="#64748b" font-family="monospace" font-size="8">INS+GNSS, AHRS</text>

                        {/* GNSS */}
                        <rect x="540" y="135" width="200" height="50" rx="8" fill="#1e293b" stroke="#22c55e" stroke-width="1.5"/>
                        <text x="640" y="155" text-anchor="middle" fill="#4ade80" font-family="monospace" font-size="11" font-weight="bold">ZED-X20D x 2</text>
                        <text x="640" y="171" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">GNSS Multibanda</text>

                        {/* Rádio */}
                        <rect x="60" y="135" width="200" height="50" rx="8" fill="#1e293b" stroke="#a855f7" stroke-width="1.5"/>
                        <text x="160" y="155" text-anchor="middle" fill="#c084fc" font-family="monospace" font-size="11" font-weight="bold">Radio FHSS</text>
                        <text x="160" y="171" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">2.4 GHz, MAVLink</text>

                        {/* Connections to Jetson */}
                        <line x1="160" y1="120" x2="300" y2="87" stroke="#475569" stroke-width="1"/>
                        <line x1="640" y1="120" x2="500" y2="87" stroke="#475569" stroke-width="1"/>
                        <line x1="640" y1="185" x2="500" y2="87" stroke="#475569" stroke-width="1" stroke-dasharray="4,2"/>
                        <line x1="160" y1="185" x2="300" y2="87" stroke="#475569" stroke-width="1" stroke-dasharray="4,2"/>

                        {/* Jetson to Tactical Bus */}
                        <line x1="400" y1="120" x2="400" y2="240" stroke="#475569" stroke-width="1.5"/>

                        {/* Battery */}
                        <rect x="60" y="290" width="160" height="45" rx="8" fill="#1e293b" stroke="#f59e0b" stroke-width="1.5"/>
                        <text x="140" y="310" text-anchor="middle" fill="#fbbf24" font-family="monospace" font-size="10" font-weight="bold">Battery + BMS</text>
                        <text x="140" y="326" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">28V DC, 3.2 kWh</text>

                        {/* Power lines */}
                        <line x1="140" y1="290" x2="140" y2="240" stroke="#64748b" stroke-width="1" stroke-dasharray="3,3"/>
                        <text x="130" y="265" text-anchor="end" fill="#64748b" font-family="monospace" font-size="8">Power</text>

                        {/* Legend */}
                        <rect x="290" y="450" width="220" height="35" rx="6" fill="#1e293b" stroke="#334155" stroke-width="1"/>
                        <text x="400" y="470" text-anchor="middle" fill="#94a3b8" font-family="monospace" font-size="9">Legenda: LINHA - Dados | TRACEJADO - Link | PONTO - Power</text>
                    </svg>
                </div>
            </div>
        </section>

        {/* Key Features */}
        <section class="py-16 bg-slate-900/50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl md:text-3xl font-bold text-center mb-12 text-slate-100">
                    Inovações Técnicas
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <FeatureCard
                        title="Redundância Inteligente"
                        desc="IMU e GNSS duplicados com lógica de arbitragem no STM32. Falha de um sensor = comutação transparente sem perda de missão."
                    />
                    <FeatureCard
                        title="Arquitetura Dual-Bus"
                        desc="Barramento táctil (Ethernet) isolado do barramento de controlo (CAN-FD). Falhas no subsistema de percepção não afetam o controlo de voo."
                    />
                    <FeatureCard
                        title="FSM Autónoma"
                        desc="Máquina de estados com 5 fases (Pré-Lançamento → Boost → Cruzeiro → Terminal → Fuzagem). Abort autónomo em qualquer fase."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn SpecRow(param: &'static str, value: &'static str, note: &'static str) -> impl IntoView {
    view! {
        <tr class="hover:bg-slate-800/30 transition-colors">
            <td class="py-3 px-4 text-slate-300 font-medium">{param}</td>
            <td class="py-3 px-4 font-mono text-hades-400 font-semibold">{value}</td>
            <td class="py-3 px-4 text-slate-500 text-sm">{note}</td>
        </tr>
    }
}

#[component]
fn ArchCard(title: &'static str, items: Vec<&'static str>, color: &'static str) -> impl IntoView {
    let border = match color {
        "naval" => "border-naval-500/20 hover:border-naval-500/40",
        "hades" => "border-hades-500/20 hover:border-hades-500/40",
        _ => "border-slate-700",
    };

    let dot = match color {
        "naval" => "bg-naval-400",
        "hades" => "bg-hades-400",
        _ => "bg-slate-400",
    };

    view! {
        <div class=format!("card {}", border)>
            <h3 class="text-base font-bold text-slate-100 mb-4">{title}</h3>
            <ul class="space-y-2">
                {items.into_iter().map(|item| {
                    view! {
                        <li class="flex items-start gap-2 text-sm text-slate-400">
                            <span class=format!("mt-1.5 w-1.5 h-1.5 rounded-full {} shrink-0", dot)></span>
                            {item}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn FeatureCard(title: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="card">
            <h3 class="text-lg font-bold text-slate-100 mb-2">{title}</h3>
            <p class="text-sm text-slate-400 leading-relaxed">{desc}</p>
        </div>
    }
}
