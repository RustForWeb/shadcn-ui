#!/bin/bash

# 🚀 Enhanced Lazy Loading System - Setup Script
# This script helps you copy and integrate the enhanced lazy loading system into your other projects

set -e

echo "🚀 Enhanced Lazy Loading System - Setup Script"
echo "================================================"
echo ""

# Get the current directory (where this script is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_NAME="enhanced-lazy-loading-demo"

echo "📍 Current project location: $SCRIPT_DIR"
echo ""

# Function to copy the system to a target directory
copy_to_project() {
    local target_dir="$1"
    
    if [ -z "$target_dir" ]; then
        echo "❌ Please provide a target directory"
        echo "Usage: $0 <target-directory>"
        exit 1
    fi
    
    if [ ! -d "$target_dir" ]; then
        echo "📁 Creating target directory: $target_dir"
        mkdir -p "$target_dir"
    fi
    
    echo "📋 Copying enhanced lazy loading system to: $target_dir"
    
    # Create the target project directory
    local project_dir="$target_dir/$PROJECT_NAME"
    mkdir -p "$project_dir"
    
    # Copy essential files
    echo "  📁 Copying source files..."
    cp -r "$SCRIPT_DIR/src" "$project_dir/"
    cp -r "$SCRIPT_DIR/style" "$project_dir/"
    cp "$SCRIPT_DIR/Cargo.toml" "$project_dir/"
    cp "$SCRIPT_DIR/README.md" "$project_dir/"
    cp "$SCRIPT_DIR/example-usage.md" "$project_dir/"
    cp "$SCRIPT_DIR/DEMO_FEATURES.md" "$project_dir/"
    
    # Copy Trunk configuration if it exists
    if [ -f "$SCRIPT_DIR/Trunk.toml" ]; then
        cp "$SCRIPT_DIR/Trunk.toml" "$project_dir/"
    fi
    
    # Copy index.html if it exists
    if [ -f "$SCRIPT_DIR/index.html" ]; then
        cp "$SCRIPT_DIR/index.html" "$project_dir/"
    fi
    
    echo "✅ Copy completed successfully!"
    echo ""
    
    # Create integration guide
    create_integration_guide "$target_dir"
}

# Function to create integration guide
create_integration_guide() {
    local target_dir="$1"
    local guide_file="$target_dir/INTEGRATION_GUIDE.md"
    
    echo "📝 Creating integration guide..."
    
    cat > "$guide_file" << 'EOF'
# 🔗 Integration Guide for Enhanced Lazy Loading System

## 🚀 Quick Setup

### 1. Add to Your Cargo.toml

```toml
[dependencies]
enhanced-lazy-loading-demo = { path = "./enhanced-lazy-loading-demo" }
```

### 2. Import in Your Rust Code

```rust
use enhanced_lazy_loading_demo::lazy_loading::LazyComponentWrapper;
use enhanced_lazy_loading_demo::bundle_analyzer::BundleAnalysisDisplay;

#[component]
pub fn MyApp() -> impl IntoView {
    view! {
        <div>
            <BundleAnalysisDisplay />
            <LazyComponentWrapper name="Alert".to_string() />
        </div>
    }
}
```

### 3. Include CSS

```html
<link rel="stylesheet" href="./enhanced-lazy-loading-demo/style/optimization.css">
```

## 📁 Project Structure

```
enhanced-lazy-loading-demo/
├── src/
│   ├── lazy_loading.rs      # Main lazy loading components
│   ├── bundle_analyzer.rs    # Bundle analysis display
│   ├── dynamic_loader.rs     # Dynamic loading management
│   └── app.rs               # Main application (example)
├── style/
│   └── optimization.css      # Complete styling system
├── Cargo.toml               # Dependencies and configuration
├── README.md                # Comprehensive documentation
├── example-usage.md         # Usage examples
└── DEMO_FEATURES.md         # Feature showcase
```

## 🎯 Next Steps

1. **Test the Integration**: Build your project to ensure everything works
2. **Customize Components**: Add your own components to the system
3. **Modify Styling**: Customize the CSS to match your design
4. **Extend Features**: Add new categories and functionality

## 🔧 Customization

See `example-usage.md` for detailed customization examples.

## 📚 Documentation

- **README.md**: Complete feature overview
- **DEMO_FEATURES.md**: Interactive feature showcase
- **example-usage.md**: Real-world integration examples

---

**Happy coding! 🎉✨**
EOF

    echo "✅ Integration guide created: $guide_file"
    echo ""
}

# Function to show usage
show_usage() {
    echo "Usage: $0 <target-directory>"
    echo ""
    echo "Examples:"
    echo "  $0 ~/my-workspace                    # Copy to your workspace"
    echo "  $0 ../my-other-project              # Copy to sibling directory"
    echo "  $0 /path/to/my/web/app              # Copy to specific path"
    echo ""
    echo "This will create a copy of the enhanced lazy loading system"
    echo "that you can use as a dependency in your other projects."
    echo ""
}

# Function to create a sample project
create_sample_project() {
    local target_dir="$1"
    local sample_dir="$target_dir/sample-lazy-loading-app"
    
    echo "🎯 Creating sample project..."
    mkdir -p "$sample_dir"
    
    # Create sample Cargo.toml
    cat > "$sample_dir/Cargo.toml" << 'EOF'
[package]
name = "sample-lazy-loading-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
enhanced-lazy-loading-demo = { path = "../enhanced-lazy-loading-demo" }
gloo-timers = { version = "0.3.0", features = ["futures"] }

[build-dependencies]
trunk = "0.21"
EOF

    # Create sample main.rs
    cat > "$sample_dir/src/main.rs" << 'EOF'
use leptos::*;
use enhanced_lazy_loading_demo::lazy_loading::LazyComponentWrapper;
use enhanced_lazy_loading_demo::bundle_analyzer::BundleAnalysisDisplay;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="sample-app">
            <header>
                <h1>"Sample Lazy Loading App"</h1>
                <p>"This demonstrates the enhanced lazy loading system"</p>
            </header>
            
            <main>
                <BundleAnalysisDisplay />
                
                <section class="component-showcase">
                    <h2>"Component Library"</h2>
                    
                    <div class="component-grid">
                        <LazyComponentWrapper name="Alert".to_string() />
                        <LazyComponentWrapper name="Button".to_string() />
                        <LazyComponentWrapper name="Card".to_string() />
                        <LazyComponentWrapper name="Form".to_string() />
                    </div>
                </section>
            </main>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
EOF

    # Create sample index.html
    cat > "$sample_dir/index.html" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Sample Lazy Loading App</title>
    <link rel="stylesheet" href="../enhanced-lazy-loading-demo/style/optimization.css">
    <style>
        .sample-app {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }
        
        .sample-app header {
            text-align: center;
            margin-bottom: 40px;
            padding: 30px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border-radius: 16px;
        }
        
        .component-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <script type="module">
        import init, { main } from './pkg/sample_lazy_loading_app.js';
        init().then(() => main());
    </script>
</body>
</html>
EOF

    # Create Trunk.toml
    cat > "$sample_dir/Trunk.toml" << 'EOF'
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "index.html"]
EOF

    # Create README for sample project
    cat > "$sample_dir/README.md" << 'EOF'
# 🎯 Sample Lazy Loading App

This is a sample project demonstrating how to integrate the Enhanced Lazy Loading System.

## 🚀 Quick Start

1. **Build the project:**
   ```bash
   trunk build
   ```

2. **Serve locally:**
   ```bash
   trunk serve --open
   ```

3. **Explore the components:**
   - Try the lazy loading components
   - See the bundle analysis display
   - Experience the professional UI

## 🔧 Customization

- Modify `src/main.rs` to add your own components
- Customize the styling in `index.html`
- Add more LazyComponentWrapper instances

## 📚 Learn More

See the parent `enhanced-lazy-loading-demo` folder for:
- Complete documentation
- Usage examples
- Feature showcase
- Customization guides

---

**This sample shows the power of the Enhanced Lazy Loading System!** 🚀✨
EOF

    echo "✅ Sample project created: $sample_dir"
    echo ""
    echo "🎯 To test the integration:"
    echo "  cd $sample_dir"
    echo "  trunk build"
    echo "  trunk serve --open"
    echo ""
}

# Main script logic
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_usage
    exit 0
fi

if [ "$1" = "--sample" ]; then
    if [ -z "$2" ]; then
        echo "❌ Please provide a target directory for the sample project"
        echo "Usage: $0 --sample <target-directory>"
        exit 1
    fi
    
    copy_to_project "$2"
    create_sample_project "$2"
    
    echo "🎉 Setup complete! You now have:"
    echo "  📁 Enhanced lazy loading system: $2/$PROJECT_NAME"
    echo "  🎯 Sample project: $2/sample-lazy-loading-app"
    echo "  📝 Integration guide: $2/INTEGRATION_GUIDE.md"
    echo ""
    echo "🚀 Ready to integrate into your projects!"
    
elif [ -n "$1" ]; then
    copy_to_project "$1"
    
    echo "🎉 Setup complete! You now have:"
    echo "  📁 Enhanced lazy loading system: $1/$PROJECT_NAME"
    echo "  📝 Integration guide: $1/INTEGRATION_GUIDE.md"
    echo ""
    echo "🚀 Ready to integrate into your projects!"
    echo ""
    echo "💡 Tip: Use '--sample' flag to also create a sample project:"
    echo "  $0 --sample $1"
    
else
    show_usage
    exit 1
fi
