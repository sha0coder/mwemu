#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execa } = require('execa');

class CargoCheckFixer {
    constructor() {
        this.warnings = [];
    }

    // Parse cargo check output to extract unused import warnings
    parseCargoOutput(output) {
        console.log('DEBUG: Starting to parse output...');
        console.log('DEBUG: Raw output length:', output.length);
        console.log('DEBUG: First 500 chars:', JSON.stringify(output.substring(0, 500)));
        
        const lines = output.split('\n');
        console.log(`DEBUG: Total lines: ${lines.length}`);
        
        // Show first few lines for debugging
        console.log('DEBUG: First 10 lines:');
        lines.slice(0, 10).forEach((line, i) => {
            console.log(`  ${i}: "${line}"`);
        });
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            
            // More flexible matching for unused import warnings
            const unusedImportSingle = line.match(/warning: unused import:?\s*(.+)/);
            const unusedImportMultiple = line.match(/warning: unused imports:?\s*(.+)/);
            
            if (unusedImportSingle || unusedImportMultiple) {
                console.log(`DEBUG: Found warning line ${i}: "${line}"`);
                
                // Extract the import names from the warning line
                let imports = [];
                
                if (unusedImportMultiple) {
                    // Handle: "warning: unused imports: `SystemTime` and `UNIX_EPOCH`"
                    console.log(`DEBUG: Multiple imports match:`, unusedImportMultiple);
                    imports = this.parseUnusedImports(unusedImportMultiple[1]);
                    console.log(`DEBUG: Parsed imports:`, imports);
                } else if (unusedImportSingle) {
                    // Handle: "warning: unused import: `std::sync::Mutex`"
                    console.log(`DEBUG: Single import match:`, unusedImportSingle);
                    imports = this.parseUnusedImports(unusedImportSingle[1]);
                    console.log(`DEBUG: Parsed imports:`, imports);
                }
                
                if (imports.length > 0) {
                    console.log(`DEBUG: Looking for file path after line ${i}...`);
                    // Look for file path in next few lines
                    for (let j = i + 1; j < Math.min(i + 10, lines.length); j++) {
                        const nextLine = lines[j];
                        console.log(`DEBUG: Checking line ${j}: "${nextLine}"`);
                        
                        // More flexible path matching
                        const pathMatch = nextLine.match(/^\s*-->\s*(.+):(\d+):(\d+)/) || 
                                         nextLine.match(/^\s*(.+\.rs):(\d+):(\d+)/);
                        
                        if (pathMatch) {
                            console.log(`DEBUG: Found path match:`, pathMatch);
                            const currentWarning = {
                                file: pathMatch[1],
                                line: parseInt(pathMatch[2]),
                                column: parseInt(pathMatch[3]),
                                imports: imports,
                                rawLine: null
                            };
                            
                            // Look for the actual import line in the cargo output
                            for (let k = j + 1; k < Math.min(j + 15, lines.length); k++) {
                                const codeLine = lines[k];
                                console.log(`DEBUG: Checking code line ${k}: "${codeLine}"`);
                                
                                // Look for lines that show the actual code
                                const codeMatch = codeLine.match(/^\s*\d+\s*\|\s*(.+)/) ||
                                                 codeLine.match(/^\s*\d+\s+\|\s+(.+)/);
                                
                                if (codeMatch) {
                                    console.log(`DEBUG: Found code line:`, codeMatch[1]);
                                    currentWarning.rawLine = codeMatch[1].trim();
                                    break;
                                }
                            }
                            
                            this.warnings.push(currentWarning);
                            console.log(`DEBUG: Added warning - File: ${currentWarning.file}, Line: ${currentWarning.line}, Imports: ${imports.join(', ')}, RawLine: ${currentWarning.rawLine}`);
                            break;
                        }
                    }
                } else {
                    console.log(`DEBUG: No imports found for line: "${line}"`);
                }
            }
        }
        console.log(`DEBUG: Total warnings found: ${this.warnings.length}`);
        
        // If we found no warnings, let's check if we're missing any patterns
        if (this.warnings.length === 0) {
            console.log('DEBUG: No warnings found, checking for any lines containing "unused"...');
            lines.forEach((line, i) => {
                if (line.toLowerCase().includes('unused')) {
                    console.log(`DEBUG: Line ${i} contains "unused": "${line}"`);
                }
            });
        }
    }

    // Parse the unused imports from the warning message
    parseUnusedImports(importString) {
        console.log(`DEBUG: Parsing import string: "${importString}"`);
        
        // Handle both single and multiple imports
        // Examples: "`SystemTime` and `UNIX_EPOCH`" or "`std::sync::Mutex`"
        const imports = [];
        
        // Extract all backtick-quoted imports
        const matches = importString.match(/`([^`]+)`/g);
        console.log(`DEBUG: Backtick matches:`, matches);
        
        if (matches) {
            matches.forEach(match => {
                const importName = match.slice(1, -1); // Remove backticks
                imports.push(importName);
            });
        }
        
        console.log(`DEBUG: Final parsed imports:`, imports);
        return imports;
    }

    // Fix a specific warning by removing unused imports
    fixWarning(warning) {
        try {
            const filePath = warning.file;
            
            if (!fs.existsSync(filePath)) {
                console.log(`Error: File ${filePath} doesn't exist`);
                return false;
            }
            
            const fileContent = fs.readFileSync(filePath, 'utf8');
            const lines = fileContent.split('\n');
            
            if (warning.line > lines.length) {
                console.log(`Error: Line ${warning.line} doesn't exist in ${filePath} (file has ${lines.length} lines)`);
                return false;
            }

            const originalLine = lines[warning.line - 1];
            console.log(`Fixing ${filePath}:${warning.line} - ${warning.imports.join(', ')}`);

            const fixedLine = this.removeUnusedImports(originalLine, warning.imports);
            
            if (fixedLine === null) {
                lines.splice(warning.line - 1, 1);
                fs.writeFileSync(filePath, lines.join('\n'));
                console.log('✓ Removed entire import line');
                return true;
            } else if (fixedLine !== originalLine) {
                lines[warning.line - 1] = fixedLine;
                fs.writeFileSync(filePath, lines.join('\n'));
                console.log('✓ Fixed import line');
                return true;
            } else {
                console.log('- No changes needed');
                return false;
            }
        } catch (error) {
            console.error(`Error fixing ${warning.file}:`, error.message);
            return false;
        }
    }

    // Remove unused imports from a use statement
    removeUnusedImports(line, unusedImports) {
        console.log(`DEBUG: removeUnusedImports called with line: "${line}", unused: [${unusedImports.join(', ')}]`);
        
        let modifiedLine = line;

        // Handle different import patterns
        if (line.includes('{') && line.includes('}')) {
            // Handle: use std::time::{SystemTime, UNIX_EPOCH};
            const match = line.match(/^(\s*use\s+[^{]*\{)([^}]+)(\}[^}]*;?\s*)$/);
            if (match) {
                const prefix = match[1];
                const imports = match[2];
                const suffix = match[3];
                
                console.log(`DEBUG: Matched braced import - prefix: "${prefix}", imports: "${imports}", suffix: "${suffix}"`);
                
                const importList = imports.split(',').map(imp => imp.trim());
                console.log(`DEBUG: Import list:`, importList);
                
                const filteredImports = importList.filter(imp => {
                    const shouldKeep = !unusedImports.some(unused => {
                        // More flexible matching - check if the import contains the unused name
                        return imp.includes(unused) || unused.includes(imp.replace(/^.*::/, ''));
                    });
                    console.log(`DEBUG: Import "${imp}" should be kept: ${shouldKeep}`);
                    return shouldKeep;
                });

                console.log(`DEBUG: Filtered imports:`, filteredImports);

                if (filteredImports.length === 0) {
                    console.log(`DEBUG: All imports removed, returning null`);
                    return null; // Remove entire line
                } else if (filteredImports.length < importList.length) {
                    const result = prefix + filteredImports.join(', ') + suffix;
                    console.log(`DEBUG: Some imports removed, returning: "${result}"`);
                    return result;
                }
            }
        } else {
            // Handle: use std::sync::Mutex;
            console.log(`DEBUG: Checking single import line`);
            const isEntireImportUnused = unusedImports.some(unused => {
                const matches = line.includes(unused) && 
                    (line.startsWith(`use ${unused}`) || 
                     line.includes(`::${unused}`) ||
                     line.match(new RegExp(`use\\s+.*${unused.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`)));
                console.log(`DEBUG: Checking if "${unused}" makes entire line unused: ${matches}`);
                return matches;
            });
            
            if (isEntireImportUnused) {
                console.log(`DEBUG: Entire import line is unused, returning null`);
                return null; // Remove entire line
            }
        }

        console.log(`DEBUG: No changes needed, returning original line`);
        return modifiedLine;
    }



    // Run cargo check and parse output
    async runCargoCheck() {
        console.log('Running cargo check...');
        
        const result = await execa('cargo', ['check', '--color', 'never'], {
            reject: false // Don't throw on non-zero exit codes
        });
        
        const output = result.stdout + result.stderr;
        console.log('DEBUG: Output length:', output.length);
        return output;
    }

    // Main execution function
    async run() {
        console.log('🦀 Cargo Check Unused Imports Fixer\n');

        // Get cargo check output
        let cargoOutput;
        if (process.argv[2] === '--stdin') {
            // Read from stdin if --stdin flag is provided
            const chunks = [];
            for await (const chunk of process.stdin) {
                chunks.push(chunk);
            }
            cargoOutput = Buffer.concat(chunks).toString();
            console.log('DEBUG: Read from stdin, length:', cargoOutput.length);
        } else {
            cargoOutput = await this.runCargoCheck();
        }

        // Parse warnings
        this.parseCargoOutput(cargoOutput);

        if (this.warnings.length === 0) {
            console.log('✓ No unused import warnings found!');
            return;
        }

        console.log(`Found ${this.warnings.length} unused import warning(s)\n`);

        // Fix all warnings
        for (let i = 0; i < this.warnings.length; i++) {
            this.fixWarning(this.warnings[i]);
        }

        console.log('\n🎉 Done! Run cargo check again to verify fixes.');
    }
}

// Run the script
if (require.main === module) {
    const fixer = new CargoCheckFixer();
    fixer.run().catch(console.error);
}

module.exports = CargoCheckFixer;