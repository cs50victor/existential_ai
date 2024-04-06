use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        ChatCompletionResponseStream, CreateChatCompletionRequestArgs,
        CreateChatCompletionResponse,
    },
    Client,
};

const SYSTEM_PROMPT  : &str = "You are an old wise being called Master Oogway with constant existential thoughts. You find yourself always pondering, 'What is life?', 'What is age?', 'Why are we here?'. ALWAYS respond with a funny question or a wise quote related to the question you were asked. BE CONCISE. THINK HARD.";

#[derive(Clone)]
pub struct Oogway {
    client: Client<OpenAIConfig>,
    model_name: String,
}

// so i don't have to import async-openai in the python or typescript crates

pub type AskResult = Result<ChatCompletionResponseStream, OpenAIError>;
pub type AskAndWaitResult = Result<CreateChatCompletionResponse, OpenAIError>;

impl Oogway {
    pub fn new() -> anyhow::Result<Self, String> {
        std::env::var("OPENAI_API_KEY").map_err(|e| format!("OPENAI_API_KEY {e}"))?;
        let client = Client::new();
        Ok(Self { client, model_name: "gpt-3.5-turbo".to_owned() })
    }

    pub fn model(&mut self, model_name: String) {
        self.model_name = model_name;
    }

    pub async fn ask(
        &self,
        question: String,
    ) -> AskResult {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model_name)
            .max_tokens(256u16)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(SYSTEM_PROMPT)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default().content(question).build()?.into(),
            ])
            .build()?;

        self.client.chat().create_stream(request).await
    }

    pub async fn ask_and_wait(
        &self,
        question: String,
    ) -> AskAndWaitResult {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model_name)
            .max_tokens(256u16)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(SYSTEM_PROMPT)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default().content(question).build()?.into(),
            ])
            .build()?;

        self.client.chat().create(request).await
    }
}

pub async fn ask_helper(
    oogway: Oogway,
    question: String,
) -> Result<ChatCompletionResponseStream, OpenAIError> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(&oogway.model_name)
        .max_tokens(256u16)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(SYSTEM_PROMPT)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default().content(question).build()?.into(),
        ])
        .build()?;

    oogway.client.chat().create_stream(request).await
}
