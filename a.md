# Account模型

skywelld中，账户生成代码。  
```c++
Account address = mTxn.getContract();
SkywellAddress dst= SkywellAddress::createAccountID(address);

Account const uDstAccountID (mTxn.getFieldAccount160 (sfDestination));
auto const index = getAccountRootIndex (uDstAccountID);
SLE::pointer sleDst (mEngine->view().entryCache (ltACCOUNT_ROOT, index));

// Create the account.
auto const newIndex = getAccountRootIndex (uDstAccountID);
sleDst = mEngine->view().entryCreate (ltACCOUNT_ROOT, newIndex);
sleDst->setFieldAccount (sfAccount, uDstAccountID);
sleDst->setFieldU32 (sfSequence, 1);

```

## 相关数据结构  

Account数据结构 __UinitTypes.h__  
```c++
/** Account is a hash representing a specific account. */
typedef base_uint<160, detail::AccountTag> Account;

```

--

SkywellAddress数据结构 __SkywellAddress.h__
```c++
class SkywellAddress
```

--

sle 数据结构 __STLedgerEntry.h__
```c++
class STLedgerEntry
using SLE = STLedgerEntry;
```

--

TransactionEngine数据结构 __TransactionEngine.h__
```c++

class TransactionEngine
```

--

## 相关方法  

```c++
static SkywellAddress createAccountID (Account const& uiAccountID);
uint256 getAccountRootIndex (Account const& account)
SLE::pointer LedgerEntrySet::entryCreate (LedgerEntryType letType, uint256 const& index)
```

