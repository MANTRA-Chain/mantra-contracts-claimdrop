// @ts-check
const { SigningCosmWasmClient, CosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { LedgerSigner } = require('@cosmjs/ledger-amino');
const { makeCosmoshubPath } = require("@cosmjs/amino");
const TransportNodeHid = require('@ledgerhq/hw-transport-node-hid').default;
const { GasPrice, calculateFee } = require('@cosmjs/stargate');
const readline = require('readline');

const GAS_PRICE_STRING = "0.025uom";
const LEDGER_ACCOUNT_PREFIX = "mantra";

async function main() {
    let transport;

    try {
        const rpcEndpoint = process.argv[2];
        const codeIdString = process.argv[3];
        const accountIndex = parseInt(process.argv[4] || "0", 10);

        if (!rpcEndpoint || !codeIdString) {
            console.error("Usage: node scripts/emergency/close_claimdrop_campaign.js <rpc_endpoint> <code_id> [account_index]");
            console.error("Example: node scripts/emergency/close_claimdrop_campaign.js \"http://localhost:26657\" 123 0");
            process.exit(1);
        }

        const codeId = parseInt(codeIdString, 10);
        if (isNaN(codeId)) {
            console.error("Error: <code_id> must be an integer.");
            console.error("Usage: node scripts/emergency/close_claimdrop_campaign.js <rpc_endpoint> <code_id> [account_index]");
            process.exit(1);
        }

        console.log("--- Emergency Claimdrop Campaign Closure Script ---");
        console.log("Configuration:");
        console.log(`  RPC Endpoint:           ${rpcEndpoint}`);
        console.log(`  Target Code ID:         ${codeId}`);
        console.log(`  Ledger Account Index:   ${accountIndex} (HD Path: m/44'/118'/0'/0/${accountIndex})`);
        console.log(`  Ledger Account Prefix:  ${LEDGER_ACCOUNT_PREFIX}`);
        console.log(`  Gas Price:              ${GAS_PRICE_STRING}`);
        console.log("----------------------------------------------------");
        console.log("IMPORTANT: This action is IRREVERSIBLE for each campaign.");
        console.log("Ensure you are targeting the correct Code ID, RPC endpoint, and Ledger account index.");
        console.log("You will need to confirm transactions on your Ledger device.");
        console.log("----------------------------------------------------");

        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });

        await new Promise(resolve => {
            rl.question("Type 'yes' to confirm and proceed: ", (answer) => {
                if (answer.toLowerCase() !== 'yes') {
                    console.log("Operation cancelled by user.");
                    rl.close();
                    process.exit(0);
                }
                rl.close();
                resolve();
            });
        });

        console.log("\nAttempting to connect to Ledger device...");
        transport = await TransportNodeHid.create();

        const hdPath = makeCosmoshubPath(accountIndex);
        const ledgerSigner = new LedgerSigner(transport, {
            hdPaths: [hdPath],
            prefix: LEDGER_ACCOUNT_PREFIX,
        });

        let accounts;
        accounts = await ledgerSigner.getAccounts();

        if (accounts.length === 0) {
            console.error(`No accounts found on Ledger for HD Path ${hdPath.toString()} with prefix '${LEDGER_ACCOUNT_PREFIX}'.`);
            process.exitCode = 1;
            return;
        }
        const senderAddress = accounts[0].address;
        console.log(`Successfully connected to Ledger. Using address: ${senderAddress}`);

        const client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, ledgerSigner, {
            gasPrice: GasPrice.fromString(GAS_PRICE_STRING)
        });
        console.log(`Connected to RPC endpoint: ${rpcEndpoint}`);

        console.log(
            `Querying contracts for code ID ${codeId} on chain via ${rpcEndpoint}...`
        );

        let contractAddresses;
        contractAddresses = await client.getContracts(codeId);

        if (!contractAddresses || contractAddresses.length === 0) {
            console.log(`No contracts found for code ID ${codeId}. Exiting.`);
            process.exitCode = 0;
            return;
        }

        console.log(
            `Found ${contractAddresses.length} contract(s) for code ID ${codeId}:`
        );
        contractAddresses.forEach((addr) => console.log(`  - ${addr}`));

        const closeCampaignMsg = {
            manage_campaign: {
                action: {
                    close_campaign: {},
                },
            },
        };

        const fee = calculateFee(300000, GasPrice.fromString(GAS_PRICE_STRING));

        console.log("\nProceeding to close campaigns for each contract...");

        for (const contractAddress of contractAddresses) {
            console.log(`\nAttempting to close campaign for contract: ${contractAddress}`);
            console.log("Please review and confirm the transaction on your Ledger device.");
            try {
                const result = await client.execute(
                    senderAddress,
                    contractAddress,
                    closeCampaignMsg,
                    fee,
                    `Emergency close campaign for ${contractAddress} (Code ID: ${codeId})`
                );
                console.log(
                    `  Successfully closed campaign. Transaction hash: ${result.transactionHash}`
                );
            } catch (error) {
                console.error(
                    `  Failed to close campaign for ${contractAddress}:`,
                    error.message || error
                );
            }
        }

        console.log("\nAll contract processing complete.");
        process.exitCode = 0;

    } catch (error) {
        console.error("\nAn unexpected error occurred within the main operation:", error.message);
        if (error.stack) console.error(error.stack);
        process.exitCode = 1;
    } finally {
        if (transport) {
            try {
                await transport.close();
                console.log("Ledger transport closed.");
            } catch (closeError) {
                console.error("Error closing Ledger transport:", closeError.message);
            }
        }
        if (process.exitCode !== undefined && process.exitCode !== null) {
            process.exit(process.exitCode);
        } else {
            process.exit(0);
        }
    }
}

main().catch(error => {
    console.error("Critical script failure (outer promise catch):", error.message);
    if (error.stack) console.error(error.stack);
    process.exit(1);
});