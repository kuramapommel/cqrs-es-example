#!/bin/sh

aws dynamodb delete-table --table-name event_journal
aws dynamodb delete-table --table-name snapshot
aws dynamodb delete-table --table-name timestamp_offset

aws dynamodb delete-table --table-name projection_spec