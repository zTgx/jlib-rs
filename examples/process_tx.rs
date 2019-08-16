extern crate jlib;

fn main() {
    let tx: String = r#"
            { "Account": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                "Fee": "10000",
                "Flags": 524288,
                "Sequence": 650,
                "SigningPubKey": "03E791056E6B4C62E26C0F1F3BB89317667AB74170B49339972716FC53FFCF007C",
                "TakerGets": "2000000000",
                "TakerPays":
                    { "currency": "CNY",
                      "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                      "value": "13.46" },
                "Timestamp": 611552863,
                "TransactionType": "OfferCreate",
                "TxnSignature": "3045022100B342C7159E1AD7FAA13452C1FB01F77A107263AFB06E93F9B4F307EF9DF9F98E0220690C964D89146250E879B9EA5ED311C3317631F1F6450360603E6818AFFB5FAF",
                "date": 611552870,
                "hash": "AB2A25557FF03911A8FC0A412293BE9D9FCB20CDD530EE05957A9859F8467C32",
                "inLedger": 12839308,
                "ledger_index": 12839308,
                "meta":
                    { "AffectedNodes": [ { "ModifiedNode":
                        { "FinalFields": { "Account": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
                            "BookDirectory": "51603377F758E3C8FA007C77312DDA06A737A1395CD5FC435D0547675A0517F6",
                            "BookNode": "0000000000000000",
                            "Flags": 0,
                            "OwnerNode": "0000000000000000",
                            "Sequence": 7031,
                            "TakerGets":
                                { "currency": "CNY",
                                    "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                                    "value": "1148.95233" },
                            "TakerPays": "170721000000" },
                            "LedgerEntryType": "Offer",
                            "LedgerIndex": "020110B8BED1F151B9D3AF9E5D412D8627CB08232B388ADE1F4B0C68E7608BEC",
                            "PreviousFields": { "TakerGets":
                                { "currency": "CNY",
                                    "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                                    "value": "1162.41233" },
                                "TakerPays": "172721000000" },
                            "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
                            "PreviousTxnLgrSeq": 12839301 } },
                        { "ModifiedNode":
                            { "FinalFields": { "Account": "jEoSyfChhUMzpRDttAJXuie8XhqyoPBYvV",
                                "Balance": "533983297806",
                                "Flags": 0,
                                "OwnerCount": 1,
                                "Sequence": 34380818 },
                                "LedgerEntryType": "AccountRoot",
                                "LedgerIndex": "109E80FB8CC6D82D4F7F7D77248C2C3C116ECCD4520B3D2A88421FFF94A57B1E",
                                "PreviousFields": { "Balance": "533983287806", "Sequence": 34380817 },
                                "PreviousTxnID": "756338B8F9D4DCC8D88382B1092B13F75F65F330970278AFC7449496FF9875E9",
                                "PreviousTxnLgrSeq": 12839308 } },
                        { "ModifiedNode":
                            { "FinalFields": { "Balance":
                                { "currency": "CNY",
                                    "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                                    "value": "-6872.222452374449" },
                                "Flags": 2228224,
                                "HighLimit":
                                    { "currency": "CNY",
                                        "issuer": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                                        "value": "10000000000" },
                                "HighNode": "0000000000000000",
                                "LowLimit":
                                    { "currency": "CNY",
                                        "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                                        "value": "0" },
                                "LowNode": "00000000000012A0" },
                                "LedgerEntryType": "SkywellState",
                                "LedgerIndex": "2600F8FCB87FEA15F74B0DB785016384C79AEA0730B62F597C1E576801BB813B",
                                "PreviousFields": { "Balance":
                                    { "currency": "CNY",
                                        "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                                        "value": "-6858.762452374449" } },
                                "PreviousTxnID": "9B28F7958E729F0F904410B132D1F81481B38DD9F017790A82168CD38C995331",
                                "PreviousTxnLgrSeq": 12838251 } },
                        { "ModifiedNode":
                            { "FinalFields":{ "Account": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
                                "Balance": "1496144192938",
                                "Flags": 0,
                                "OwnerCount": 8,
                                "Sequence": 7032 },
                                "LedgerEntryType": "AccountRoot",
                                "LedgerIndex": "40A20BDD3C226C987579F6C821BF84492E1C6B6EFB62311481BA6B8CB1D7775A",
                                "PreviousFields": { "Balance": "1494144192938" },
                                "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
                                "PreviousTxnLgrSeq": 12839301 } },
                        { "ModifiedNode":
                            { "FinalFields": { "Account": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                                "Balance": "500538133",
                                "Flags": 0,
                                "OwnerCount": 10,
                                "Sequence": 651 },
                                "LedgerEntryType": "AccountRoot",
                                "LedgerIndex": "B39BD926378886F7EF4F81CEF862FC4D1E8E6D1265945AA9EC40FD85132DC629",
                                "PreviousFields": { "Balance": "2500548133", "Sequence": 650 },
                                "PreviousTxnID": "5BA24DE17EF64EDF942D99F247ED1495F5A61ED9260513FEDCA3E4BADBADFF3E",
                                "PreviousTxnLgrSeq": 12839303 } },
                        { "ModifiedNode":
                            { "FinalFields": { "Balance":
                                { "currency": "CNY",
                                    "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                                    "value": "1148.954817858577" },
                                "Flags": 1114112,
                                "HighLimit":
                                    { "currency": "CNY",
                                        "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                                        "value": "0" },
                                "HighNode": "000000000000172A",
                                "LowLimit":
                                    { "currency": "CNY",
                                        "issuer": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
                                        "value": "10000000000" },
                                "LowNode": "0000000000000000" },
                                "LedgerEntryType": "SkywellState",
                                "LedgerIndex": "E3E9FE1827E83B52F7017D3038F8C769F09343801BB073A993DE620756069137",
                                "PreviousFields": { "Balance":
                                    { "currency": "CNY",
                                        "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                                        "value": "1162.414817858577" } },
                                "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
                                "PreviousTxnLgrSeq": 12839301 } } ],
                        "TransactionIndex": 3,
                        "TransactionResult": "tesSUCCESS" },
            "validated": true };
    "#;
    let v = process_tx(tx);
    println!("result： \n\n{}", v);
}
