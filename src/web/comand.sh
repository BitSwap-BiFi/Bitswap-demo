$ curl --request POST \
  --url http://localhost:8000/swap \
  --header 'Content-Type: application/json' \
  --data '{
    "from_token": "BTC",
    "to_token": "USDT",
    "amount": 0.1
  }'
