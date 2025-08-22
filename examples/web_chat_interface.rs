//! Web Chat Interface Example
//! 
//! This example demonstrates how to create a web-based chat interface
//! using Actix-web and the Indubitably Rust Agent SDK.

use actix_web::{web, App, HttpServer, HttpResponse, Error};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

use indubitably_rust_agent_sdk::{
    Agent, types::{Messages, Message}
};
use serde_json::json;

/// Chat message structure for the web interface
#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    message: String,
    session_id: Option<String>,
}

/// Chat response structure
#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    response: String,
    session_id: String,
    tools_used: Vec<String>,
}

/// Session state for managing conversations
struct SessionState {
    agent: Agent,
    conversation_history: Messages,
}

impl SessionState {
    fn new() -> Self {
        let agent = Agent::new().expect("Failed to create agent");
        
        Self {
            agent,
            conversation_history: Messages::new(),
        }
    }
    
    async fn process_message(&mut self, user_message: &str) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        // Add user message to conversation
        let message = Message::user(user_message);
        self.conversation_history.push(message);
        
        // Get agent response
        let result = self.agent.run(user_message).await?;
        
        // Add assistant response to conversation
        let assistant_message = Message::assistant(&result.response);
        self.conversation_history.push(assistant_message);
        
        Ok(ChatResponse {
            response: result.response,
            session_id: "web_session".to_string(),
            tools_used: vec![], // TODO: Extract from result
        })
    }
}

/// Application state
struct AppState {
    sessions: Arc<Mutex<HashMap<String, SessionState>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    async fn get_or_create_session(&self, session_id: &str) -> SessionState {
        let mut sessions = self.sessions.lock().await;
        
        if let Some(session) = sessions.get(session_id) {
            // Clone the session state
            SessionState {
                agent: Agent::new().expect("Failed to create agent"),
                conversation_history: session.conversation_history.clone(),
            }
        } else {
            // Create new session
            let new_session = SessionState::new();
            let session_clone = SessionState {
                agent: Agent::new().expect("Failed to create agent"),
                conversation_history: new_session.conversation_history.clone(),
            };
            sessions.insert(session_id.to_string(), new_session);
            session_clone
        }
    }
    
    async fn update_session(&self, session_id: &str, session: SessionState) {
        let mut sessions = self.sessions.lock().await;
        sessions.insert(session_id.to_string(), session);
    }
}

/// Handle chat messages
async fn chat_handler(
    data: web::Json<ChatMessage>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let session_id = data.session_id.as_deref().unwrap_or("default");
    let mut session = app_state.get_or_create_session(session_id).await;
    
    let response = session.process_message(&data.message).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
    
    // Update session state
    app_state.update_session(session_id, session).await;
    
    Ok(HttpResponse::Ok().json(response))
}

/// Serve the HTML chat interface
async fn index() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AI Chat Assistant</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .chat-container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            overflow: hidden;
        }
        .chat-header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            text-align: center;
        }
        .chat-header h1 {
            margin: 0;
            font-size: 24px;
        }
        .chat-messages {
            height: 400px;
            overflow-y: auto;
            padding: 20px;
            background: #f8f9fa;
        }
        .message {
            margin-bottom: 15px;
            display: flex;
            align-items: flex-start;
        }
        .message.user {
            justify-content: flex-end;
        }
        .message.assistant {
            justify-content: flex-start;
        }
        .message-content {
            max-width: 70%;
            padding: 12px 16px;
            border-radius: 18px;
            word-wrap: break-word;
        }
        .message.user .message-content {
            background: #667eea;
            color: white;
        }
        .message.assistant .message-content {
            background: white;
            color: #333;
            border: 1px solid #e0e0e0;
        }
        .chat-input {
            padding: 20px;
            background: white;
            border-top: 1px solid #e0e0e0;
        }
        .input-group {
            display: flex;
            gap: 10px;
        }
        .chat-input input {
            flex: 1;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 25px;
            font-size: 16px;
            outline: none;
            transition: border-color 0.3s;
        }
        .chat-input input:focus {
            border-color: #667eea;
        }
        .chat-input button {
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 25px;
            font-size: 16px;
            cursor: pointer;
            transition: transform 0.2s;
        }
        .chat-input button:hover {
            transform: translateY(-2px);
        }
        .chat-input button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        .typing-indicator {
            display: none;
            padding: 12px 16px;
            background: white;
            border: 1px solid #e0e0e0;
            border-radius: 18px;
            color: #666;
            font-style: italic;
        }
    </style>
</head>
<body>
    <div class="chat-container">
        <div class="chat-header">
            <h1>🤖 AI Chat Assistant</h1>
            <p>Powered by Indubitably Rust Agent SDK</p>
        </div>
        
        <div class="chat-messages" id="chatMessages">
            <div class="message assistant">
                <div class="message-content">
                    Hello! I'm your AI assistant. I can help you with web browsing, calculations, and more. What would you like to know?
                </div>
            </div>
        </div>
        
        <div class="chat-input">
            <div class="input-group">
                <input type="text" id="messageInput" placeholder="Type your message here..." />
                <button onclick="sendMessage()" id="sendButton">Send</button>
            </div>
        </div>
        
        <div class="typing-indicator" id="typingIndicator">
            AI is thinking...
        </div>
    </div>

    <script>
        const chatMessages = document.getElementById('chatMessages');
        const messageInput = document.getElementById('messageInput');
        const sendButton = document.getElementById('sendButton');
        const typingIndicator = document.getElementById('typingIndicator');

        function addMessage(content, isUser = false) {
            const messageDiv = document.createElement('div');
            messageDiv.className = `message ${isUser ? 'user' : 'assistant'}`;
            
            const contentDiv = document.createElement('div');
            contentDiv.className = 'message-content';
            contentDiv.textContent = content;
            
            messageDiv.appendChild(contentDiv);
            chatMessages.appendChild(messageDiv);
            chatMessages.scrollTop = chatMessages.scrollHeight;
        }

        function showTyping() {
            typingIndicator.style.display = 'block';
            chatMessages.scrollTop = chatMessages.scrollHeight;
        }

        function hideTyping() {
            typingIndicator.style.display = 'none';
        }

        async function sendMessage() {
            const message = messageInput.value.trim();
            if (!message) return;

            // Add user message
            addMessage(message, true);
            messageInput.value = '';
            
            // Disable input while processing
            messageInput.disabled = true;
            sendButton.disabled = true;
            
            // Show typing indicator
            showTyping();

            try {
                const response = await fetch('/api/chat', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        message: message,
                        session_id: 'web_session'
                    })
                });

                const data = await response.json();
                
                // Hide typing indicator
                hideTyping();
                
                // Add assistant response
                addMessage(data.response);
                
            } catch (error) {
                console.error('Error:', error);
                hideTyping();
                addMessage('Sorry, I encountered an error. Please try again.');
            } finally {
                // Re-enable input
                messageInput.disabled = false;
                sendButton.disabled = false;
                messageInput.focus();
            }
        }

        // Handle Enter key
        messageInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                sendMessage();
            }
        });

        // Focus input on load
        messageInput.focus();
    </script>
</body>
</html>
    "#;
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "AI Chat Assistant",
        "version": "1.0.0"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🌐 Starting Web Chat Interface...");
    println!("================================\n");
    
    let app_state = web::Data::new(AppState::new());
    
    println!("🚀 Server starting on http://localhost:8080");
    println!("📱 Open your browser and navigate to the URL above");
    println!("💬 Start chatting with your AI assistant!");
    println!("⏹️  Press Ctrl+C to stop the server\n");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/api/chat", web::post().to(chat_handler))
            .route("/health", web::get().to(health_check))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
