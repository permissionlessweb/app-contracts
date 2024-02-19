#!/bin/bash
for d in ../artifacts/nft_loans_nc.wasm; do 
echo $d;
st tx wasm i 3631 '{"fee_rate":"0.1","name":"draft-nft-loans-nc","treasury_addr":"stars1n5x097nd7v8dv8ng4x4xeux5xdv6jas62qslh3"}'   --from test1 --gas auto --admin stars122xnz0c6e529qnns8prf5997eup4waukvlhpdx --label "nft-loans-nc" --fees 500000ustars --gas-adjustment 3 -y
done 
sleep 6

for e in ../artifacts/raffles.wasm; do 
echo $e;
st tx wasm i 3630 '{"name":"atlas-app-raffles","nois_proxy_addr":"stars1atcndw8yfrulzux6vg6wtw2c0u4y5wvy9423255h472f4x3gn8dq0v8j45","nois_proxy_coin":{"amount":"500000", "denom":"ustars"}, "raffle_fee": "0.0"}'  --from test1 --gas auto --admin stars122xnz0c6e529qnns8prf5997eup4waukvlhpdx --label "raffles" --fees 500000ustars --gas-adjustment 3 -y
done