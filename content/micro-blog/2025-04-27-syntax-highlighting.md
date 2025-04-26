---
title: "Syntax Highlighting Test"
date: 2025-04-26
tags: ["code", "hugo", "css"]
draft: true
---

This post demonstrates the automatic syntax highlighting that adapts between light and dark mode.

## Python Code

```python
def factorial(n):
    """Calculate the factorial of a number recursively"""
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)

# Test our function
for i in range(5):
    print(f"{i}! = {factorial(i)}")
```

## JavaScript Code

```javascript
// Function to calculate fibonacci numbers
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n-1) + fibonacci(n-2);
}

// Generate the first 10 fibonacci numbers
const results = [];
for (let i = 0; i < 10; i++) {
  results.push(fibonacci(i));
}

console.log("Fibonacci sequence:", results);
```

## CSS Code

```css
/* Main container styling */
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  body {
    background-color: #121212;
    color: #e0e0e0;
  }
  
  a {
    color: #90caf9;
  }
}
```

## HTML Code

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <header>
    <h1>Hello World</h1>
  </header>
  <main>
    <p>This is a <strong>sample</strong> HTML document.</p>
  </main>
</body>
</html>
```

## Inline Code

You can also use `inline code` within paragraphs. For example, if you want to reference a variable like `fibonacci(n)` or a CSS class like `.container`.