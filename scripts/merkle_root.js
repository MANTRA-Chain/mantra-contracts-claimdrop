const fs = require('fs');
const Papa = require('papaparse');
const sha256 = require('crypto-js/sha256');
const {MerkleTree} = require('merkletreejs');

class AirdropCampaign {
    constructor(items) {
        const leaves = items.map(i => sha256(i.contract_addr + i.address + i.amount));
        this.tree = new MerkleTree(leaves, sha256, {sort: true});
    }

    getMerkleRoot() {
        return this.tree.getHexRoot().replace('0x', '');
    }
}

// input the file you want to make the merkle root for

// csv file, formatted (with header) as contract_addr, address, amount
// see merkle_data.csv for an example
const file = 'merkle_data.csv';

let receivers;
const csvData = fs.readFileSync(file, 'utf-8');
receivers = Papa.parse(csvData, {
    header: true,
    dynamicTyping: function (column) {
        return column !== 'amount';
    },
    skipEmptyLines: true,
}).data;

const airdrop = new AirdropCampaign(receivers);
const merkleRoot = airdrop.getMerkleRoot();
console.log('Merkle Root:', merkleRoot);