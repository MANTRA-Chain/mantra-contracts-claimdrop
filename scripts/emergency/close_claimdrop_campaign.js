// @ts-check
const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
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
        console.log("IMPORTANT: This script will attempt to close campaigns ONLY on contracts");
        console.log("           where the connected Ledger account is the current owner.");
        console.log("           This action is IRREVERSIBLE for each campaign closed.");
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
        try {
            transport = await TransportNodeHid.create();
        } catch (e) {
            console.error("Error connecting to Ledger device. Ensure it's connected, unlocked, and the MANTRA app is open.");
            console.error("Details:", e.message);
            process.exit(1);
        }

        const hdPath = makeCosmoshubPath(accountIndex);
        const ledgerSigner = new LedgerSigner(transport, {
            hdPaths: [hdPath],
            prefix: LEDGER_ACCOUNT_PREFIX,
        });

        let accounts;
        try {
            accounts = await ledgerSigner.getAccounts();
        } catch (e) {
            console.error("Error getting accounts from Ledger.");
            console.error("Details:", e.message);
            process.exit(1);
        }

        if (accounts.length === 0) {
            console.error(`No accounts found on Ledger for HD Path ${hdPath.toString()} with prefix '${LEDGER_ACCOUNT_PREFIX}'.`);
            process.exit(1);
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
        try {
            contractAddresses = await client.getContracts(codeId);
        } catch (err) {
            console.error(`Failed to query contracts for code ID ${codeId}:`, err);
            process.exit(1);
        }

        if (!contractAddresses || contractAddresses.length === 0) {
            console.log(`No contracts found for code ID ${codeId}. Exiting.`);
            process.exit(0);
        }

        console.log(
            `Found ${contractAddresses.length} contract(s) for code ID ${codeId}.`
        );

        const closeCampaignMsg = {
            manage_campaign: {
                action: {
                    close_campaign: {},
                },
            },
        };

        const fee = calculateFee(300000, GasPrice.fromString(GAS_PRICE_STRING));

        console.log("\nProcessing contracts to potentially close campaigns...");
        let successCount = 0;
        let failureCount = 0;
        let skippedNotOwnerCount = 0;
        let skippedNoCampaignCount = 0;

        for (const contractAddress of contractAddresses) {
            console.log(`\nInspecting contract: ${contractAddress}`);
            let campaignExists = false;
            try {
                const campaignQuery = { campaign: {} };
                try {
                    await client.queryContractSmart(contractAddress, campaignQuery);
                    campaignExists = true;
                    console.log(`  Query for active campaign successful.`);
                } catch (campaignQueryError) {
                    console.log(`No active campaign found.`);
                    skippedNoCampaignCount++;
                    continue;
                }

                if (campaignExists) {
                    const ownershipQuery = { ownership: {} };
                    const ownershipResponse = await client.queryContractSmart(contractAddress, ownershipQuery);

                    if (ownershipResponse && ownershipResponse.owner) {
                        const contractOwner = ownershipResponse.owner;
                        console.log(`  Contract owner: ${contractOwner}`);

                        if (contractOwner === senderAddress) {
                            console.log(`  Ledger address (${senderAddress}) IS the owner. Proceeding to close campaign.`);
                            console.log("  Please review and confirm the transaction on your Ledger device.");
                            try {
                                const result = await client.execute(
                                    senderAddress,
                                    contractAddress,
                                    closeCampaignMsg,
                                    fee,
                                    `Emergency close campaign for ${contractAddress} (Code ID: ${codeId})`
                                );
                                console.log(
                                    `    Successfully closed campaign. Transaction hash: ${result.transactionHash}`
                                );
                                successCount++;
                            } catch (executeError) {
                                console.error(
                                    `    Failed to close campaign for ${contractAddress} (owned by sender):`,
                                    executeError.message || executeError
                                );
                                failureCount++;
                            }
                        } else {
                            console.log(`  Ledger address (${senderAddress}) is NOT the owner. Skipping.`);
                            skippedNotOwnerCount++;
                        }
                    } else {
                        console.warn(`  Could not determine owner for contract ${contractAddress}. Response:`, ownershipResponse);
                        skippedNotOwnerCount++;
                    }
                }
            } catch (processContractError) {
                console.error(`  Failed to process contract ${contractAddress} (e.g., ownership query failed after campaign check):`, processContractError.message || processContractError);
                failureCount++;
            }
        }

        console.log("\n--- Processing Summary ---");
        console.log(`Successfully closed campaigns for ${successCount} contract(s).`);
        console.log(`Failed to close campaigns for ${failureCount} contract(s) (due to execution or query errors).`);
        console.log(`Skipped ${skippedNotOwnerCount} contract(s) (sender not owner or owner undetermined).`);
        console.log(`Skipped ${skippedNoCampaignCount} contract(s) (no active/queryable campaign found).`);
        console.log("All contract processing complete.");

        if (failureCount > 0 || (successCount === 0 && (skippedNotOwnerCount + skippedNoCampaignCount === contractAddresses.length) && contractAddresses.length > 0) ) {
            process.exitCode = 1;
        } else {
            process.exitCode = 0;
        }

    } catch (error) {
        console.error("\nAn critical unexpected error occurred:", error.message);
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
        process.exit(process.exitCode === undefined ? 1 : process.exitCode);
    }
}

main();