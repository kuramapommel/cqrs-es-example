use std::{any::Any, sync::Arc};

use anyhow::Error;
use serde::Deserialize;

use crate::Request;

#[derive(Debug, Deserialize)]
struct ReservationConfirmedPayload {
    #[serde(rename = "reservationId")]
    reservation_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "tableId")]
    table_id: String,
}

#[derive(Debug, Deserialize)]
struct ReservationCancelledPayload {
    #[serde(rename = "reservationId")]
    reservation_id: String,
}

pub trait DomainEvent: Any {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservationEvent {
    ReservationConfirmed {
        reservation_id: String,
        user_id: String,
        table_id: String,
    },
    ReservationCancelled {
        reservation_id: String,
    },
}

impl DomainEvent for ReservationEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableManagementEvent {
    ReservationConfirmed {
        table_id: String,
        user_id: String,
        reservation_id: String,
    },
}

impl DomainEvent for TableManagementEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Deserializer;

impl Deserializer {
    pub fn deserialize(event_data: Request) -> Result<Arc<dyn DomainEvent>, Error> {
        let (payload, manifest) = event_data
            .dynamodb
            .new_image
            .as_ref()
            .and_then(|new_image| {
                new_image
                    .get("event_payload")
                    .zip(new_image.get("event_ser_manifest"))
                    .map(|(payload, manifest)| (payload.clone(), manifest.clone()))
            })
            .and_then(|event_data| {
                let (payload, manifest) = event_data;
                payload.b.zip(manifest.s)
            })
            .unwrap_or_else(|| panic!("event_payload is None"));

        let event: Arc<dyn DomainEvent> = match manifest.as_ref() {
            // 予約コンテキスト
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Confirmed" => {
                serde_json::from_str(&payload)
                    .map(
                        |ReservationConfirmedPayload {
                             reservation_id,
                             user_id,
                             table_id,
                         }| Arc::new(ReservationEvent::ReservationConfirmed {
                            reservation_id,
                            user_id,
                            table_id,
                        }),
                    )
                    .expect("Failed to deserialize event payload")
            }
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Cancelled" => {
                serde_json::from_str(&payload)
                    .map(
                        |ReservationCancelledPayload {
                             reservation_id
                         }| Arc::new(ReservationEvent::ReservationCancelled {
                            reservation_id,
                        }),
                    )
                    .expect("Failed to deserialize event payload")
            }

            // テーブル管理コンテキスト
            "com.kuramapommel.cqrs_es_example.domain.table_management.package$Event$ReservationConfirmed" => {
                serde_json::from_str(&payload)
                    .map(
                        |ReservationConfirmedPayload {
                             reservation_id,
                             user_id,
                             table_id,
                         }| Arc::new(TableManagementEvent::ReservationConfirmed {
                            table_id,
                            user_id,
                            reservation_id,
                        }),
                    )
                    .expect("Failed to deserialize event payload")
            }

            // 存在しないイベント
            _ => panic!("Unknown event type"),
        };

        Ok(event)
    }
}

#[cfg(test)]
mod test {
    use crate::{AttributeValue, DynamoDBStreamRecord, Request};

    use super::*;
    use std::collections::HashMap;

    #[allow(non_snake_case)]
    #[test]
    fn マニフェストが_reservation_package_Event_Confirmed_の場合_ReservationConfirmed_にデシリアライズされる(
    ) {
        let event_data = Request {
            dynamodb: DynamoDBStreamRecord {
                new_image: Some(HashMap::from([
                    ("event_payload".to_string(), AttributeValue {
                        s: None,
                        b: Some(r#"{
                            "reservationId":"test-reservation-id",
                            "userId":"test-user-id",
                            "tableId":"test-table-id"
                        }"#.to_string())
                    }),
                    ("event_ser_manifest".to_string(), AttributeValue {
                        s: Some("com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Confirmed".to_string()),
                        b: None
                    })
                ])),
            },
        };

        let expected = ReservationEvent::ReservationConfirmed {
            reservation_id: "test-reservation-id".to_string(),
            user_id: "test-user-id".to_string(),
            table_id: "test-table-id".to_string(),
        };

        let result = Deserializer::deserialize(event_data).expect("Failed to deserialize");
        let downcasted = result
            .as_any()
            .downcast_ref::<ReservationEvent>()
            .expect("Wrong event type");
        assert_eq!(downcasted, &expected);
    }
}
