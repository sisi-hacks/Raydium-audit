# Reorganize the repo:
/contracts     # Original Raydium code
/audit
  ├── findings.md   # Structured report
  ├── poc/         # Proof-of-Concept exploits
  └── tools.md     # Slither/Foundry configs used
  # 🔎 Raydium Protocol Security Review  
**Scope:** [List contracts audited, e.g., `Staking.sol`]  
**Duration:** 2 weeks (June 2024)  
**Tools:** Slither, Foundry, Manual Review  

## 🚨 Critical Findings  
1. **High: Missing Access Control in `withdraw()`**  
   - **Impact:** Any user can drain funds.  
   - **Fix:** Add `onlyOwner` modifier.  

## 🔗 Links  
- [Full Report](audit/findings.md)  
- [Exploit PoC](audit/poc/reentrancy.sol)
- ## 🕵️ Finding 1: Reentrancy in `claimRewards()`  
**File:** `contracts/Staking.sol#L120`  
**Severity:** HIGH  
**Description:**  
The function updates state after external calls, allowing recursive withdrawals.  

**Proof of Concept:**  
```solidity
contract Exploit {
    function attack() public {
        staking.claimRewards(); // Recursive call
    }
}
