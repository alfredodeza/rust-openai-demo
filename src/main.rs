use std::error::Error;
use async_openai::{Client, config::OpenAIConfig};
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    //Client,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // No need for a CLI framework, lets grab all the arguments in the CLI
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide one argument as input.");
        std::process::exit(1);
    }
    let input: &str = &args[1];

    let config = OpenAIConfig::new()
        .with_api_key("") // we are talking to a local server, so no need for an API key
        .with_api_base("http://localhost:8080/v1"); // hardcoded to the local server

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(input)
                .build()?
                .into(),
            // An example on how to inject more context, perhaps with RAG
            // ChatCompletionRequestAssistantMessageArgs::default()
            //     .content("")
            //     .build()?
            //     .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: \nRole: {}  \nContent: \n{}",
            choice.index, choice.message.role, choice.message.content.unwrap_or("No response".to_string())
        );
    }

    Ok(())
}
