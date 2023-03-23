# OpenAI API for Rust

A Rust library provides a simple and convenient way to interact with the OpenAI API.

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/openai-rs/openai-api/rust.yml?style=flat-square)
![GitHub](https://img.shields.io/github/license/openai-rs/openai-api?style=flat-square)

## Usage

Add the following to your Cargo.toml file:

```toml
openai_api = "0.1.1"
```

Then use the crate in your Rust code:

```rust
use openai_api::*;
use openai_api::edits::*;

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

## API

### Models

> List and describe the various models available in the API. You can refer to the Models documentation to understand what models are available and the differences between them.

### Completions

> Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position.

### Chat

> Given a chat conversation, the model will return a chat completion response.

### Edits

> Given a prompt and an instruction, the model will return an edited version of the prompt.

### Images

> Given a prompt and/or an input image, the model will generate a new image.

### Embeddings

> Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.

### Audio

> Learn how to turn audio into text.

## License

This library is distributed under the terms of the MIT license. See [LICENSE](https://opensource.org/license/mit/) for details.
