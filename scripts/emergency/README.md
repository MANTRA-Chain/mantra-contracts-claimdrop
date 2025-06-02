# Emergency Scripts

This directory contains scripts designed for emergency situations to manage and control aspects of MANTRA Chain contracts. These scripts are powerful and should be used with extreme caution by authorized personnel only.

**WARNING: EXECUTING THESE SCRIPTS CAN HAVE SIGNIFICANT AND POTENTIALLY IRREVERSIBLE CONSEQUENCES. ALWAYS DOUBLE-CHECK PARAMETERS AND FULLY UNDERSTAND THE SCRIPT'S ACTIONS BEFORE EXECUTION.**

## Table of Contents

1.  [Prerequisites](#prerequisites)
2.  [Available Scripts](#available-scripts)
    *   [Close Claimdrop Campaign](#close-claimdrop-campaign)
3.  [General Usage](#general-usage)
4.  [Safety Guidelines](#safety-guidelines)
5.  [Troubleshooting](#troubleshooting)

## 1. Prerequisites

### 1.1 Software & Environment
*   **Node.js**: Version 18.x or higher. Required to run the JavaScript files. Download from [nodejs.org](https://nodejs.org/).
*   **npm/yarn**: For installing Node.js dependencies.
*   **Ledger Hardware Wallet**: With the MANTRA app (or Cosmos app if MANTRA app not available/applicable) installed and updated.
*   **Operating System**: Linux, macOS, or Windows (Node.js is cross-platform).

### 1.2 Dependencies
It's recommended to create a `package.json` in this `scripts/emergency/` directory to manage dependencies for these scripts.
If you don't have one, you can create it by running `npm init -y` inside `scripts/emergency/`.

Then, install the required Node.js dependencies:
```bash
# Navigate to scripts/emergency if you are not already there
# cd scripts/emergency

npm install @cosmjs/cosmwasm-stargate @cosmjs/ledger-amino @cosmjs/amino @ledgerhq/hw-transport-node-hid @cosmjs/stargate readline secp256k1
```
Ensure `secp256k1` can be built, which might require build tools like `python`, `make`, `g++` depending on your OS. The `readline` module is used for the confirmation prompt.

### 1.3 Ledger Setup
*   Connect your Ledger device and unlock it.
*   Open the MANTRA app (or Cosmos app) on the Ledger.
*   Ensure the Ledger firmware and app are up to date.

## 2. Available Scripts

### 2.1 Close Claimdrop Campaign

**Script File:**
*   `close_claimdrop_campaign.js` (Node.js script)

**Purpose:**
This script attempts to close the current campaign for ALL claimdrop contracts instantiated from a specific `CODE_ID`. This is an emergency measure to halt claimdrop activities.

**Action:**
For each contract found associated with the given `CODE_ID`, it executes the `ManageCampaign` message with the `CloseCampaign` action. You will be prompted for confirmation in the terminal and must approve each transaction on your Ledger device.

**Execution:**
The script is configured using command-line arguments.

```bash
node scripts/emergency/close_claimdrop_campaign.js <rpc_endpoint> <code_id> [account_index]
```

**Arguments:**

*   `<rpc_endpoint>`: (Required) RPC endpoint of the target chain (e.g., `"http://localhost:26657"` or `"https://rpc.mantrachain.io"`).
*   `<code_id>`: (Required) The numeric Code ID of the claimdrop contracts to target (e.g., `123`).
*   `[account_index]`: (Optional) The account index for the Ledger HD path (`m/44'/118'/0'/0/X`). Defaults to `0` if not provided.

**Hardcoded Values in Script:**
*   `LEDGER_ACCOUNT_PREFIX`: "mantra"
*   `GAS_PRICE_STRING`: "0.025uom"

**Example Usage:**
```bash
# Using default Ledger account index 0
node scripts/emergency/close_claimdrop_campaign.js "http://localhost:26657" 1

# Using Ledger account index 5
node scripts/emergency/close_claimdrop_campaign.js "https://rpc.mantrachain.io" 456 5
```

**Execution Flow:**
1.  Run the Node.js script with the required command-line arguments.
2.  The script will display the configuration and a critical warning.
3.  You will be prompted for confirmation in the terminal. Type 'yes' to proceed.
4.  The script connects to the Ledger, queries contracts by `CODE_ID`.
5.  For each contract found, it prepares to broadcast a `ManageCampaign { action: { close_campaign: {} } }` message.
6.  You must approve each transaction on your Ledger device.
7.  Results (success or failure for each contract) are logged to the console.

## 3. General Usage

1.  **Navigate to the Project Root:** Open your terminal in the root directory of the `mantra-contracts-claimdrop` repository.
2.  **Install Dependencies:** If you haven't already, navigate to `scripts/emergency` and install the Node.js dependencies (see [Prerequisites](#12-dependencies)).
    ```bash
    cd scripts/emergency
    npm install
    # cd ../.. # Go back to project root if needed
    ```
3.  **Connect Ledger:** Ensure your Ledger device is connected, unlocked, and the MANTRA (or Cosmos) app is open.
4.  **Execute the Script:**
    ```bash
    node scripts/emergency/close_claimdrop_campaign.js "<your_rpc_url>" <target_code_id> [ledger_account_index]
    ```
    (Modify with actual values).
5.  **Follow Prompts:** The script will guide you through confirmation steps on the terminal and your Ledger device. Read all prompts carefully.
6.  **Monitor Output:** Check the console output for success or error messages. Verify actions on-chain using a block explorer if necessary.

## 4. Safety Guidelines

*   **AUTHORIZED PERSONNEL ONLY:** This script should only be run by individuals who fully understand the smart contracts and the implications of these emergency actions.
*   **TESTNET FIRST:** Always test this script thoroughly on a testnet environment that mirrors mainnet conditions as closely as possible before ever considering its use on mainnet.
*   **VERIFY PARAMETERS:** Triple-check all command-line arguments (`<rpc_endpoint>`, `<code_id>`, `[account_index]`) before execution.
*   **SECURE ENVIRONMENT:** Run this script from a secure, trusted machine.
*   **LEDGER CONFIRMATION:** Pay close attention to the details displayed on your Ledger device screen before approving any transaction.
*   **UNDERSTAND THE CODE:** Review the Node.js script code.
*   **COMMUNICATION:** If operating as part of a team, communicate clearly before, during, and after script execution.

## 5. Troubleshooting

*   **Ledger Connection Issues:**
    *   Ensure Ledger is unlocked, MANTRA/Cosmos app is open.
    *   Try reconnecting. Check for conflicting apps (e.g., Ledger Live).
    *   Update Ledger firmware and app.
*   **Node.js Script Errors:**
    *   `Usage: node scripts/emergency/...`: Missing or incorrect command-line arguments.
    *   `Error: <code_id> must be an integer.`: Ensure the code ID is a valid number.
    *   `cannot find module '@cosmjs/...'`: Run `npm install` in `scripts/emergency/`.
    *   Verify RPC endpoint is correct and accessible.
*   **Transaction Failures:**
    *   `Out of gas`: The default gas amount per transaction is `300000`. You might need to adjust this value in `calculateFee(300000, ...)` within the script if transactions fail due to insufficient gas. The `GAS_PRICE_STRING` is hardcoded to "0.025uom".
    *   `Account sequence mismatch`: Usually handled by CosmJS.
    *   `Signature verification failed` / `Denied by user?`: Ensure correct Ledger operation.
    *   Contract-specific errors: Check error message for details from the contract.
*   **General Troubleshooting:**
    *   Ensure the `account_index` corresponds to an initialized account on your Ledger with funds.
    *   Confirm the `CODE_ID` is correct for the target claimdrop contracts.
