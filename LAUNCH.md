
Launch validator1
```bash
/disco-grande/github-libs/kitties/target/release/node-kitties \
--chain=local \
--base-path /tmp/validator1 \
--alice \
--node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a \
--port 30333 \
--ws-port 9944

```


Launch validator2
```bash
/disco-grande/github-libs/kitties/target/release/node-kitties \
--chain=local \
--base-path /tmp/validator2 \
--bob \
--node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 \
--port 30334 \
--ws-port 9945

```

Launch validator3
```bash
/disco-grande/github-libs/kitties/target/release/node-kitties \
--chain=local \
--base-path /tmp/validator3 \
--name charlie  \
--node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
--port 30335 \
--ws-port=9946 \
--offchain-worker always
```

Launch validator4
```bash
/disco-grande/github-libs/kitties/target/release/node-kitties \
--chain=local \
--base-path /tmp/validator4 \
--name dave \
--node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a \
--port 30336 \
--ws-port 9947 \
--offchain-worker always
```