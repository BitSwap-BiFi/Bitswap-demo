OP_IF
    // Check that both BTC and USDT inputs are present
    #btc_input && #usdt_input &&
    // Check that the BTC input amount is greater than 0
    #btc_input.amount > 0 &&
    // Check that the USDT input amount is greater than 0
    #usdt_input.amount > 0 &&
    // Check that the BTC and USDT inputs have the same issuer
    #btc_input.issuer == #usdt_input.issuer &&
    // Check that the BTC and USDT inputs have different owners
    #btc_input.owner != #usdt_input.owner
OP_THEN
    // Calculate the exchange rate
    #rate = #usdt_input.amount / #btc_input.amount
    // Create BTC and USDT outputs with swapped amounts
    #btc_output = { "asset": "BTC", "amount": #usdt_input.amount, "issuer": #btc_input.issuer, "owner": #btc_input.owner }
    #usdt_output = { "asset": "USDT", "amount": #btc_input.amount, "issuer": #usdt_input.issuer, "owner": #usdt_input.owner }
    // Return the outputs and the exchange rate
    { "btc_output": #btc_output, "usdt_output": #usdt_output, "rate": #rate }
OP_ENDIF
