#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_successful_ticket_purchase_and_checkin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TicketContract, ());
    let client = TicketContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let event_name = String::from_str(&env, "Festival de Teatro Santiago a Mil");

    // 1. Inicializar evento
    let price = 50_000_000; // 5 XLM en stroops
    let capacity = 100;
    client.initialize(&admin, &event_name, &price, &capacity);

    // 2. Comprar ticket
    let ticket_id = client.buy_ticket(&buyer);
    assert_eq!(ticket_id, 1);

    // 3. Consultar ticket
    let ticket = client.get_ticket(&ticket_id);
    assert_eq!(ticket.owner, buyer);
    assert_eq!(ticket.price_paid, price);
    assert_eq!(ticket.is_used, false);

    // 4. Realizar Check-in en la puerta
    let checkin_result = client.check_in(&admin, &ticket_id);
    assert_eq!(checkin_result, true);

    // 5. Verificar que el ticket ahora está marcado como usado
    let updated_ticket = client.get_ticket(&ticket_id);
    assert_eq!(updated_ticket.is_used, true);
}

#[test]
fn test_sold_out_rejection() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TicketContract, ());
    let client = TicketContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer1 = Address::generate(&env);
    let buyer2 = Address::generate(&env);
    let buyer3 = Address::generate(&env);
    let event_name = String::from_str(&env, "Concierto Acustico Intimo");

    // Aforo pequeño de solo 2 entradas
    client.initialize(&admin, &event_name, &100_000_000, &2);

    assert_eq!(client.buy_ticket(&buyer1), 1);
    assert_eq!(client.buy_ticket(&buyer2), 2);

    // La tercera compra debe fallar con error SoldOut (código 3)
    let err = client.try_buy_ticket(&buyer3).unwrap_err();
    assert_eq!(err, Ok(Error::SoldOut));
}

#[test]
fn test_prevent_double_checkin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TicketContract, ());
    let client = TicketContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let event_name = String::from_str(&env, "Exposicion de Arte Digital");

    client.initialize(&admin, &event_name, &20_000_000, &50);
    let ticket_id = client.buy_ticket(&buyer);

    // Primer check-in: exitoso
    assert_eq!(client.check_in(&admin, &ticket_id), true);

    // Segundo intento de check-in: debe fallar con AlreadyUsed (código 5)
    let err = client.try_check_in(&admin, &ticket_id).unwrap_err();
    assert_eq!(err, Ok(Error::AlreadyUsed));
}
