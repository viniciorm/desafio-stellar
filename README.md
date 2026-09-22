# 🎟️ CulturaGo Stellar Ticket

Sistema descentralizado de emisión, gestión y validación de entradas para eventos culturales en la red **Stellar** utilizando smart contracts con **Soroban**, desarrollado con flujos colaborativos **Humano + IA** utilizando **Exponential CLI**.

---

## 🚀 Estructura del Proyecto

```text
├── ARCHITECTURE.md                 # Especificación técnica y diseño del sistema
├── Cargo.toml                      # Configuración de dependencias y Soroban SDK
├── src/
│   ├── lib.rs                      # Smart contract principal en Rust (TicketContract)
│   └── test.rs                     # Suite de pruebas unitarias
└── scripts/
    └── simulate_ticket_flow.js     # Simulador de compra, sold-out y validación check-in
```

---

## 🛠️ Tecnologías Utilizadas

* **Stellar / Soroban SDK (v22):** Smart Contracts en Rust.
* **Exponential CLI:** Gestión ágil de tareas, OKRs y sincronización entre humanos y agentes de IA.
* **Node.js:** Scripts de simulación y testing interactivo.

---

## 📋 Resumen del Flujo de Trabajo con Exponential CLI

1. **Definición de Tareas:**
   * `exponential actions create -n "Definir arquitectura de CulturaGo Stellar Ticket" ...`
   * `exponential actions create -n "Implementar contrato Soroban para emisión y validación" ...`
   * `exponential actions create -n "Crear tests unitarios y simulación de compra" ...`
2. **Ciclo de Desarrollo:**
   * Actualización de estados en tiempo real (`IN_PROGRESS` -> `DONE`).
3. **Validación:**
   * Simulación interactiva con `node scripts/simulate_ticket_flow.js`.

---

## 🧪 Ejecutar Simulación

```bash
node scripts/simulate_ticket_flow.js
```
