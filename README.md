# Raydium-audit
The update_amm_config function in the Raydium AMM v3 smart contract fails to properly validate input parameters before updating critical configuration settings. This oversight creates a significant security risk as an attacker can exploit this vulnerability in a production environment by injecting malicious values like extreme fee rates.
