document.addEventListener('DOMContentLoaded', function() {
    // SVG for clipboard icon (borrowed from github.com)
    const clipboardSvg = `
      <svg aria-hidden="true" focusable="false" viewBox="0 0 16 16" width="16" height="16" fill="currentColor" display="inline-block" overflow="visible" style="vertical-align:text-bottom">
        <path d="M0 6.75C0 5.784.784 5 1.75 5h1.5a.75.75 0 0 1 0 1.5h-1.5a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-1.5a.75.75 0 0 1 1.5 0v1.5A1.75 1.75 0 0 1 9.25 16h-7.5A1.75 1.75 0 0 1 0 14.25Z"></path>
        <path d="M5 1.75C5 .784 5.784 0 6.75 0h7.5C15.216 0 16 .784 16 1.75v7.5A1.75 1.75 0 0 1 14.25 11h-7.5A1.75 1.75 0 0 1 5 9.25Zm1.75-.25a.25.25 0 0 0-.25.25v7.5c0 .138.112.25.25.25h7.5a.25.25 0 0 0 .25-.25v-7.5a.25.25 0 0 0-.25-.25Z"></path>
      </svg>
    `;
  
    // Find all code blocks
    const codeBlocks = document.querySelectorAll('.highlight');
    
    codeBlocks.forEach(block => {
      // Create the copy button
      const button = document.createElement('button');
      button.className = 'copy-button';
      button.innerHTML = clipboardSvg + 'Copy';
      button.type = 'button';
      
      // Add the button to the code block
      block.appendChild(button);
      
      // Add click event
      button.addEventListener('click', async () => {
        const code = block.querySelector('pre').textContent;
        
        try {
          await navigator.clipboard.writeText(code);
          button.classList.add('copied');
          
          // Remove the class after animation completes
          setTimeout(() => {
            button.classList.remove('copied');
          }, 2000);
        } catch (err) {
          console.error('Failed to copy: ', err);
        }
      });
    });
  });