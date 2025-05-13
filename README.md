# Claimdrop Contract V2

The Claimdrop Contract V2 is a smart contract designed to distribute tokens to a list of addresses in a secure and 
efficient manner. The contract uses a **batch upload mechanism** to add a list of allocations, with addresses and their 
corresponding token amounts.

## Features

- Batch upload allocations. Batch upload can only be done before the campaign starts, afterwards, the feature is disabled.
- Lump sum and/or linear vesting distribution. Two distribution types are supported simultaneously. For instance one 
could be a lump sum distribution and the other could be a linear vesting distribution.
- Only one campaign per contract. If there's an error with the current campaign, the owner can close the campaign, 
retrieving all the unclaimed tokens back. It's possible to get a snapshot of all the tokens claimed up to that point 
with the Claimed query, then create a new contract/campaign with the right data.
- The owner is the only one who can create campaigns
- Anyone can top up the campaign by sending funds to the contract by using a BankMsg.
- The owner can close the campaign at any point before the campaign ends. 
When a campaign is ended, the owner will receive the remaining, unclaimed tokens in the campaign.
- Only a single claim entry per address is allowed.
- Addresses are added as strings, in case there are users entitled to claim but still haven't bridged from Ethereum to 
MANTRA, a placeholder can be used for those addresses.
- Ability to replace an address in the allocation's registry. When this occurs, the claims performed by the "old" wallet
are attached to the new address, same as the original allocation entry. The entries for the old wallet are removed.
- Coin agnostic, any native coin is supported.
- Ability to blacklist addresses (in case of hacked for instance). Blacklisted wallets cannot claim.
- The owner (of the contract) is the only one able to do all permissioned actions, i.e. create a campaign, close a 
campaign, blacklist users, batch upload addresses.

## When can it be used?

**Scenario 1:** Gendrop distribution of tokens via a linear (eg. 1 year) vesting airdrop to early investors.

**Scenario 2:** Post Gendrop rewarding active liquidity providers with quarterly token allocations over 1 year based on their pool shares.

## Resources

1. [Website](https://mantra.zone/)
2. [Docs](https://docs.mantrachain.io/mantra-smart-contracts/claimdrop_contract_v2)
