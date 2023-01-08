use crate::databases::database::IdempotencyTransaction;
use crate::open_idempotency::{IdempotencyData, IdempotencyId, IdempotencyStructure, MessageStatus};

pub fn convert_to_idempotency_status(id: String, app_id: String, transaction: IdempotencyTransaction) -> IdempotencyStructure{
    IdempotencyStructure{
        status: i32::from(transaction.status.map_to_grpc()),
        message: Some(IdempotencyData {
            id: Some(IdempotencyId {
                id,
                app_id
            }),
            data: transaction.response,
            custom_ttl: 0
        })
    }
}