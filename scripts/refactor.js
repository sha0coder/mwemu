const fs = require('fs');
const path = require('path');

// Configuration - easily change source/destination
const CONFIG = {
    // Source file to extract functions from
    sourceFile: 'crates/libmwemu/src/winapi/winapi32/kernel32.rs.bak',
    
    // Destination directory for extracted functions
    destDir: 'crates/libmwemu/src/winapi/winapi32/kernel32/',
    
    // Pattern to identify functions to extract (e.g., test functions)
    functionPattern: /#\[test\]/,

    functionNames: [
        "AddVectoredExceptionHandler",
        "AreFileApisANSI",
        "CloseHandle",
        "ConnectNamedPipe",
        "CopyFileA",
        "CopyFileW",
        "CreateEventA",
        "CreateFileMappingA",
        "CreateFileMappingW",
        "CreateFileW",
        "CreateMutexA",
        "CreateMutexW",
        "CreateNamedPipeA",
        "CreateProcessA",
        "CreateRemoteThread",
        "CreateThread",
        "CreateToolhelp32Snapshot",
        "CryptCreateHash",
        "DecodePointer",
        "DisconnectNamedPipe",
        "EncodePointer",
        "EnterCriticalSection",
        "ExitProcess",
        "ExpandEnvironmentStringsA",
        "ExpandEnvironmentStringsW",
        "FileTimeToDosDateTime",
        "FileTimeToLocalFileTime",
        "FileTimeToSystemTime",
        "FindClose",
        "FindFirstFileA",
        "FindFirstFileW",
        "FindNextFileA",
        "FindNextFileW",
        "FindResourceA",
        "FindResourceW",
        "FlsAlloc",
        "FlsGetValue",
        "FlsSetValue",
        "FreeLibrary",
        "FreeResource",
        "GetAcp",
        "GetACP",
        "GetCommandLineA",
        "GetCommandLineW",
        "GetComputerNameA",
        "GetCPInfo",
        "GetCurrentDirectoryA",
        "GetCurrentDirectoryW",
        "GetCurrentProcess",
        "GetCurrentProcessId",
        "GetCurrentThreadId",
        "GetEnvironmentStrings",
        "GetEnvironmentStringsW",
        "GetFileAttributesA",
        "GetFileAttributesW",
        "GetFileType",
        "GetFullPathNameA",
        "GetFullPathNameW",
        "GetLastError",
        "GetLogicalDrives",
        "GetLongPathNameW",
        "GetModuleFileNameA",
        "GetModuleFileNameW",
        "GetModuleHandleA",
        "GetModuleHandleW",
        "GetNativeSystemInfo",
        "GetOEMCP",
        "GetProcAddress",
        "GetProcessAffinityMask",
        "GetProcessHeap",
        "GetStartupInfoA",
        "GetStartupInfoW",
        "GetStdHandle",
        "GetStringTypeW",
        "GetSystemDirectoryA",
        "GetSystemDirectoryW",
        "GetSystemInfo",
        "GetSystemTime",
        "GetSystemTimeAsFileTime",
        "GetSystemWindowsDirectoryA",
        "GetSystemWindowsDirectoryW",
        "GetTempPathW",
        "GetThreadContext",
        "GetThreadPreferredUILanguages",
        "GetThreadUILanguage",
        "GetTickCount",
        "GetTimeZoneInformation",
        "GetUserDefaultLangID",
        "GetUserDefaultUILanguage",
        "GetVersion",
        "GetVersionExW",
        "GetWindowsDirectoryA",
        "GetWindowsDirectoryW",
        "HeapAlloc",
        "HeapCreate",
        "HeapDestroy",
        "HeapFree",
        "HeapSetInformation",
        "InitializeCriticalSection",
        "InitializeCriticalSectionAndSpinCount",
        "InitializeCriticalSectionEx",
        "InterlockedIncrement",
        "IsDebuggerPresent",
        "IsProcessorFeaturePresent",
        "IsValidCodePage",
        "IsValidLocale",
        "LCMapStringW",
        "LeaveCriticalSection",
        "LoadLibraryA",
        "LoadLibraryExA",
        "LoadLibraryExW",
        "LoadLibraryW",
        "LoadResource",
        "LocalAlloc",
        "LockResource",
        "lstrcat",
        "lstrcmp",
        "lstrcmpA",
        "lstrcmpW",
        "lstrcpy",
        "lstrlen",
        "MapViewOfFile",
        "MoveFileA",
        "MoveFileW",
        "MultiByteToWideChar",
        "OpenProcess",
        "OpenProcessToken",
        "OpenThread",
        "QueryPerformanceCounter",
        "RaiseException",
        "ReadFile",
        "ReadProcessMemory",
        "RegCloseKey",
        "RegCreateKeyExA",
        "RegCreateKeyExW",
        "RegOpenKeyA",
        "RegOpenKeyExW",
        "RegOpenKeyW",
        "RegSetValueExA",
        "RegSetValueExW",
        "ResumeThread",
        "SetErrorMode",
        "SetHandleCount",
        "SetLastError",
        "SetThreadContext",
        "SetThreadLocale",
        "SetUnhandledExceptionFilter",
        "SizeofResource",
        "Sleep",
        "SystemTimeToTzSpecificLocalTime",
        "TerminateProcess",
        "Thread32First",
        "Thread32Next",
        "TlsAlloc",
        "TlsFree",
        "TlsGetValue",
        "TlsSetValue",
        "UnhandledExceptionFilter",
        "VerifyVersionInfoW",
        "VirtualAlloc",
        "VirtualAllocEx",
        "VirtualAllocExNuma",
        "VirtualFree",
        "VirtualProtect",
        "VirtualProtectEx",
        "VirtualQuery",
        "VirtualQueryEx",
        "WaitForSingleObject",
        "WideCharToMultiByte",
        "WinExec",
        "WriteFile",
        "WriteProcessMemory",
    ],
    
    // Whether to convert function names to snake_case for filenames
    useSnakeCase: false,
    
    // Functions to skip extraction (keep in main file)
    skipFunctions: [],
    
    // Custom imports for each extracted file
    generateImports: function() {
        return `use crate::emu;

`;
    },
    
    // Whether to make extracted functions public
    makePublic: true,
    
    // Whether to generate a mod.rs file
    generateModFile: true
};

// Helper function to convert CamelCase to snake_case
function camelToSnakeCase(str) {
    return str.replace(/([A-Z])/g, '_$1').toLowerCase().replace(/^_/, '');
}

// Helper function to extract function name from function declaration
function extractFunctionName(line) {
    const match = line.match(/^\s*fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
    return match ? match[1] : null;
}

// Main extraction function
function extractFunctions() {
    const content = fs.readFileSync(CONFIG.sourceFile, 'utf8');
    const lines = content.split('\n');
    
    // Create output directory
    if (!fs.existsSync(CONFIG.destDir)) {
        fs.mkdirSync(CONFIG.destDir, { recursive: true });
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
        
        // Check if this is a function definition we want to extract
        const extractedName = extractFunctionName(trimmed);
        if (extractedName && CONFIG.functionNames.includes(extractedName) && !CONFIG.skipFunctions.includes(extractedName)) {
            
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
    
    // Generate mod.rs if configured
    if (CONFIG.generateModFile) {
        generateModFile(functionsExtracted);
    }
    
    // Generate updated main file with imports
    generateUpdatedMainFile(remainingContent, functionsExtracted);
    
    console.log(`\n✅ Extracted ${functionsExtracted.length} functions from ${CONFIG.sourceFile}:`);
    functionsExtracted.forEach(name => {
        const filename = CONFIG.useSnakeCase ? camelToSnakeCase(name) : name;
        console.log(`  - ${name} -> ${CONFIG.destDir}/${filename}.rs`);
    });
}

function saveFunction(name, lines) {
    if (!name || lines.length === 0) return;
    
    const filename = CONFIG.useSnakeCase ? camelToSnakeCase(name) : camelToSnakeCase(name);
    const filepath = path.join(CONFIG.destDir, `${filename}.rs`);
    
    // Check if file already exists and has content
    if (fs.existsSync(filepath)) {
        const existingContent = fs.readFileSync(filepath, 'utf8');
        if (existingContent.trim().length > 0) {
            console.log(`⚠️  Skipping ${filename}.rs - file already has content`);
            return;
        }
    }
    
    let content = CONFIG.generateImports();
    
    // Remove any leading indentation from the entire function
    // Find the minimum indentation (excluding empty lines)
    let minIndent = Infinity;
    for (const line of lines) {
        if (line.trim().length > 0) {
            const leadingSpaces = line.match(/^(\s*)/)[1].length;
            minIndent = Math.min(minIndent, leadingSpaces);
        }
    }
    
    // Remove the minimum indentation from all lines
    const dedentedLines = lines.map(line => {
        if (line.trim().length === 0) return '';
        return line.substring(minIndent);
    });
    
    // Add function content
    let functionContent = dedentedLines.join('\n');
    
    // Make function public if configured
    if (CONFIG.makePublic) {
        functionContent = functionContent.replace(
            new RegExp(`^(\\s*)fn\\s+${name}`, 'm'), 
            'pub fn ' + name
        );
    }
    
    content += functionContent;
    
    fs.writeFileSync(filepath, content);
    console.log(`Created: ${filepath}`);
}

function generateModFile(functionNames) {
    const modPath = path.join(CONFIG.destDir, 'mod.rs');
    
    // Check if mod.rs already exists
    if (fs.existsSync(modPath)) {
        console.log('⚠️  mod.rs already exists, skipping generation');
        return;
    }
    
    let modContent = '// Auto-generated module declarations\n\n';
    
    functionNames.forEach(name => {
        const moduleName = camelToSnakeCase(name);
        modContent += `pub mod ${moduleName};\n`;
    });
    
    if (CONFIG.makePublic) {
        modContent += '\n// Re-export all functions\n';
        functionNames.forEach(name => {
            const moduleName = camelToSnakeCase(name);
            modContent += `pub use ${moduleName}::${name};\n`;
        });
    }
    
    fs.writeFileSync(modPath, modContent);
    console.log(`Created: ${modPath}`);
}

function generateUpdatedMainFile(remainingContent, functionNames) {
    const backupPath = CONFIG.sourceFile.replace('.bak', '_updated.rs');
    
    // Add module declarations at the top
    let updatedContent = '// Modules for extracted functions\n';
    if (CONFIG.generateModFile) {
        updatedContent += `mod ${path.basename(CONFIG.destDir)};\n\n`;
    }
    
    updatedContent += remainingContent.join('\n');
    
    fs.writeFileSync(backupPath, updatedContent);
    console.log(`\nCreated updated main file: ${backupPath}`);
}

// Run the extraction
try {
    console.log('🚀 Starting function extraction...');
    console.log(`Source: ${CONFIG.sourceFile}`);
    console.log(`Destination: ${CONFIG.destDir}`);
    console.log(`Pattern: ${CONFIG.functionPattern}`);
    
    extractFunctions();
    
    console.log('\n✅ Function extraction completed successfully!');
    console.log('\nNext steps:');
    console.log('1. Review the generated files');
    console.log('2. Adjust imports in individual files if needed');
    console.log('3. Update your test runner to use the new module structure');
} catch (error) {
    console.error('❌ Error during extraction:', error);
}