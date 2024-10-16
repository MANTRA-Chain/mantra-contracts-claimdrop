# Airdrop Manager

To foster widespread token distribution, incentivize participation, and drive network adoption.

The Airdrop Manager contract can be used to distribute tokens to a list of addresses. The contract uses a Merkle tree to 
store the list of addresses and their corresponding token amounts. The root of the Merkle tree is stored on the contract's 
campaign and is used to verify the validity of the proofs submitted by the recipients.

## Features

- Merkle tree based distribution
- Lump sum and/or linear vesting distribution. Two distribution types are supported simultaneously. For instance one 
could be a lump sum distribution and the other could be a linear vesting distribution.
- Only one campaign can be active at a time. If there's an error with the current campaign, the owner can take a snapshot of
all the tokens claimed up to that point with the Claimed query, end the campaign and then create a new one with the right data.
- The owner of the contract is the only one who can create campaigns. A different address can be assigned as the owner
of the campaign upon creation.
- The owner of the campaign can top up the campaign with more tokens at any point before the campaign ends.
- The owner of the campaign or the owner of the contract can end the active campaign at any point before the campaign ends. 
When a campaign is ended, the owner of the campaign will receive the remaining, unclaimed tokens in the campaign.

## When can it be used?

**Scenario 1:** Gendrop distribution of tokens via a linear (eg. 1 year) vesting airdrop to early investors.

**Scenario 2:** Post Gendrop rewarding active liquidity providers with quarterly token allocations over 1 year based on their pool shares.
