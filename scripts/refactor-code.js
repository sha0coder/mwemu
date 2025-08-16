const fs = require('fs');
const path = require('path');

// Helper function to convert CamelCase to snake_case
function camelToSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase().replace(/^_/, '');
}

// Helper function to extract function name from function declaration
function extractFunctionName(line) {
    const match = line.match(/^\s*fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
    return match ? match[1] : null;
}

// Helper function to generate the imports for each file
function generateImports() {
    return `use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

`;
}

// Main extraction function
function extractFunctions() {
    const content = fs.readFileSync('crates/libmwemu/src/winapi/winapi64/kernel32.rs.bak', 'utf8');
    const lines = content.split('\n');
    
    // Create output directory
    if (!fs.existsSync('crates/libmwemu/src/winapi/winapi64/kernel32')) {
        fs.mkdirSync('crates/libmwemu/src/winapi/winapi64/kernel32', { recursive: true });
    }
    
    let currentFunction = [];
    let functionName = '';
    let inFunction = false;
    let braceCount = 0;
    let functionsExtracted = [];
    let remainingContent = [];
    let skipMode = false;
    
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();
        
        // Check if this is a function definition (but not gateway or helper functions we want to keep)
        const extractedName = extractFunctionName(trimmed);
        if (extractedName && !['gateway', 'dump_module_iat', 'resolve_api_addr_to_name', 
            'resolve_api_name', 'search_api_name', 'guess_api_name', 'load_library', 
            'get_library_handle', 'clear_last_error', 'advance_tick'].includes(extractedName)) {
            
            if (inFunction && currentFunction.length > 0) {
                // Save previous function
                saveFunction(functionName, currentFunction);
                functionsExtracted.push(functionName);
            }
            
            // Start new function
            functionName = extractedName;
            currentFunction = [line];
            inFunction = true;
            braceCount = 0;
            skipMode = true;
            
            // Count opening braces in the first line
            for (const char of line) {
                if (char === '{') braceCount++;
                else if (char === '}') braceCount--;
            }
            
            continue;
        }
        
        if (inFunction) {
            currentFunction.push(line);
            
            // Count braces to determine when function ends
            for (const char of line) {
                if (char === '{') {
                    braceCount++;
                } else if (char === '}') {
                    braceCount--;
                    if (braceCount === 0) {
                        // Function ended
                        saveFunction(functionName, currentFunction);
                        functionsExtracted.push(functionName);
                        inFunction = false;
                        skipMode = false;
                        currentFunction = [];
                        break;
                    }
                }
            }
        } else if (!skipMode) {
            // Keep non-function lines for the main file
            remainingContent.push(line);
        } else {
            skipMode = false;
        }
    }
    
    // Handle case where file ends while in a function
    if (inFunction && currentFunction.length > 0) {
        saveFunction(functionName, currentFunction);
        functionsExtracted.push(functionName);
    }
    
    // Generate mod.rs
    generateModFile(functionsExtracted);
    
    // Generate updated main file with imports
    generateUpdatedMainFile(remainingContent, functionsExtracted);
    
    console.log(`Extracted ${functionsExtracted.length} functions:`);
    functionsExtracted.forEach(name => {
        console.log(`  - ${name} -> kernel32/${camelToSnakeCase(name)}.rs`);
    });
}

function saveFunction(name, lines) {
    if (!name || lines.length === 0) return;
    
    const snakeCaseName = camelToSnakeCase(name);
    const filename = path.join('crates/libmwemu/src/winapi/winapi64/kernel32', `${snakeCaseName}.rs`);
    
    let content = generateImports();
    
    // Make function public and add content
    const functionContent = lines.join('\n').replace(
        new RegExp(`^(\\s*)fn\\s+${name}`, 'm'), 
        '$1pub fn ' + name
    );
    
    content += functionContent;
    
    fs.writeFileSync(filename, content);
    console.log(`Created: ${filename}`);
}

function generateModFile(functionNames) {
    let modContent = '// Auto-generated module declarations\n\n';
    
    functionNames.forEach(name => {
        const snakeCaseName = camelToSnakeCase(name);
        modContent += `pub mod ${snakeCaseName};\n`;
    });
    
    modContent += '\n// Re-export all functions\n';
    functionNames.forEach(name => {
        const snakeCaseName = camelToSnakeCase(name);
        modContent += `pub use ${snakeCaseName}::${name};\n`;
    });
    
    //fs.writeFileSync('crates/libmwemu/src/winapi/winapi64/kernel32/mod.rs', modContent);
    console.log('Created: kernel32/mod.rs');
}

function generateUpdatedMainFile(remainingLines, extractedFunctions) {
    let content = remainingLines.join('\n');
    
    // Add import for the kernel32 module at the top
    const imports = `mod kernel32;\nuse kernel32::*;\n\n`;
    content = imports + content;
    
    // Update the gateway function to remove the extracted functions from the match
    content = content.replace(
        /match api\.as_str\(\) \{([\s\S]*?)\}/,
        (match, matchContent) => {
            const lines = matchContent.split('\n');
            const filteredLines = lines.filter(line => {
                const trimmed = line.trim();
                // Keep lines that don't match our extracted functions
                const isExtractedFunction = extractedFunctions.some(funcName => 
                    trimmed.includes(`"${funcName}"`) && trimmed.includes(`${funcName}(emu)`)
                );
                return !isExtractedFunction;
            });
            return `match api.as_str() {${filteredLines.join('\n')}}`;
        }
    );
    
    fs.writeFileSync('main_updated.rs', content);
    console.log('Created: main_updated.rs (updated main file)');
}

// Run the extraction
try {
    extractFunctions();
    console.log('\n✅ Function extraction completed successfully!');
    console.log('\nNext steps:');
    console.log('1. Review the generated files');
    console.log('2. Adjust imports in individual files if needed');
    console.log('3. Replace your original file with main_updated.rs');
    console.log('4. Add "mod kernel32;" to your main.rs or lib.rs');
} catch (error) {
    console.error('❌ Error during extraction:', error);
}