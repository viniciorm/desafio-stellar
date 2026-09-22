# 📄 PRD: CulturaGo Stellar Ticket

**Versión:** 1.0.0  
**Estado:** Aprobado  
**Enfoque:** Web2.5 Cultural Ticketing & Artist NFT Platform on Stellar  

---

## 1. 📌 Resumen Ejecutivo y Visión del Producto

**CulturaGo Stellar Ticket** es una plataforma Web2.5 de ticketing y coleccionables digitales para eventos culturales (obras de teatro, recitales, festivales) con aforo de más de 200 personas. 

Aprovecha la velocidad, bajas comisiones y capacidades de emisión de activos de **Stellar y Soroban** para ofrecer:
1. **Ticketing Antifraude:** Entradas no transferibles vinculadas al comprador para erradicar la reventa especulativa.
2. **Pagos Multidivisa:** Compra con XLM, stablecoins (USDC en Stellar) o moneda local (fiat) mediante Stellar Path Payments.
3. **Experiencia Web2.5 Sin Fricción:** Autenticación social (Google / Email) con billeteras embebidas/custodiadas en segundo plano.
4. **Economía del Artista (NFT Marketplace):** Venta de coleccionables y arte digital de los artistas del evento con soporte de royalties opcionales (cobrando únicamente el costo de red/gas).
5. **Validación Rápida con QR:** Check-in en puerta mediante escaneo de códigos QR criptográficos dinámicos.

---

## 2. 👥 Personas y Casos de Uso

| Persona | Rol | Necesidad Principal |
| :--- | :--- | :--- |
| **Audiencia Cultural (Elena, 28)** | Asistente al evento | Comprar su entrada con tarjeta o cripto en segundos, sin crear billeteras complejas, y guardar su NFT del artista como recuerdo. |
| **Organizador / Productora (Matías, 42)** | Administrador del evento | Gestionar aforos (+200 personas), recibir liquidaciones instantáneas y validar accesos sin caídas de red ni duplicados. |
| **Artista / Creador (Sofía, 31)** | Artista del evento | Monetizar arte digital conmemorativo del concierto/obra sin barreras y decidir libremente los royalties. |
| **Personal de Puerta / Validador (Lucas, 24)** | Staff de acceso | Escanear entradas QR a alta velocidad (menos de 1 segundo por persona) con prevención automática de reuso. |

---

## 3. 🏗️ Requerimientos Funcionales y Arquitectura

### 3.1. Módulo Web2.5 (Auth & Embedded Wallet)
* **Social Login:** Autenticación vía Google, Apple o Magic Link (Email).
* **Key Management:** Generación determinista de par de llaves Stellar (ed25519) sin exponer frases semilla al usuario final.

### 3.2. Módulo de Ticketing & Pagos Multimoneda (Stellar / Soroban)
* **Emisión no transferible:** El contrato emite tickets bloqueados a la cuenta del comprador (`transfer_ticket` deshabilitado o restringido a casos de fuerza mayor con autorización del admin).
* **Pagos Flexibles:** Soporte nativo de pagos en XLM y enrutamiento automático mediante Stellar Decentralized Exchange (DEX / Path Payments) para aceptar cualquier token/fiat on-ramp.
* **Control de Capacidad:** Límite estricto de aforo programado en el smart contract (mínimo 200 cupos).

### 3.3. Módulo de Validación en Puerta (QR Check-in)
* **QR Dinámico:** Generación de payload firmado por la clave del usuario que incluye `ticket_id`, `event_id` y `timestamp`.
* **Smart Contract `check_in`:** Al escanear, el validador invoca `check_in(admin, ticket_id)`. Si `is_used == false`, se marca como `true` y emite evento en Stellar.

### 3.4. Módulo de Coleccionables / NFTs de Artistas
* **Minting de Arte Digital:** Emisión de NFTs en Stellar para conmemorar el evento.
* **Política de Royalties Flexible:** El artista configura si desea un royalty porcentual o únicamente el costo de gas de la red Stellar.

---

## 4. 🗂️ Épicas y Desglose de Features

### **Épica 1: Core de Smart Contracts en Soroban (`EPIC-SC`)**
* `FEAT-SC-1`: Contrato de Tickets no transferibles con control de aforo (+200).
* `FEAT-SC-2`: Función de Check-in y prevención de doble uso en puerta.
* `FEAT-SC-3`: Contrato de NFTs de artistas con royalties opcionales y liquidación de gas.

### **Épica 2: Integración Web2.5 y Pagos (`EPIC-PAY`)**
* `FEAT-PAY-1`: Servicio de Embedded Wallets para login social.
* `FEAT-PAY-2`: Módulo de pagos multidivisa (XLM, tokens y Path Payments).

### **Épica 3: Sistema de Acceso y App de Validación (`EPIC-VAL`)**
* `FEAT-VAL-1`: Generador de códigos QR dinámicos en la app del usuario.
* `FEAT-VAL-2`: Interfaz de escaneo para personal de puerta conectada a Stellar.

---

## 5. 🎯 OKRs y Métricas de Éxito

* **Objetivo:** Lograr el despliegue del MVP de CulturaGo con cero fricción de onboarding y 100% de entradas verificadas.
  * **KR 1:** 100% de los tests unitarios y de estrés de Soroban pasando exitosamente para aforos > 200 personas.
  * **KR 2:** Tiempo de validación de QR en puerta inferior a 1.2 segundos por persona.
  * **KR 3:** 0 casos de reventa o falsificación en los eventos piloto.
