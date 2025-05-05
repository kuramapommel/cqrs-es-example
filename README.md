# CQRS + ES サンプルプロジェクト

## 環境構築

```sh
lefthook install
```

## 動作確認

```sh
curl -X POST http://localhost:8080/api/reservation \
  --cookie "userId=test-user-id" \
  -H "Content-Type: application/json" \
  -d '{"tableId": "test-table-id"}'
```

```sh
curl http://localhost:3080/reservations
```
