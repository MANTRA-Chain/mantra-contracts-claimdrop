# Airdrop Manager Scripts

These scripts are meant to help you to generate the Merkle data you need when creating an airdrop campaign.

## How to use

First, configure the csv file, which is formatted as formatted (with header) as contract_addr, address, amount.

```json
contract_addr,address,amount
mantra1j28m8g0afvfr23423k5wypfykqrxsu94xhxvxdeyrfc4jkqm7zhqckdf5w,mantra1x5nk33zpglp4ge6q9a8xx3zceqf4g8nvaggjmc,100
mantra1j28m8g0afvfr23423k5wypfykqrxsu94xhxvxdeyrfc4jkqm7zhqckdf5w,mantra1rj2n3hge32n5u6zzw0u7clrys76srapulsvv39,200
mantra1j28m8g0afvfr23423k5wypfykqrxsu94xhxvxdeyrfc4jkqm7zhqckdf5w,mantra18mv5sz7nj2arpsqjc2aeslhh3v475np8ng6tt5,300
mantra1j28m8g0afvfr23423k5wypfykqrxsu94xhxvxdeyrfc4jkqm7zhqckdf5w,mantra16qtk5fnm4se6362yaah0scdmatx0qvp70fhej2,400
```

`node merkle_root.js` -> generates the merkle root
`node merkle_proof.js` -> generates the proofs. Consider tweaking the variables in the script to generate valid proofs 
according to the csv data.

Use the merkle root when creating an airdrop campaign, and the merkle proofs to claim the airdrop with a given address.
