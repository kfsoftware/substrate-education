
```bash
rm -rf resources
mkdir -p resources/node1
mkdir -p resources/node2
mkdir -p resources/node3
```



Node 1
```bash
subkey generate --scheme sr25519 --output-type=Json > resources/node1/sr25519.json
export PHRASE=$(cat resources/node1/sr25519.json | jq -r .secretPhrase)  
subkey inspect --scheme ed25519 $PHRASE --output-type=Json > resources/node1/ed25519.json
AURA_ADDRESS=$(cat resources/node1/sr25519.json | jq -r .ss58Address)
echo "NODE1_AURA_ADDRESS= ${AURA_ADDRESS}"
GRANDPA_ADDRESS=$(cat resources/node1/ed25519.json | jq -r .ss58Address)
echo "NODE1_GRANDPA_ADDRESS= ${GRANDPA_ADDRESS}"
```

Node 2
```bash
subkey generate --scheme sr25519 --output-type=Json > resources/node2/sr25519.json
export PHRASE=$(cat resources/node2/sr25519.json | jq -r .secretPhrase)  
subkey inspect --scheme ed25519 $PHRASE --output-type=Json > resources/node2/ed25519.json

AURA_ADDRESS=$(cat resources/node2/sr25519.json | jq -r .ss58Address)
echo "NODE2_AURA_ADDRESS= ${AURA_ADDRESS}"
GRANDPA_ADDRESS=$(cat resources/node2/ed25519.json | jq -r .ss58Address)
echo "NODE2_GRANDPA_ADDRESS= ${GRANDPA_ADDRESS}"

```

Node 3
```bash
subkey generate --scheme sr25519 --output-type=Json > resources/node3/sr25519.json
export PHRASE=$(cat resources/node3/sr25519.json | jq -r .secretPhrase)  
subkey inspect --scheme ed25519 $PHRASE --output-type=Json > resources/node3/ed25519.json
AURA_ADDRESS=$(cat resources/node3/sr25519.json | jq -r .ss58Address)
echo "NODE3_AURA_ADDRESS= ${AURA_ADDRESS}"
GRANDPA_ADDRESS=$(cat resources/node3/ed25519.json | jq -r .ss58Address)
echo "NODE3_GRANDPA_ADDRESS= ${GRANDPA_ADDRESS}"
```

```bash
./target/release/node-kitties build-spec --disable-default-bootnode --chain local > customSpec.json

/disco-grande/github-libs/kitties/target/release/node-kitties \
build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json

```

Launch validator1
```bash
rm -rf /disco-grande/kfs/network/node1
export NODE1_KEY="51f23ff088192967ea6201a3a4d78ceb024835adedbfbe483ab1b0c5e01a26c3"  # 12D3KooWQaNSARF1MfSTVSt6nfy3x5DK8AZ6k2777m7UKFWPWHxq
/disco-grande/github-libs/kitties/target/release/node-kitties \
  --base-path /disco-grande/kfs/network/node1 \
  --chain /disco-grande/github-libs/kitties/customSpecRaw.json \
  --port 30333 \
  --ws-port 9944 \
  --node-key=$NODE1_KEY \
  --rpc-port 9933 \
  --public-addr "/dns4/bootnode1.dev.kfs.network/tcp/30333/p2p/12D3KooWQaNSARF1MfSTVSt6nfy3x5DK8AZ6k2777m7UKFWPWHxq" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01


PUBLIC_KEY=$(cat resources/node1/sr25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node1/sr25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["aura", $mnemonicPhrase, $publicKey] }' > resources/node1/aura.json
curl -X POST https://rpc.bootnode1.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node1/aura.json"

PUBLIC_KEY=$(cat resources/node1/ed25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node1/ed25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["gran", $mnemonicPhrase, $publicKey] }' > resources/node1/gran.json
curl -X POST https://rpc.bootnode1.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node1/gran.json"


```


Launch validator2
```bash
rm -rf /disco-grande/kfs/network/node2
export NODE2_KEY=d2af9e0bee5f3ebd8da6d315ce0a40a106530d3cd6c4f791de2e112b81193706 # 12D3KooWE4Qgezev1fYRPodnBcp294K5Yc5s4RAyMzZ9K2UxgFNq
/disco-grande/github-libs/kitties/target/release/node-kitties \
  --base-path /disco-grande/kfs/network/node2 \
  --chain /disco-grande/github-libs/kitties/customSpecRaw.json \
  --port 30334 \
  --node-key=$NODE2_KEY \
  --ws-port 9946 \
  --rpc-port 9934 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes "/dns4/bootnode1.dev.kfs.network/tcp/30333/p2p/12D3KooWQaNSARF1MfSTVSt6nfy3x5DK8AZ6k2777m7UKFWPWHxq"



PUBLIC_KEY=$(cat resources/node2/sr25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node2/sr25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["aura", $mnemonicPhrase, $publicKey] }' > resources/node2/aura.json
curl -X POST https://rpc.bootnode2.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node2/aura.json"

PUBLIC_KEY=$(cat resources/node2/ed25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node2/ed25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["gran", $mnemonicPhrase, $publicKey] }' > resources/node2/gran.json
curl -X POST https://rpc.bootnode2.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node2/gran.json"

```




Launch validator3
```bash
rm -rf /disco-grande/kfs/network/node3

export NODE3_KEY=29fcb50c27aa23e9ca3b169e81d4eca10586b090f0362125bfdbab3099294aa8 # 12D3KooWC4AVPTwTLb56mewipeZF8c5fG68asdK2hju1G65HiAU7
/disco-grande/github-libs/kitties/target/release/node-kitties \
  --base-path /disco-grande/kfs/network/node3 \
  --chain /disco-grande/github-libs/kitties/customSpecRaw.json \
  --node-key=$NODE3_KEY \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode03 \
  --bootnodes "/dns4/bootnode1.dev.kfs.network/tcp/30333/p2p/12D3KooWQaNSARF1MfSTVSt6nfy3x5DK8AZ6k2777m7UKFWPWHxq"

PUBLIC_KEY=$(cat resources/node3/sr25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node3/sr25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["aura", $mnemonicPhrase, $publicKey] }' > resources/node3/aura.json
curl -X POST https://rpc.bootnode3.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node3/aura.json"

PUBLIC_KEY=$(cat resources/node3/ed25519.json | jq -r .publicKey)
SECRET_PHRASE=$(cat resources/node3/ed25519.json | jq -r .secretPhrase)

jq -n --arg mnemonicPhrase $SECRET_PHRASE --arg publicKey $PUBLIC_KEY \
    '{  "jsonrpc": "2.0",  "id": 1,  "method": "author_insertKey",  "params": ["gran", $mnemonicPhrase, $publicKey] }' > resources/node3/gran.json
curl -X POST https://rpc.bootnode3.test.kfs.network -H "Content-Type:application/json;charset=utf-8" -d "@resources/node3/gran.json"

```

