use serde::Serialize;

#[derive(PartialEq, Clone)]
enum ClientStatus {
  Active,
  Frozen,
}

#[derive(Default, Clone)]
struct Wallet {
  available: f32,
  held: f32,
}

/// A `Client` representation
#[derive(Clone)]
pub struct Client {
  id: u16,
  wallet: Wallet,
  status: ClientStatus,
}

impl Client {
  /// Construct a `Client`.
  ///
  /// The client defaults to an empty active wallet.
  pub fn new(id: u16) -> Self {
    Self {
      id,
      status: ClientStatus::Active,
      wallet: Default::default(),
    }
  }

  /// Credits client account with `amount` value.
  ///
  /// This method only adds funds on active client.
  pub fn deposit_funds(&mut self, amount: f32) -> bool {
    if self.is_locked() {
      return false;
    }

    self.wallet.available += amount;

    true
  }

  /// Credits client account with `amount` value.
  ///
  /// This method only remove funds on active client and if client
  /// has enough funds.
  pub fn withdraw_funds(&mut self, amount: f32) -> bool {
    if self.is_locked() || !self.has_enough_available_funds(amount) {
      return false;
    }

    self.wallet.available -= amount;

    true
  }

  pub fn block_funds(&mut self, amount: f32) -> bool {
    if self.is_locked() || !self.has_enough_available_funds(amount) {
      return false;
    }

    self.wallet.available -= amount;
    self.wallet.held += amount;

    true
  }

  pub fn release_funds(&mut self, amount: f32) -> bool {
    if self.is_locked() || !self.has_enough_held_funds(amount) {
      return false;
    }

    self.wallet.available += amount;
    self.wallet.held -= amount;

    true
  }

  pub fn chargeback_funds(&mut self, amount: f32) -> bool {
    if self.is_locked() || !self.has_enough_held_funds(amount) {
      return false;
    }

    self.wallet.held -= amount;
    self.status = ClientStatus::Frozen;

    true
  }

  fn has_enough_available_funds(&self, amount: f32) -> bool {
    amount <= self.wallet.available
  }

  fn has_enough_held_funds(&self, amount: f32) -> bool {
    amount <= self.wallet.held
  }

  pub fn is_locked(&self) -> bool {
    matches!(self.status, ClientStatus::Frozen)
  }
}

#[derive(Debug, Serialize)]
pub(crate) struct ClientView {
  pub client: u16,
  pub available: f32,
  pub held: f32,
  pub total: f32,
  pub locked: bool,
}

impl From<Client> for ClientView {
  fn from(client: Client) -> Self {
    Self {
      client: client.id,
      available: client.wallet.available,
      held: client.wallet.held,
      total: client.wallet.available + client.wallet.held,
      locked: client.is_locked(),
    }
  }
}
