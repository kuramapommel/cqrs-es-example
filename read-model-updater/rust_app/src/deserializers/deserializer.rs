use anyhow::Error;

use crate::Request;

pub struct Deserializer;

impl Deserializer {
    pub fn deserialize(event_data: Request) -> Result<(String, String), Error> {
        Ok(event_data
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
            .unwrap_or_else(|| panic!("event_payload is None")))
    }
}

#[cfg(test)]
mod tests {
    use crate::{AttributeValue, DynamoDBStreamRecord, Request};

    use super::*;
    use std::collections::HashMap;

    #[allow(non_snake_case)]
    #[test]
    fn DynamoDBStreams_から流れてきたデータを_payload_と_manifest_にデシリアライズする() {
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

        let (payload, manifest) =
            Deserializer::deserialize(event_data).expect("Failed to deserialize");
        assert_eq!(
            payload,
            r#"{
                            "reservationId":"test-reservation-id",
                            "userId":"test-user-id",
                            "tableId":"test-table-id"
                        }"#
            .to_string()
        );
        assert_eq!(
            manifest,
            "com.kuramapommel.cqrs_es_example.domain.reservation.package$Event$Confirmed"
                .to_string()
        )
    }
}
