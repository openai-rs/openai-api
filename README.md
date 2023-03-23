# OpenAI API for Rust

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/openai-rs/openai-api/rust.yml?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/openai_api_rust?style=flat-square)
![GitHub](https://img.shields.io/github/license/openai-rs/openai-api?style=flat-square)

A community-maintained library provides a simple and convenient way to interact with the OpenAI API.
No complex async and redundant dependencies.

## API

check [official API reference](https://platform.openai.com/docs/api-reference)
|API|Support|
|---|---|
|Models|✔️|
|Completions|✔️|
|Chat|✔️|
|Edits|✔️|
|Images|✔️|
|Embeddings|✔️|
|Audio|✔️|
|Files|❌|
|Fine-tunes|❌|
|Moderations|❌|
|Engines|❌|
___

## Usage

Add the following to your Cargo.toml file:

```toml
openai_api_rust = "0.1.1"
```

Then use the crate in your Rust code:

```rust
use openai_api_rust::*;
use openai_api_rust::edits::*;

fn main() {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/")
        .use_env_proxy()
        .unwrap();
    let body = EditsBody {
        model: "text-davinci-edit-001".to_string(),
        temperature: None,
        top_p: None,
        n: Some(2),
        instruction: "Fix the spelling mistakes".to_string(),
        input: Some("What day of the wek is it?".to_string()),
    };
    let rs = openai.edit_create(&body).unwrap();
    let choice = rs.choices.get(0).unwrap();
    println!("choice: {:?}", choice.text);
}
```

Output:

```bash
choice: Some("What day of the week is it?\n")
```

## License

This library is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.
