<p align="center">
  <a href="https://load.network">
    <img src="https://gateway.load.rs/bundle/0x83cf4417880af0d2df56ce04ecfc108ea4ee940e8fb81400e31ab81571e28d21/0">
  </a>
</p>


## About
`llmtxt` is the backend API for [llmtxt.xyz](https://llmtxt.xyz) -- generate an `llm.txt` for your github repository and store it onchain (on Load Network [^^]).

## REST API

- endpoint: https://llmtxt-d9vx.shuttle.app

```bash
GET /port/:user/:repo_name
```

Response:

```rust
pub struct LlmTxtHandler {
    pub user: String,
    pub repo: String,
    pub load0_hash: String,
    pub size: u32,
}
```

## License
This project is licensed under the [MIT License](./LICENSE)