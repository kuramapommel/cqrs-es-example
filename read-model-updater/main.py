import boto3
import time
import json
import os
import requests
from datetime import date, datetime

# DynamoDB Streams configuration
table = 'event_journal'
dynamodb_endpoint = f"http://{os.getenv('DYNAMODB_HOSTNAME')}:8000"
dynamodb = boto3.client('dynamodb', endpoint_url=dynamodb_endpoint)
dynamodb_streams = boto3.client('dynamodbstreams', endpoint_url=dynamodb_endpoint)

latest_stream_arn = dynamodb.describe_table(TableName=table)['Table']['LatestStreamArn']
shards = dynamodb_streams.describe_stream(StreamArn=latest_stream_arn)['StreamDescription']['Shards']

shard_iterators = {}
for shard in shards:
    shard_id = shard['ShardId']
    shard_iterators[shard_id] = dynamodb_streams.get_shard_iterator(
        StreamArn=latest_stream_arn,
        ShardId=shard_id,
        ShardIteratorType='LATEST',
    )['ShardIterator']

# デフォルトのシリアライザを上書き
def json_serial(obj):
    if isinstance(obj, (datetime, date)):
        return obj.isoformat()
    if isinstance(obj, bytes):
        return obj.decode('utf-8')
    raise TypeError('Type %s not serializable' % type(obj))

# lambda_functionを呼び出す
def invoke_function(records):
    print(records)
    
    lambda_endpoint = f"http://{os.getenv('READ_MODEL_UPDATER_HOSTNAME')}:8080/2015-03-31/functions/function/invocations"
    for record in records:
        lamda_body = json.loads(json.dumps(record, default=json_serial))
        response = requests.post(lambda_endpoint, json=lamda_body)
        print(f"response code: {response.status_code}, response: {response.text}")

    print("completed.")

# DynamoDB Streams をポーリングして変更を検知
while True:
    if shard_iterators == []:
        break

    for shard_id, iterator in [*shard_iterators.items()]:
        if iterator is None:
            del shard_iterators[shard_id]
            continue

        get_records_result = dynamodb_streams.get_records(ShardIterator=iterator)
        next_shard_iterator = get_records_result.get('NextShardIterator')

        records = get_records_result['Records']

        while next_shard_iterator == iterator:
            get_records_result = dynamodb_streams.get_records(ShardIterator=next_shard_iterator)
            next_shard_iterator = get_records_result.get('NextShardIterator')
            records.append(get_records_result['Records'])

        shard_iterators[shard_id] = next_shard_iterator
        if records == []:
            continue

        invoke_function(records)

    time.sleep(1)