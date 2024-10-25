const fs = require('fs');
const Papa = require('papaparse');
const sha256 = require('crypto-js/sha256');
const {MerkleTree} = require('merkletreejs');

class ClaimdropCampaign {
    constructor(items) {
        const leaves = items.map(i => sha256(i.contract_addr + i.address + i.amount));
        this.tree = new MerkleTree(leaves, sha256, {sort: true});
    }

    getMerkleProof(item) {
        return this.tree.getHexProof(sha256(item.contract_addr + item.address + item.amount).toString())
            .map(v => v.replace('0x', ''));
    }
}

// input the data you want to make the merkle proof for

// csv file, formatted (with header) as contract_addr, address, amount
// see merkle_data.csv for an example
const file = 'merkle_data.csv';

// the contract address of the claimdrop contract hosting the airdrop campaign
const contractAddress = 'mantra1j28m8g0afvfr23423k5wypfykqrxsu94xhxvxdeyrfc4jkqm7zhqckdf5w';

// the address of the recipient you want to make the proof for
const address = 'mantra1x5nk33zpglp4ge6q9a8xx3zceqf4g8nvaggjmc';

// the amount of the airdrop according to the file
const amount = '100';


let receivers;
const csvData = fs.readFileSync(file, 'utf-8');
receivers = Papa.parse(csvData, {
    header: true,
    dynamicTyping: function (column) {
        return column !== 'amount';
        // Convert other columns to numbers if possible
    },
    skipEmptyLines: true,
}).data;

const claimdrop = new ClaimdropCampaign(receivers);
const proof = claimdrop.getMerkleProof({contract_addr: contractAddress, address, amount});
console.log('Merkle Proof:', proof);