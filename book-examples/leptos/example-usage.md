# 📚 Example Usage in Your Web App

## 🚀 **How to Use This Enhanced Lazy Loading System in Your Projects**

### **Step 1: Copy the System to Your Local Development**

```bash
# Clone or copy the enhanced-lazy-loading-demo folder to your local workspace
cp -r /path/to/enhanced-lazy-loading-demo ~/my-workspace/
```

### **Step 2: Add as a Path Dependency in Your Project**

```toml
# In your project's Cargo.toml
[dependencies]
enhanced-lazy-loading-demo = { path = "../enhanced-lazy-loading-demo" }

# Or if you want to use specific features
[dependencies]
enhanced-lazy-loading-demo = { path = "../enhanced-lazy-loading-demo", features = ["essential"] }
```

### **Step 3: Import and Use in Your App**

```rust
use enhanced_lazy_loading_demo::lazy_loading::LazyComponentWrapper;
use enhanced_lazy_loading_demo::bundle_analyzer::BundleAnalysisDisplay;

#[component]
pub fn MyWebApp() -> impl IntoView {
    view! {
        <div class="my-app">
            <header>
                <h1>"My Amazing Web App"</h1>
            </header>
            
            <main>
                // Use the bundle analysis display
                <BundleAnalysisDisplay />
                
                // Create your own component showcase
                <section class="my-components">
                    <h2>"My Component Library"</h2>
                    
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
```

### **Step 4: Include the CSS**

```html
<!-- In your index.html or main HTML file -->
<head>
    <link rel="stylesheet" href="../enhanced-lazy-loading-demo/style/optimization.css">
    <!-- Or copy the CSS file to your project and reference it locally -->
    <link rel="stylesheet" href="./styles/enhanced-lazy-loading.css">
</head>
```

### **Step 5: Customize for Your Needs**

#### **Add Your Own Components:**

```rust
// In lazy_loading.rs, add your components to the component_info closure
"CustomButton" => ComponentInfo {
    name: "CustomButton".to_string(),
    category: "Custom Components".to_string(),
    estimated_size: "18KB".to_string(),
    dependencies: vec!["my-custom-lib".to_string()],
    description: "A custom button component for my app".to_string(),
},
```

#### **Create Custom Categories:**

```rust
// In app.rs, add your own category section
<div class="category" data-category="custom">
    <h4>"🛠️ Custom Components"</h4>
    <div class="lazy-grid">
        <LazyComponentWrapper name="CustomButton".to_string() />
        <LazyComponentWrapper name="CustomModal".to_string() />
        <LazyComponentWrapper name="CustomChart".to_string() />
    </div>
</div>
```

## 🎯 **Real-World Integration Examples**

### **Example 1: Dashboard Application**

```rust
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="dashboard">
            <nav class="sidebar">
                <h3>"Component Library"</h3>
                <BundleAnalysisDisplay />
            </nav>
            
            <main class="content">
                <h1>"Dashboard"</h1>
                
                <div class="widgets">
                    <LazyComponentWrapper name="DataTable".to_string() />
                    <LazyComponentWrapper name="Chart".to_string() />
                    <LazyComponentWrapper name="Metrics".to_string() />
                </div>
            </main>
        </div>
    }
}
```

### **Example 2: Component Playground**

```rust
#[component]
pub fn ComponentPlayground() -> impl IntoView {
    view! {
        <div class="playground">
            <header class="playground-header">
                <h1>"Component Playground"</h1>
                <BundleStatusDisplay />
            </header>
            
            <div class="playground-content">
                <div class="component-categories">
                    <div class="category" data-category="form">
                        <h4>"Form Components"</h4>
                        <div class="lazy-grid">
                            <LazyComponentWrapper name="Input".to_string() />
                            <LazyComponentWrapper name="Select".to_string() />
                            <LazyComponentWrapper name="Checkbox".to_string() />
                        </div>
                    </div>
                    
                    <div class="category" data-category="layout">
                        <h4>"Layout Components"</h4>
                        <div class="lazy-grid">
                            <LazyComponentWrapper name="Card".to_string() />
                            <LazyComponentWrapper name="Grid".to_string() />
                            <LazyComponentWrapper name="Container".to_string() />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
```

## 🔧 **Advanced Customization**

### **Custom Loading States:**

```rust
// You can customize the loading behavior
let custom_load_component = move |_| {
    set_is_loading.set(true);
    
    // Your custom loading logic here
    spawn_local(async move {
        // Simulate API call or actual component loading
        gloo_timers::future::TimeoutFuture::new(2000).await;
        
        // Load your actual component
        // load_my_component().await;
        
        set_is_loading.set(false);
        set_is_loaded.set(true);
    });
};
```

### **Custom Styling:**

```css
/* In your project's CSS, you can override or extend the styles */
.my-custom-theme .lazy-component-wrapper {
    border-color: #8b5cf6;
    background: linear-gradient(135deg, #f3e8ff 0%, #e9d5ff 100%);
}

.my-custom-theme .component-category {
    background: #8b5cf6;
}
```

## 📦 **Distribution Options**

### **Option 1: Local Development (Recommended for testing)**
- Copy the entire folder to your workspace
- Use path dependencies in Cargo.toml
- Modify and customize as needed

### **Option 2: Git Submodule**
```bash
# Add as a submodule in your project
git submodule add https://github.com/your-username/enhanced-lazy-loading-demo.git
```

### **Option 3: Copy Specific Files**
- Copy only the components you need
- Copy the CSS file
- Integrate into your existing project structure

## 🎉 **Benefits of This Approach**

✅ **Full Control**: Modify and customize as needed  
✅ **No External Dependencies**: Everything runs locally  
✅ **Easy Testing**: Test in your actual project environment  
✅ **Customizable**: Adapt to your specific needs  
✅ **Professional Quality**: Use the enhanced UI system  
✅ **Scalable**: Easy to add more components  

---

**Now you can use this professional-grade lazy loading system in your own web apps!** 🚀✨
