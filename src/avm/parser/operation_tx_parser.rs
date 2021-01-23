
use tracing::{instrument, trace};

use std::borrow::Borrow;
use std::error::Error;

use crate::avm::parser::{Context, OperationTx};
use crate::avm::parser::base_tx_parser::base_tx_parser;
use crate::avm::parser::transfer_op_parser::transfer_op_parser;
use crate::utils::conversion::pop_u32;

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn operation_tx_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<OperationTx, Box<dyn Error>> {
    let base = base_tx_parser(_raw_msg, _context)?;

    let transfer_op_number =
        pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow()) as usize;
    *_context.offset += 4;
    trace!(
        "Ipc: {} -- TxID: {} \n Operation -- Transfer Operation number {}",
        _context.ipc,
        _context.tx_id,
        transfer_op_number
    );

    let mut index = 0;
    let mut transfer_op = Vec::new();
    while index < transfer_op_number {
        trace!(
            "Operation -- initial state number {} -- bytes {:?}",
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        transfer_op.push(transfer_op_parser(_raw_msg, _context)?);
        index += 1;
    }

    Ok(OperationTx {
        base_tx: base,
        transferable_ops: transfer_op,
    })
}
