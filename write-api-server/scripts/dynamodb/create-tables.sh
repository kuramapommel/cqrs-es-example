#!/usr/bin/env bash

function fail {
  echo "$@"
  exit 1
}

declare no_journal_index=false
declare no_snapshot_index=false

while [[ $# -gt 0 ]] ; do
  case "$1" in
    --no-journal-index  ) no_journal_index=true  ; shift ;;
    --no-snapshot-index ) no_snapshot_index=true ; shift ;;
    *                   ) fail "unknown option: $1"    ;
  esac
done

if $no_journal_index ; then
  #create-event-journal-table
  aws dynamodb create-table \
    --table-name event_journal \
    --endpoint-url "$DYNAMODB_ENDPOINT" \
    --attribute-definitions \
        AttributeName=pid,AttributeType=S \
        AttributeName=seq_nr,AttributeType=N \
    --key-schema \
        AttributeName=pid,KeyType=HASH \
        AttributeName=seq_nr,KeyType=RANGE \
    --provisioned-throughput \
        ReadCapacityUnits=5,WriteCapacityUnits=5 \
    --stream-specification \
        StreamEnabled=true,StreamViewType=NEW_IMAGE
  #create-event-journal-table
else
  #create-event-journal-table-with-slice-index
  aws dynamodb create-table \
    --table-name event_journal \
    --endpoint-url "$DYNAMODB_ENDPOINT" \
    --attribute-definitions \
        AttributeName=pid,AttributeType=S \
        AttributeName=seq_nr,AttributeType=N \
        AttributeName=entity_type_slice,AttributeType=S \
        AttributeName=ts,AttributeType=N \
    --key-schema \
        AttributeName=pid,KeyType=HASH \
        AttributeName=seq_nr,KeyType=RANGE \
    --provisioned-throughput \
        ReadCapacityUnits=5,WriteCapacityUnits=5 \
    --stream-specification \
        StreamEnabled=true,StreamViewType=NEW_IMAGE \
    --global-secondary-indexes \
      '[
         {
           "IndexName": "event_journal_slice_idx",
           "KeySchema": [
             {"AttributeName": "entity_type_slice", "KeyType": "HASH"},
             {"AttributeName": "ts", "KeyType": "RANGE"}
           ],
           "Projection": {
             "ProjectionType": "ALL"
           },
           "ProvisionedThroughput": {
             "ReadCapacityUnits": 5,
             "WriteCapacityUnits": 5
           }
        }
      ]'
  #create-event-journal-table-with-slice-index
fi

if $no_snapshot_index ; then
  #create-snapshot-table
  aws dynamodb create-table \
    --table-name snapshot \
    --endpoint-url "$DYNAMODB_ENDPOINT" \
    --attribute-definitions \
        AttributeName=pid,AttributeType=S \
    --key-schema \
        AttributeName=pid,KeyType=HASH \
    --provisioned-throughput \
        ReadCapacityUnits=5,WriteCapacityUnits=5
  #create-snapshot-table
else
  #create-snapshot-table-with-slice-index
  aws dynamodb create-table \
    --table-name snapshot \
    --endpoint-url "$DYNAMODB_ENDPOINT" \
    --attribute-definitions \
        AttributeName=pid,AttributeType=S \
        AttributeName=entity_type_slice,AttributeType=S \
        AttributeName=event_timestamp,AttributeType=N \
    --key-schema \
        AttributeName=pid,KeyType=HASH \
    --provisioned-throughput \
        ReadCapacityUnits=5,WriteCapacityUnits=5 \
    --global-secondary-indexes \
      '[
         {
           "IndexName": "snapshot_slice_idx",
           "KeySchema": [
             {"AttributeName": "entity_type_slice", "KeyType": "HASH"},
             {"AttributeName": "event_timestamp", "KeyType": "RANGE"}
           ],
           "Projection": {
             "ProjectionType": "ALL"
           },
           "ProvisionedThroughput": {
             "ReadCapacityUnits": 5,
             "WriteCapacityUnits": 5
           }
        }
      ]'
  #create-snapshot-table-with-slice-index
fi

aws dynamodb create-table \
  --table-name timestamp_offset \
  --endpoint-url "$DYNAMODB_ENDPOINT" \
  --attribute-definitions \
      AttributeName=name_slice,AttributeType=S \
      AttributeName=pid,AttributeType=S \
  --key-schema \
      AttributeName=name_slice,KeyType=HASH \
      AttributeName=pid,KeyType=RANGE \
  --provisioned-throughput \
      ReadCapacityUnits=5,WriteCapacityUnits=5

aws dynamodb create-table \
  --table-name projection_spec \
  --endpoint-url "$DYNAMODB_ENDPOINT" \
  --attribute-definitions \
      AttributeName=id,AttributeType=S \
  --key-schema \
      AttributeName=id,KeyType=HASH \
  --provisioned-throughput \
      ReadCapacityUnits=5,WriteCapacityUnits=5