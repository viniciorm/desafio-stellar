/**
 * 🎟️ CulturaGo Stellar Ticket - Simulador de Flujo en Stellar / Soroban
 * Demostración del ciclo completo: Inicialización -> Compra con XLM -> Validación Check-in -> Prevención de Doble Uso
 */

console.log("=========================================================");
console.log("🎭 CULTURAGO STELLAR TICKET - SIMULACIÓN EN TIEMPO REAL");
console.log("=========================================================\n");

function delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function simulate() {
    console.log("1️⃣ [ADMIN] Configurando evento en el Smart Contract...");
    const eventConfig = {
        name: "Festival Internacional de Cine y Cultura 2026",
        admin: "GADM...CULTURAGO_ORGANIZER",
        priceInXLM: 15.0,
        totalCapacity: 3,
        ticketsSold: 0
    };
    console.log(`   📌 Evento: ${eventConfig.name}`);
    console.log(`   💰 Precio: ${eventConfig.priceInXLM} XLM por entrada`);
    console.log(`   🎟️ Aforo Total: ${eventConfig.totalCapacity} personas\n`);
    await delay(600);

    const ticketsDb = [];

    function buyTicket(buyerAddress) {
        if (eventConfig.ticketsSold >= eventConfig.totalCapacity) {
            console.log(`   ❌ ERROR: ¡Sold Out! No quedan entradas disponibles para ${buyerAddress}`);
            return null;
        }

        eventConfig.ticketsSold += 1;
        const ticketId = eventConfig.ticketsSold;
        const ticket = {
            id: ticketId,
            owner: buyerAddress,
            eventName: eventConfig.name,
            pricePaid: `${eventConfig.priceInXLM} XLM`,
            isUsed: false,
            issuedAt: new Date().toISOString()
        };
        ticketsDb.push(ticket);

        console.log(`   ✅ [STEL-TX] Compra confirmada para ${buyerAddress}`);
        console.log(`      -> Ticket ID: #${ticket.id} | Hash: 0x${Math.random().toString(16).substring(2, 10)}...`);
        return ticket;
    }

    console.log("2️⃣ [VENTAS] Simulación de compras concurrentes por usuarios:");
    const t1 = buyTicket("GUSER_ALICE...991");
    const t2 = buyTicket("GUSER_BOB...842");
    const t3 = buyTicket("GUSER_CARLOS...331");
    // Intento de compra con aforo superado:
    const t4 = buyTicket("GUSER_DANIELA...119");
    console.log(`\n   📊 Estado del Aforo: ${eventConfig.ticketsSold}/${eventConfig.totalCapacity} vendidas.\n`);
    await delay(800);

    console.log("3️⃣ [EVENTO] Control de Acceso y Check-in en Puerta con Smart Contract:");
    
    function checkIn(ticketId, adminValidator) {
        const ticket = ticketsDb.find(t => t.id === ticketId);
        if (!ticket) {
            console.log(`   ❌ Ticket #${ticketId} no encontrado.`);
            return;
        }

        if (ticket.isUsed) {
            console.log(`   ⛔ ALERTA DE FRAUDE: El Ticket #${ticketId} YA FUE UTILIZADO previamente. Entrada denegada a ${ticket.owner}`);
            return;
        }

        ticket.isUsed = true;
        console.log(`   🎫 ACCESO CONCEDIDO: Ticket #${ticket.id} válido para ${ticket.owner}. Marcado como utilizado.`);
    }

    // Alice ingresa al evento
    checkIn(1, eventConfig.admin);
    // Bob ingresa al evento
    checkIn(2, eventConfig.admin);
    // Intento de reutilizar o duplicar el ticket de Alice
    checkIn(1, eventConfig.admin);

    console.log("\n=========================================================");
    console.log("🎉 SIMULACIÓN COMPLETADA EXITOSAMENTE");
    console.log("   Todas las reglas del Smart Contract de CulturaGo fueron validadas.");
    console.log("=========================================================");
}

simulate();
