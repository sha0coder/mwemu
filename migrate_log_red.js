#!/usr/bin/env node

const fs = require('fs').promises;
const path = require('path');

async function findRustFiles(dir) {
    const { glob } = await import('glob');
    const files = await glob(path.join(dir, '**/*.rs'), {
        ignore: ['**/target/**', '**/node_modules/**']
    });
    return files;
}

function extractLogInfo(content, startIdx) {
    let depth = 0;
    let foundStart = false;
    let endIdx = startIdx;
    
    for (let i = startIdx; i < content.length; i++) {
        if (content[i] === '(') {
            if (!foundStart) foundStart = true;
            depth++;
        } else if (content[i] === ')') {
            depth--;
            if (depth === 0 && foundStart) {
                endIdx = i + 1;
                break;
            }
        }
    }
    
    if (endIdx === startIdx) return null;
    
    return content.substring(startIdx, endIdx);
}

function parseLogArgs(logCall) {
    // Extract the content between log::info!( and the final )
    const match = logCall.match(/log::info!\s*\(([\s\S]*)\)$/);
    if (!match) return null;
    
    const argsContent = match[1];
    
    // Split by commas but respect nested structures
    const args = [];
    let currentArg = '';
    let depth = 0;
    let inString = false;
    let escaped = false;
    
    for (let i = 0; i < argsContent.length; i++) {
        const char = argsContent[i];
        
        if (escaped) {
            currentArg += char;
            escaped = false;
            continue;
        }
        
        if (char === '\\') {
            escaped = true;
            currentArg += char;
            continue;
        }
        
        if (char === '"' && !inString) {
            inString = true;
        } else if (char === '"' && inString) {
            inString = false;
        }
        
        if (!inString) {
            if (char === '(' || char === '{' || char === '[') {
                depth++;
            } else if (char === ')' || char === '}' || char === ']') {
                depth--;
            } else if (char === ',' && depth === 0) {
                args.push(currentArg.trim());
                currentArg = '';
                continue;
            }
        }
        
        currentArg += char;
    }
    
    if (currentArg.trim()) {
        args.push(currentArg.trim());
    }
    
    return args;
}

function migrateLogToLogRed(content) {
    let modified = false;
    let result = content;
    
    // Pattern to find log::info! with emu.colors.light_red
    const pattern = /log::info!\s*\(/g;
    let match;
    let replacements = [];
    
    while ((match = pattern.exec(content)) !== null) {
        const startIdx = match.index;
        const logCall = extractLogInfo(content, startIdx);
        
        if (!logCall) continue;
        
        // Check if this log::info! contains emu.colors.light_red and emu.pos
        if (logCall.includes('emu.colors.light_red') && logCall.includes('emu.pos')) {
            const args = parseLogArgs(logCall);
            if (!args || args.length < 3) continue;
            
            // First arg should be the format string
            let formatStr = args[0];
            
            // Find indices of color args
            let colorStartIdx = -1;
            let posIdx = -1;
            let colorEndIdx = -1;
            
            for (let i = 1; i < args.length; i++) {
                if (args[i].includes('emu.colors.light_red')) {
                    colorStartIdx = i;
                } else if (args[i].includes('emu.pos')) {
                    posIdx = i;
                } else if (args[i].includes('emu.colors.nc')) {
                    colorEndIdx = i;
                    break;
                }
            }
            
            // Extract the meaningful arguments (skip color and pos args)
            const meaningfulArgs = [];
            for (let i = 1; i < args.length; i++) {
                if (i !== colorStartIdx && i !== posIdx && i !== colorEndIdx) {
                    meaningfulArgs.push(args[i]);
                }
            }
            
            // Clean up the format string
            // Remove leading {}** {} pattern
            let cleanFormat = formatStr.replace(/^"?\s*\{\}\s*\*\*\s*\{\}\s*/, '"');
            
            // Remove trailing {} for color reset
            cleanFormat = cleanFormat.replace(/\s*\{\}\s*"?\s*$/, '"');
            
            // Ensure proper quoting
            if (!cleanFormat.startsWith('"')) {
                cleanFormat = '"' + cleanFormat;
            }
            if (!cleanFormat.endsWith('"')) {
                cleanFormat = cleanFormat + '"';
            }
            
            // Build the new log_red! call
            let newCall = 'log_red!(\n        emu,\n        ' + cleanFormat;
            
            if (meaningfulArgs.length > 0) {
                newCall += ',\n        ' + meaningfulArgs.join(',\n        ');
            }
            
            newCall += '\n    )';
            
            replacements.push({
                start: startIdx,
                end: startIdx + logCall.length,
                oldText: logCall,
                newText: newCall
            });
            
            modified = true;
        }
    }
    
    // Apply replacements in reverse order to maintain positions
    replacements.reverse();
    for (const replacement of replacements) {
        result = result.substring(0, replacement.start) + 
                 replacement.newText + 
                 result.substring(replacement.end);
        console.log(`  Migrated log::info! at position ${replacement.start}`);
    }
    
    return { content: result, modified };
}

async function processFile(filePath) {
    try {
        const content = await fs.readFile(filePath, 'utf8');
        const { content: newContent, modified } = migrateLogToLogRed(content);
        
        if (modified) {
            // Create backup
            //await fs.writeFile(filePath + '.bak', content, 'utf8');
            await fs.writeFile(filePath, newContent, 'utf8');
            console.log(`✓ Modified: ${filePath} (backup: ${filePath}.bak)`);
            return true;
        }
        
        return false;
    } catch (error) {
        console.error(`Error processing ${filePath}:`, error.message);
        return false;
    }
}

async function main() {
    const projectRoot = process.argv[2] || '.';
    
    console.log(`Searching for Rust files in: ${projectRoot}`);
    
    try {
        const files = await findRustFiles(projectRoot);
        console.log(`Found ${files.length} Rust files`);
        
        let modifiedCount = 0;
        
        for (const file of files) {
            const wasModified = await processFile(file);
            if (wasModified) {
                modifiedCount++;
            }
        }
        
        console.log(`\nMigration complete!`);
        console.log(`Modified ${modifiedCount} files`);
        console.log(`\nBackup files created with .bak extension`);
        console.log(`To remove backups: find ${projectRoot} -name "*.rs.bak" -delete`);
        
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run main
main().catch(console.error);