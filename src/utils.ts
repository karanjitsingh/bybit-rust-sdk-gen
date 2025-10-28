/**
 * Utility functions for naming and identifier manipulation
 */

// Rust keywords that need to be escaped
export const RUST_KEYWORDS = new Set([
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
    "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
    "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield"
]);

/**
 * Escape Rust keywords by prefixing with r#
 */
export function escapeRustKeyword(name: string): string {
    if (RUST_KEYWORDS.has(name)) {
        return `r#${name}`;
    }
    return name;
}

/**
 * Make a valid Rust identifier from a string
 */
export function makeValidRustIdent(name: string): string {
    // Handle empty strings
    if (!name || name.trim() === "") {
        return "Empty";
    }
    
    // If it starts with a number or is just numbers, prefix with T
    if (/^[0-9]/.test(name)) {
        return "T" + name.replace(/[^a-zA-Z0-9_]/g, "_");
    }
    
    // Replace invalid characters with underscore
    return name.replace(/[^a-zA-Z0-9_]/g, "_");
}

/**
 * Convert TypeScript file path to Rust file path
 * Example: types/request/v5-account.ts -> types/request/v5_account.rs
 * Special case: constants/enum.ts -> constants/enums.rs (avoid Rust keyword)
 */
export function tsFileToRustFile(tsRelativePath: string): string {
    let rustPath = tsRelativePath
        .replace(/\.ts$/, ".rs")
        .replace(/-/g, "_");
    
    // Rename enum.rs to enums.rs to avoid Rust keyword
    if (rustPath === "constants/enum.rs") {
        rustPath = "constants/enums.rs";
    }
    
    return rustPath;
}

/**
 * Convert Rust file path to module path (for imports)
 * Example: types/request/v5_account.rs -> crate::types::request::v5_account
 * Example: constants/enums.rs -> crate::constants::enums
 */
export function rustFileToModulePath(rustFilePath: string): string {
    let modulePath = "crate::" + rustFilePath
        .replace(/\.rs$/, "")
        .replace(/\//g, "::");
    
    // Handle mod.rs files - they represent the parent module, not a submodule named "mod"
    // e.g., "util/mod.rs" -> "crate::util", not "crate::util::mod"
    modulePath = modulePath.replace(/::mod$/, "");
    
    // Escape Rust keywords if needed (e.g., type -> r#type), but not 'crate', 'self', 'super' at start of paths
    // Note: We renamed enum.rs to enums.rs to avoid this issue
    const rustKeywords = ['as', 'break', 'const', 'continue', 'else', 'enum', 
                          'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 
                          'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 
                          'static', 'struct', 'trait', 'true', 
                          'type', 'unsafe', 'use', 'where', 'while'];
    
    const parts = modulePath.split('::');
    return parts.map((part, index) => {
        // Don't escape crate, self, super when they're path roots (first position)
        if (index === 0 && (part === 'crate' || part === 'self' || part === 'super')) {
            return part;
        }
        // Escape other keywords
        return rustKeywords.includes(part) ? `r#${part}` : part;
    }).join('::');
}

