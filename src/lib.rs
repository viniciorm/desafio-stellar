#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    SoldOut = 3,
    TicketNotFound = 4,
    AlreadyUsed = 5,
    Unauthorized = 6,
    InvalidPriceOrCapacity = 7,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Config,
    Ticket(u32),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventConfig {
    pub admin: Address,
    pub event_name: String,
    pub ticket_price: i128,
    pub total_capacity: u32,
    pub tickets_sold: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    pub id: u32,
    pub owner: Address,
    pub price_paid: i128,
    pub is_used: bool,
    pub issue_timestamp: u64,
}

#[contract]
pub struct TicketContract;

#[contractimpl]
impl TicketContract {
    /// Inicializa el evento con su administrador, nombre, precio por entrada y aforo total.
    pub fn initialize(
        env: Env,
        admin: Address,
        event_name: String,
        ticket_price: i128,
        total_capacity: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Config) {
            return Err(Error::AlreadyInitialized);
        }

        if ticket_price < 0 || total_capacity == 0 {
            return Err(Error::InvalidPriceOrCapacity);
        }

        admin.require_auth();

        let config = EventConfig {
            admin,
            event_name,
            ticket_price,
            total_capacity,
            tickets_sold: 0,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        Ok(())
    }

    /// Permite a un usuario comprar un ticket para el evento cultural.
    pub fn buy_ticket(env: Env, buyer: Address) -> Result<u32, Error> {
        buyer.require_auth();

        let mut config: EventConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(Error::NotInitialized)?;

        if config.tickets_sold >= config.total_capacity {
            return Err(Error::SoldOut);
        }

        let new_ticket_id = config.tickets_sold + 1;
        config.tickets_sold = new_ticket_id;

        let ticket = Ticket {
            id: new_ticket_id,
            owner: buyer.clone(),
            price_paid: config.ticket_price,
            is_used: false,
            issue_timestamp: env.ledger().timestamp(),
        };

        // Guardar ticket y actualizar configuración
        env.storage()
            .persistent()
            .set(&DataKey::Ticket(new_ticket_id), &ticket);
        env.storage().instance().set(&DataKey::Config, &config);

        // Emitir evento en Stellar
        env.events().publish(
            (symbol_short!("ticket"), symbol_short!("minted")),
            (new_ticket_id, buyer),
        );

        Ok(new_ticket_id)
    }

    /// Valida y marca el ticket como utilizado al ingresar al evento (Check-in).
    pub fn check_in(env: Env, admin: Address, ticket_id: u32) -> Result<bool, Error> {
        let config: EventConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(Error::NotInitialized)?;

        // Solo el admin del evento o personal autorizado puede hacer check-in
        admin.require_auth();
        if admin != config.admin {
            return Err(Error::Unauthorized);
        }

        let key = DataKey::Ticket(ticket_id);
        let mut ticket: Ticket = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::TicketNotFound)?;

        if ticket.is_used {
            return Err(Error::AlreadyUsed);
        }

        // Marcar como usado para prevenir doble entrada
        ticket.is_used = true;
        env.storage().persistent().set(&key, &ticket);

        // Emitir evento de check-in
        env.events().publish(
            (symbol_short!("ticket"), symbol_short!("checked")),
            ticket_id,
        );

        Ok(true)
    }

    /// Transfiere un ticket a un nuevo propietario.
    pub fn transfer_ticket(
        env: Env,
        current_owner: Address,
        new_owner: Address,
        ticket_id: u32,
    ) -> Result<bool, Error> {
        current_owner.require_auth();

        let key = DataKey::Ticket(ticket_id);
        let mut ticket: Ticket = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::TicketNotFound)?;

        if ticket.owner != current_owner {
            return Err(Error::Unauthorized);
        }

        if ticket.is_used {
            return Err(Error::AlreadyUsed);
        }

        ticket.owner = new_owner.clone();
        env.storage().persistent().set(&key, &ticket);

        env.events().publish(
            (symbol_short!("ticket"), symbol_short!("xfer")),
            (ticket_id, new_owner),
        );

        Ok(true)
    }

    /// Obtiene los detalles de un ticket específico.
    pub fn get_ticket(env: Env, ticket_id: u32) -> Result<Ticket, Error> {
        let key = DataKey::Ticket(ticket_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(Error::TicketNotFound)
    }

    /// Obtiene los datos generales del evento y aforo actual.
    pub fn get_event_info(env: Env) -> Result<EventConfig, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(Error::NotInitialized)
    }
}

#[cfg(test)]
mod test;
