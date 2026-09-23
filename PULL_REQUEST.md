# 🔀 Pull Request: CulturaGo Stellar Ticket MVP (Web2.5 Ticketing & Soroban Smart Contracts)

## 📌 Resumen de Cambios
Este Pull Request introduce la arquitectura completa, contratos inteligentes en Soroban y documentación de producto para **CulturaGo Stellar Ticket**, una plataforma descentralizada de ticketing para eventos culturales de +200 personas con emisión antifraude y soporte de coleccionables para artistas en la red **Stellar**.

---

## 🎯 Motivación y Contexto
Desarrollado en el marco del workshop de flujos con IA y **Exponential CLI** (`/grill-me` -> `/to-prd` -> `/to-expo`).
* **Decisiones Registradas:**
  * `D-0001`: Entradas no transferibles y pagos multidivisa (XLM / Path Payments).
  * `D-0002`: Marketplace de NFTs de artistas con royalties opcionales (costo de gas).
  * `D-0003`: Onboarding Web2.5 con billeteras embebidas y validación QR.

---

## 🧱 Componentes Incluidos

### 1. Documentación de Producto y Arquitectura
* `PRD.md`: Documento de requerimientos de producto, casos de uso y métricas de éxito.
* `ARCHITECTURE.md`: Modelos de datos Soroban, diagrama de secuencia y flujo de vida del ticket.

### 2. Smart Contract Soroban (Rust)
* `Cargo.toml`: Configuración de dependencias con `soroban-sdk v22.0.0`.
* `src/lib.rs`: Smart contract `TicketContract` con métodos:
  * `initialize`: Configuración de aforo y precio.
  * `buy_ticket`: Emisión de ticket inmutable.
  * `check_in`: Validación de acceso y prevención de reuso.
  * `transfer_ticket`: Transferencia controlada.
  * `get_ticket` / `get_event_info`: Lectura pública de estado.

### 3. Suite de Pruebas y Simulación
* `src/test.rs`: Tests unitarios automatizados para compras, sold-out y detección de fraude.
* `scripts/simulate_ticket_flow.js`: Simulador interactivo en tiempo real con Node.js.

---

## 🧪 Plan de Verificación y Pruebas
1. **Ejecutar simulación local:**
   ```bash
   node scripts/simulate_ticket_flow.js
   ```
2. **Ejecutar tests unitarios de Soroban:**
   ```bash
   cargo test
   ```

---

## 👥 Checklist de Revisión para el Revisor
- [x] El aforo máximo se valida antes de cualquier emisión.
- [x] El check-in previene estrictamente la doble entrada con el flag `is_used`.
- [x] Los eventos de Stellar (`ticket:minted`, `ticket:checked`) se emiten correctamente.
- [x] La documentación refleja fielmente las decisiones tomadas en Exponential.
