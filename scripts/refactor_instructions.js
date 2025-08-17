const fs = require('fs');
const path = require('path');

// Configuration
const CONFIG = {
    // Source file containing the giant match statement
    sourceFile: 'crates/libmwemu/src/engine/mod.rs.bak',
    
    // Destination directory for extracted instructions
    destDir: 'crates/libmwemu/src/engine/instructions/',
    
    // Generate trait implementation
    generateTraitImpl: true,
    traitName: 'InstructionHandler',
    
    // Instructions to extract for proof of concept
    // Set to null to extract all instructions
    testInstructions: null,  // Extract ALL instructions
    
    // Generate imports for each instruction file
    generateImports: function(mnemonic) {
        return `use crate::emu::Emu;
use crate::console::Console;
use crate::{color, get_bit, set_bit, to32};
use iced_x86::{Instruction, Mnemonic, Register};

`;
    }
};

// Helper function to convert PascalCase to snake_case
function pascalToSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase().replace(/^_/, '');
}

// Parse match arms from the source file
function extractMatchArms() {
    const content = fs.readFileSync(CONFIG.sourceFile, 'utf8');
    const lines = content.split('\n');
    
    const instructions = new Map();
    let currentMnemonic = null;
    let currentBody = [];
    let braceCount = 0;
    let inMatchStatement = false;
    let matchStarted = false;
    
    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();
        
        // Find the start of the match statement
        if (!matchStarted && trimmed.includes('match ins.mnemonic()')) {
            matchStarted = true;
            console.log(`Found match statement at line ${i + 1}`);
            continue;
        }
        
        if (!matchStarted) continue;
        
        // Check for match arm start (can have multiple mnemonics with |)
        const matchArmMatch = trimmed.match(/^Mnemonic::(\w+)(?:\s*\|\s*Mnemonic::(\w+))*\s*=>\s*\{?/);
        if (matchArmMatch) {
            // Save previous instruction if exists
            if (currentMnemonic && currentBody.length > 0) {
                // Save for each mnemonic in the previous group
                if (Array.isArray(currentMnemonic)) {
                    currentMnemonic.forEach(m => instructions.set(m, currentBody.join('\n')));
                } else {
                    instructions.set(currentMnemonic, currentBody.join('\n'));
                }
            }
            
            // Extract all mnemonics from this match arm
            const mnemonics = [];
            const fullMatch = trimmed.match(/Mnemonic::(\w+)/g);
            if (fullMatch) {
                fullMatch.forEach(m => {
                    const name = m.replace('Mnemonic::', '');
                    mnemonics.push(name);
                });
            }
            console.log(`Found mnemonics at line ${i + 1}: ${mnemonics.join(', ')}`)
            
            // Filter based on test instructions if specified
            let relevantMnemonics = mnemonics;
            if (CONFIG.testInstructions) {
                relevantMnemonics = mnemonics.filter(m => CONFIG.testInstructions.includes(m));
                if (relevantMnemonics.length === 0) {
                    currentMnemonic = null;
                    currentBody = [];
                    continue;
                }
            }
            
            currentMnemonic = relevantMnemonics.length === 1 ? relevantMnemonics[0] : relevantMnemonics;
            currentBody = [];
            braceCount = 0;
            inMatchStatement = true;
            
            // Check if the brace is on the same line
            if (trimmed.includes('=>') && trimmed.includes('{')) {
                braceCount = 1;
                // If there's content after the brace on the same line, include it
                const afterBrace = line.substring(line.indexOf('{') + 1).trim();
                if (afterBrace && afterBrace !== '}') {
                    currentBody.push('            ' + afterBrace);
                }
            }
            continue;
        }
        
        // If we're inside a match arm, collect the body
        if (inMatchStatement && currentMnemonic) {
            // Count braces
            for (const char of line) {
                if (char === '{') braceCount++;
                else if (char === '}') {
                    braceCount--;
                    if (braceCount === 0) {
                        // End of this match arm - save it
                        if (Array.isArray(currentMnemonic)) {
                            currentMnemonic.forEach(m => instructions.set(m, currentBody.join('\n')));
                        } else {
                            instructions.set(currentMnemonic, currentBody.join('\n'));
                        }
                        currentMnemonic = null;
                        currentBody = [];
                        inMatchStatement = false;
                        break;
                    }
                }
            }
            
            // Add the line to the current body (unless it's the closing brace that ends the arm)
            if (inMatchStatement) {
                currentBody.push(line);
            }
        }
        
        // Don't break on just any closing brace - we're still in the match statement
    }
    
    // Save the last instruction if the file ended
    if (currentMnemonic && currentBody.length > 0) {
        if (Array.isArray(currentMnemonic)) {
            currentMnemonic.forEach(m => instructions.set(m, currentBody.join('\n')));
        } else {
            instructions.set(currentMnemonic, currentBody.join('\n'));
        }
    }
    
    return instructions;
}

// Generate a single instruction file
function generateInstructionFile(mnemonic, body) {
    const filename = pascalToSnakeCase(mnemonic) + '.rs';
    const filepath = path.join(CONFIG.destDir, filename);
    
    let content = CONFIG.generateImports(mnemonic);
    
    // Process the body - remove excessive indentation
    const bodyLines = body.split('\n');
    
    // Find the minimum indentation (excluding empty lines)
    let minIndent = Infinity;
    bodyLines.forEach(line => {
        if (line.trim().length > 0) {
            const leadingSpaces = line.match(/^(\s*)/)[1].length;
            minIndent = Math.min(minIndent, leadingSpaces);
        }
    });
    
    // Remove the minimum indentation from all lines
    const dedentedLines = bodyLines.map(line => {
        if (line.trim().length === 0) return '';
        return line.substring(minIndent);
    });
    
    let processedBody = dedentedLines.join('\n').trim();
    
    // Check if the body ends with a return statement
    const lastNonEmptyLine = dedentedLines.filter(l => l.trim()).pop()?.trim() || '';
    const hasExplicitReturn = lastNonEmptyLine.includes('return');
    
    // Add proper indentation (4 spaces) to each line
    const indentedBody = processedBody.split('\n').map(line => 
        line.trim() ? '    ' + line : ''
    ).join('\n');
    
    // Add the execute function
    if (!hasExplicitReturn) {
        // Add default return true if no explicit return
        content += `pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
${indentedBody}
    true
}
`;
    } else {
        content += `pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
${indentedBody}
}
`;
    }
    
    fs.writeFileSync(filepath, content);
    console.log(`Created: ${filepath}`);
    return filename.replace('.rs', '');
}

// Generate mod.rs file for instructions directory
function generateModFile(modules) {
    const modPath = path.join(CONFIG.destDir, 'mod.rs');
    
    let content = '// Auto-generated module declarations for instructions\n\n';
    
    // Add module declarations
    modules.forEach(module => {
        content += `pub mod ${module};\n`;
    });
    
    fs.writeFileSync(modPath, content);
    console.log(`Created: ${modPath}`);
}

// Generate the updated main mod.rs with the new dispatcher
function generateDispatcher(mnemonics) {
    const dispatcherPath = 'crates/libmwemu/src/engine/mod_dispatcher.rs';
    
    let content = `// Dispatcher for instruction execution
use crate::emu::Emu;
use iced_x86::{Instruction, Mnemonic};

pub mod instructions;

pub fn emulate_instruction(
    emu: &mut Emu,
    ins: &Instruction,
    instruction_sz: usize,
    rep_step: bool,
) -> bool {
    match ins.mnemonic() {
`;
    
    mnemonics.forEach(mnemonic => {
        const module = pascalToSnakeCase(mnemonic);
        content += `        Mnemonic::${mnemonic} => instructions::${module}::execute(emu, ins, instruction_sz, rep_step),\n`;
    });
    
    content += `        _ => {
            log::info!("Unimplemented instruction: {:?}", ins.mnemonic());
            false
        }
    }
}
`;
    
    fs.writeFileSync(dispatcherPath, content);
    console.log(`Created: ${dispatcherPath}`);
}

// Main extraction function
function main() {
    console.log('🚀 Starting instruction extraction...');
    console.log(`Source: ${CONFIG.sourceFile}`);
    console.log(`Destination: ${CONFIG.destDir}`);
    
    // Create destination directory
    if (!fs.existsSync(CONFIG.destDir)) {
        fs.mkdirSync(CONFIG.destDir, { recursive: true });
    }
    
    // Extract match arms
    const instructions = extractMatchArms();
    console.log(`\nFound ${instructions.size} instructions to extract`);
    
    if (instructions.size === 0) {
        console.error('❌ No instructions found. Check the source file and pattern.');
        return;
    }
    
    // Generate individual instruction files
    const modules = [];
    for (const [mnemonic, body] of instructions) {
        console.log(`\nProcessing: ${mnemonic}`);
        const module = generateInstructionFile(mnemonic, body);
        modules.push(module);
    }
    
    // Generate mod.rs
    generateModFile(modules);
    
    // Generate dispatcher
    generateDispatcher(Array.from(instructions.keys()));
    
    console.log('\n✅ Instruction extraction completed successfully!');
    console.log(`\nExtracted ${instructions.size} instructions:`);
    instructions.forEach((_, mnemonic) => {
        console.log(`  - ${mnemonic}`);
    });
    
    console.log('\nNext steps:');
    console.log('1. Review the generated files in', CONFIG.destDir);
    console.log('2. Check the dispatcher in mod_dispatcher.rs');
    console.log('3. Update the main mod.rs to use the new structure');
    console.log('4. Run tests to verify everything works');
}

// Run the extraction
try {
    main();
} catch (error) {
    console.error('❌ Error during extraction:', error);
    process.exit(1);
}