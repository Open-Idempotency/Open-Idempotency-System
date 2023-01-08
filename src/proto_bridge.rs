use crate::databases::database::IdempotencyTransaction;
use crate::open_idempotency::{IdempotencyDataMessage, IdempotencyStatus, MessageStatus};

pub fn convert_to_idempotency_status(id: String, transaction: IdempotencyTransaction) -> IdempotencyStatus{
    IdempotencyStatus{
        status: i32::from(transaction.status.map_to_grpc()),
        message: Some(IdempotencyDataMessage {
            id,
            data: transaction.response
        })
    }
}