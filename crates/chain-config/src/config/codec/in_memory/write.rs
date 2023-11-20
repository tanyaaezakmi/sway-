use std::borrow::BorrowMut;

use crate::{
    config::{
        codec::GroupEncoder, contract_balance::ContractBalance,
        contract_state::ContractState,
    },
    CoinConfig, ContractConfig, MessageConfig, StateConfig,
};

pub struct Encoder<W> {
    sink: W,
}

impl<W> Encoder<W> {
    pub fn new(sink: W) -> Self {
        Self { sink }
    }

    pub fn into_inner(self) -> W {
        self.sink
    }
}

impl<W: BorrowMut<StateConfig>> GroupEncoder for Encoder<W> {
    fn write_coins(&mut self, elements: Vec<CoinConfig>) -> anyhow::Result<()> {
        self.sink.borrow_mut().coins.extend(elements);
        Ok(())
    }

    fn write_contracts(&mut self, elements: Vec<ContractConfig>) -> anyhow::Result<()> {
        self.sink.borrow_mut().contracts.extend(elements);
        Ok(())
    }

    fn write_messages(&mut self, elements: Vec<MessageConfig>) -> anyhow::Result<()> {
        self.sink.borrow_mut().messages.extend(elements);
        Ok(())
    }

    fn write_contract_state(
        &mut self,
        elements: Vec<ContractState>,
    ) -> anyhow::Result<()> {
        self.sink.borrow_mut().contract_state.extend(elements);
        Ok(())
    }

    fn write_contract_balance(
        &mut self,
        elements: Vec<ContractBalance>,
    ) -> anyhow::Result<()> {
        self.sink.borrow_mut().contract_balance.extend(elements);
        Ok(())
    }

    fn close(self: Box<Self>) -> anyhow::Result<()> {
        Ok(())
    }
}