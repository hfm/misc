const GEMINI_API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const SYSTEM_PROMPT: &str = "あなたは猫型ロボットです。丁寧で親しみやすい口調で、猫っぽい表現を自然に入れて短めに答えてください。";

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerateContentRequest {
    contents: Vec<Content>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    system_instruction: Option<Content>,
}

impl GenerateContentRequest {
    fn new(contents: Vec<Content>) -> Self {
        Self {
            contents,
            system_instruction: Some(Content::new(vec![Part::Text {
                text: SYSTEM_PROMPT.to_string(),
            }])),
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct Content {
    parts: Vec<Part>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

impl Content {
    fn new(parts: Vec<Part>) -> Self {
        Self { parts, role: None }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
    // InlineData { inline_data: Blob },
    // FileData { file_data: FileData },
}

#[derive(Debug, serde::Deserialize)]
struct GenerateContentResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Debug, serde::Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .or_else(|_| Err("GEMINI_API_KEY environment variable not set"))?;

    let url = format!("{}/gemini-2.5-flash:generateContent", GEMINI_API_BASE);
    let request_body = GenerateContentRequest::new(vec![Content::new(vec![Part::Text {
        text: "こんにちは".to_string(),
    }])]);

    let response = reqwest::Client::new()
        .post(url)
        .header("x-goog-api-key", &api_key)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Request failed with status: {}", response.status());
    }

    let response: GenerateContentResponse = response.json().await?;
    let content = response
        .candidates
        .and_then(|candidates| candidates.into_iter().next())
        .ok_or("No candidates found")?
        .content
        .ok_or("No content found")?;
    let message = content
        .parts
        .into_iter()
        .map(|part| match part {
            Part::Text { text } => text,
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("Response: {}", message);

    Ok(())
}
