#!/bin/bash

# Build and Run Script for Indubitably Rust Agent SDK Examples
# This script helps you build and run the example applications

set -e

echo "🚀 Indubitably Rust Agent SDK Examples"
echo "======================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the examples directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the examples directory"
    print_status "Navigate to the examples directory and try again:"
    echo "  cd examples"
    echo "  ./build_and_run.sh"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed or not in PATH"
    print_status "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
print_status "Rust version: $RUST_VERSION"

# Check if main SDK is built
if [ ! -d "../target" ]; then
    print_warning "Main SDK not built. Building it first..."
    cd ..
    cargo build
    cd examples
fi

# Function to build examples
build_examples() {
    print_status "Building all examples..."
    cargo build
    
    if [ $? -eq 0 ]; then
        print_success "Examples built successfully!"
    else
        print_error "Failed to build examples"
        exit 1
    fi
}

# Function to run a specific example
run_example() {
    local example_name=$1
    local display_name=$2
    
    print_status "Running $display_name..."
    echo ""
    
    if cargo run --bin "$example_name"; then
        print_success "$display_name completed successfully!"
    else
        print_error "$display_name failed to run"
    fi
    
    echo ""
    read -p "Press Enter to continue to the next example..."
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    cargo test
    
    if [ $? -eq 0 ]; then
        print_success "All tests passed!"
    else
        print_error "Some tests failed"
    fi
}

# Function to show available examples
show_examples() {
    echo "Available examples:"
    echo "  1. Simple Chat Assistant (simple_chat_assistant)"
    echo "  2. Multi-Agent System (multi_agent_system)"
    echo "  3. Web Chat Interface (web_chat_interface)"
    echo ""
}

# Function to run all examples in sequence
run_all_examples() {
    print_status "Running all examples in sequence..."
    echo ""
    
    run_example "simple_chat_assistant" "Simple Chat Assistant"
    run_example "multi_agent_system" "Multi-Agent System"
    
    print_status "Starting Web Chat Interface..."
    print_status "This will start a web server. Open http://localhost:8080 in your browser."
    print_status "Press Ctrl+C to stop the web server when done."
    echo ""
    
    cargo run --bin web_chat_interface
}

# Function to run a specific example by name
run_specific_example() {
    local example_name=$1
    
    case $example_name in
        "simple_chat_assistant"|"1")
            run_example "simple_chat_assistant" "Simple Chat Assistant"
            ;;
        "multi_agent_system"|"2")
            run_example "multi_agent_system" "Multi-Agent System"
            ;;
        "web_chat_interface"|"3")
            print_status "Starting Web Chat Interface..."
            print_status "Open http://localhost:8080 in your browser"
            print_status "Press Ctrl+C to stop the server"
            echo ""
            cargo run --bin web_chat_interface
            ;;
        *)
            print_error "Unknown example: $example_name"
            show_examples
            exit 1
            ;;
    esac
}

# Main menu
show_menu() {
    echo "What would you like to do?"
    echo ""
    echo "1. Build all examples"
    echo "2. Run tests"
    echo "3. Run Simple Chat Assistant"
    echo "4. Run Multi-Agent System"
    echo "5. Run Web Chat Interface"
    echo "6. Run all examples in sequence"
    echo "7. Show available examples"
    echo "8. Exit"
    echo ""
}

# Main script logic
main() {
    while true; do
        show_menu
        read -p "Enter your choice (1-8): " choice
        
        case $choice in
            1)
                build_examples
                ;;
            2)
                run_tests
                ;;
            3)
                run_specific_example "simple_chat_assistant"
                ;;
            4)
                run_specific_example "multi_agent_system"
                ;;
            5)
                run_specific_example "web_chat_interface"
                ;;
            6)
                run_all_examples
                ;;
            7)
                show_examples
                ;;
            8)
                print_success "Goodbye! 👋"
                exit 0
                ;;
            *)
                print_error "Invalid choice. Please enter a number between 1 and 8."
                ;;
        esac
        
        echo ""
        read -p "Press Enter to continue..."
        echo ""
    done
}

# Check command line arguments
if [ $# -eq 0 ]; then
    # No arguments, show interactive menu
    main
elif [ $# -eq 1 ]; then
    # One argument, run specific example
    case $1 in
        "build")
            build_examples
            ;;
        "test")
            run_tests
            ;;
        "all")
            run_all_examples
            ;;
        "help"|"-h"|"--help")
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  build     - Build all examples"
            echo "  test      - Run tests"
            echo "  all       - Run all examples in sequence"
            echo "  help      - Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 build                    # Build examples"
            echo "  $0 test                     # Run tests"
            echo "  $0 all                      # Run all examples"
            echo "  $0                          # Interactive menu"
            ;;
        *)
            run_specific_example "$1"
            ;;
    esac
else
    print_error "Too many arguments"
    echo "Usage: $0 [command]"
    echo "Run '$0 help' for more information"
    exit 1
fi
