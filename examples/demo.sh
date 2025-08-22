#!/bin/bash

# Demo Script for Indubitably Rust Agent SDK Examples
# This script demonstrates the capabilities of the examples

set -e

echo "🎭 Indubitably Rust Agent SDK Examples Demo"
echo "==========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_demo() {
    echo -e "${BLUE}[DEMO]${NC} $1"
}

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if examples are built
if [ ! -d "target/debug" ]; then
    print_warning "Examples not built yet. Building them first..."
    cargo build
fi

echo "This demo will showcase three different examples:"
echo "1. Simple Chat Assistant - Basic tool usage"
echo "2. Multi-Agent System - Collaborative agents"
echo "3. Web Chat Interface - Web-based chat"
echo ""

read -p "Press Enter to start the demo..."

# Demo 1: Simple Chat Assistant
echo ""
print_demo "Demo 1: Simple Chat Assistant"
echo "=================================="
print_info "This example shows basic tool usage with web browsing and calculator tools."
echo ""

print_demo "Sample interactions you can try:"
echo "- Ask about weather: 'What's the weather like today?'"
echo "- Request calculations: 'Can you calculate 15 * 23 for me?'"
echo "- Search for information: 'Search for information about AI'"
echo ""

read -p "Press Enter to run the Simple Chat Assistant demo..."

# Run the simple chat assistant in the background
cargo run --bin simple_chat_assistant &
SIMPLE_PID=$!

# Wait a bit for it to start
sleep 2

print_info "Simple Chat Assistant is running. Try the sample interactions above."
print_info "Type 'quit' to exit the demo and continue to the next example."
echo ""

# Wait for the process to finish
wait $SIMPLE_PID

# Demo 2: Multi-Agent System
echo ""
print_demo "Demo 2: Multi-Agent System"
echo "==============================="
print_info "This example demonstrates multiple specialized agents working together."
echo ""

print_demo "Available agents:"
echo "- Research Agent: Gathers information using web tools"
echo "- Analysis Agent: Analyzes and summarizes data"
echo "- Creative Agent: Creates content based on analyzed information"
echo ""

print_demo "Sample interactions you can try:"
echo "- Research: 'Research the latest AI developments'"
echo "- Analysis: 'Analyze the research findings'"
echo "- Creative: 'Create a story about AI technology'"
echo ""

read -p "Press Enter to run the Multi-Agent System demo..."

# Run the multi-agent system in the background
cargo run --bin multi_agent_system &
MULTI_PID=$!

# Wait a bit for it to start
sleep 2

print_info "Multi-Agent System is running. Try the sample interactions above."
print_info "Type 'quit' to exit the demo and continue to the next example."
echo ""

# Wait for the process to finish
wait $MULTI_PID

# Demo 3: Web Chat Interface
echo ""
print_demo "Demo 3: Web Chat Interface"
echo "==============================="
print_info "This example shows a modern web-based chat interface using Actix-web."
echo ""

print_demo "Features:"
echo "- Beautiful, responsive web UI"
echo "- Real-time chat capabilities"
echo "- Session management"
echo "- RESTful API endpoints"
echo ""

print_demo "To use the web interface:"
echo "1. The server will start on http://localhost:8080"
echo "2. Open your browser and navigate to the URL"
echo "3. Start chatting with the AI assistant"
echo "4. Try asking about weather, calculations, or web searches"
echo "5. Press Ctrl+C to stop the server when done"
echo ""

read -p "Press Enter to start the Web Chat Interface demo..."

print_info "Starting web server on http://localhost:8080..."
print_info "Open your browser and navigate to the URL above."
print_info "Press Ctrl+C to stop the server when you're done exploring."
echo ""

# Run the web chat interface
cargo run --bin web_chat_interface

echo ""
print_info "Demo completed! 🎉"
echo ""
print_info "What you've seen:"
echo "- Basic tool integration with web browsing and calculations"
echo "- Multi-agent collaboration and workflow orchestration"
echo "- Modern web interface with real-time chat capabilities"
echo ""
print_info "Next steps:"
echo "- Explore the code to understand how everything works"
echo "- Modify the tools to add real functionality"
echo "- Create your own specialized agents"
echo "- Build custom workflows for your use case"
echo ""
print_info "Happy coding with the Indubitably Rust Agent SDK! 🦀✨"
