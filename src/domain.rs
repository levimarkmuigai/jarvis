pub struct Prompt {
    pub raw: String,
}

impl Prompt {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn to_payload(&self) -> String {
        format!("Answer this like Jarvis from Iron Man: {}", self.raw)
    }
}

pub struct Response {
    pub content: String,
}

pub trait AIClient {
    fn ask(&self, prompt: &Prompt) -> Response;
}

pub struct JarvisService<C: AIClient> {
    client: C,
}

impl<C: AIClient> JarvisService<C>{
    pub fn new(client: C) -> Self {
        Self { client }
    }
    pub fn ask(&self, prompt: &Prompt) -> Response {
        self.client.ask(prompt)
    }
}
