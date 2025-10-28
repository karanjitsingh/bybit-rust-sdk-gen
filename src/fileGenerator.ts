/**
 * File generation and module structure creation
 */

import * as fs from "fs";
import * as path from "path";
import { RustType } from "./types";
import { rustFileToModulePath, escapeRustKeyword, makeValidRustIdent } from "./utils";
import { TypeRegistry } from "./typeRegistry";

export class FileGenerator {
    constructor(private registry: TypeRegistry, private outputDir: string) { }

    /**
     * Generate Rust source file with types and imports
     */
    generateFile(rustFilePath: string, types: RustType[], inlineTypes: any[] = []): void {
        const outputPath = path.join(this.outputDir, rustFilePath);
        const outputDir = path.dirname(outputPath);

        // Create directory if needed
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir, { recursive: true });
        }

        // Generate file content
        let fileContent = `// Auto-generated from TypeScript definitions\n`;
        fileContent += `// Source: ${types[0].sourceFile}\n\n`;
        fileContent += `#![allow(non_camel_case_types)]\n`;
        fileContent += `#![allow(non_snake_case)]\n\n`;

        // Collect all content first to check what types are actually used
        let typeContent = "";

        // Group by category
        const enums = types.filter(t => t.category === "enum");
        const typeAliases = types.filter(t => t.category === "type_alias");
        const structs = types.filter(t => t.category === "struct");
        const skipped = types.filter(t => t.category === "skipped");

        // Add skipped types as comments at the top
        if (skipped.length > 0) {
            typeContent += `// ============================================================================\n`;
            typeContent += `// Skipped Type Aliases (${skipped.length})\n`;
            typeContent += `// ============================================================================\n`;
            typeContent += `// The following type aliases were not converted. See reasons below:\n`;
            typeContent += `//\n`;
            skipped.forEach(rustType => {
                typeContent += rustType.content;
            });
            typeContent += `\n`;
        }

        // Add type aliases (tuples, etc.)
        if (typeAliases.length > 0) {
            typeAliases.forEach(rustType => {
                typeContent += rustType.content;
                typeContent += `\n`;
            });
        }

        // Add enums
        if (enums.length > 0) {
            enums.forEach(rustType => {
                typeContent += rustType.content;
                typeContent += `\n`;
            });
        }

        // Add structs
        if (structs.length > 0) {
            structs.forEach(rustType => {
                typeContent += rustType.content;
                typeContent += `\n`;
            });
        }

        // Only add serde/builder imports if actually used in the generated content
        const needsSerde = typeContent.includes("Deserialize") || typeContent.includes("Serialize");
        const needsBuilder = typeContent.includes("Builder");
        
        if (needsSerde) {
            fileContent += `use serde::{Deserialize, Serialize};\n`;
        }
        if (needsBuilder) {
            fileContent += `use derive_builder::Builder;\n`;
        }
        if (needsSerde || needsBuilder) {
            fileContent += `\n`;
        }

        // Add imports only for types that are actually referenced in the content
        const deps = this.registry.getFileDependencies(rustFilePath);
        
        if (deps && deps.size > 0) {
            const currentModulePath = rustFileToModulePath(rustFilePath);
            const currentInlineModulePath = `${currentModulePath}::inline`;
            const imports = new Set<string>();

            for (const depType of deps) {
                // Only add import if the type is actually used in the content
                // Check with word boundaries to avoid false positives
                const regex = new RegExp(`\\b${depType}\\b`);
                if (regex.test(typeContent)) {
                    const depModulePath = this.registry.getTypeModulePath(depType);
                    // Skip if the type is from the same file
                    // Or if it's from THIS file's inline submodule (already imported with use self::inline::*)
                    if (depModulePath && 
                        depModulePath !== currentModulePath && 
                        depModulePath !== currentInlineModulePath) {
                        imports.add(`use ${depModulePath}::${depType};`);
                    }
                }
            }

            if (imports.size > 0) {
                fileContent += Array.from(imports).sort().join("\n");
                fileContent += "\n";
            }
        }

        // Note: Stub imports are no longer needed as all types are now generated automatically

        // Add strum_macros import only if EnumString or Display are actually used in derive attributes
        // Match #[derive(...)] blocks that contain these traits
        const hasEnumStringDerive = /#\[derive\([^\]]*EnumString[^\]]*\)\]/.test(typeContent);
        const hasDisplayDerive = /#\[derive\([^\]]*Display[^\]]*\)\]/.test(typeContent);

        if (hasEnumStringDerive || hasDisplayDerive) {
            // Only import what's actually used
            const imports: string[] = [];
            if (hasEnumStringDerive) imports.push('EnumString');
            if (hasDisplayDerive) imports.push('Display');
            fileContent += `use strum_macros::{${imports.join(', ')}};\n`;
        }

        // Import inline types if this file has any
        if (inlineTypes.length > 0) {
            fileContent += `\n// Import inline types from the submodule\n`;
            fileContent += `use self::inline::*;\n`;
        }

        fileContent += "\n";
        fileContent += typeContent;

        // Add inline types as a submodule if any exist
        if (inlineTypes.length > 0) {
            fileContent += `// ============================================================================\n`;
            fileContent += `// Inline Types Submodule\n`;
            fileContent += `// ============================================================================\n`;
            fileContent += `// These types are inline unions/literals used only within this file\n`;
            fileContent += `\n`;
            fileContent += `pub mod inline {\n`;
            fileContent += `    use serde::{Deserialize, Serialize};\n`;
            
            // Check if we need strum imports for inline types
            const needsStrumForInline = inlineTypes.some(inlineType => !inlineType.isHeterogeneous);
            if (needsStrumForInline) {
                fileContent += `    use strum_macros::{EnumString, Display};\n`;
            }
            fileContent += `\n`;

            for (const inlineType of inlineTypes) {
                // Generate doc comment with source information if available
                if (inlineType.sourceInterface && inlineType.sourceProperty) {
                    fileContent += `    /// \`${inlineType.sourceInterface}:${inlineType.sourceProperty}\`\n`;
                } else if (inlineType.sourceProperty) {
                    fileContent += `    /// \`${inlineType.sourceProperty}\`\n`;
                }

                if (inlineType.isHeterogeneous) {
                    // Check if this is a generic inline type
                    const isGeneric = inlineType.genericParams && inlineType.genericParams.length > 0;
                    const genericDecl = isGeneric ? `<${inlineType.genericParams.join(", ")}>` : '';

                    // Heterogeneous union: string | number | CustomType
                    fileContent += `    #[derive(Debug, Clone, Serialize, Deserialize)]\n`;
                    fileContent += `    #[serde(untagged)]\n`;
                    fileContent += `    pub enum ${inlineType.typeName}${genericDecl} {\n`;

                    const seenVariants = new Set<string>();
                    for (const variant of inlineType.variants) {
                        let rustType: string;
                        let variantName: string;

                        if (isGeneric && inlineType.genericParams.includes(variant)) {
                            rustType = variant;
                            variantName = variant;
                        } else if (variant === "string") {
                            rustType = "String";
                            variantName = "String";
                        } else if (variant === "number") {
                            rustType = "f64";
                            variantName = "Number";
                        } else if (variant === "boolean") {
                            rustType = "bool";
                            variantName = "Boolean";
                        } else {
                            rustType = variant;
                            variantName = variant;
                        }

                        if (!seenVariants.has(variantName)) {
                            seenVariants.add(variantName);
                            fileContent += `        ${variantName}(${rustType}),\n`;
                        }
                    }

                    fileContent += `    }\n\n`;
                } else {
                    // Homogeneous union: string literals or number literals
                    const variants = inlineType.variants.map((v: string) => ({
                        name: makeValidRustIdent(v),
                        value: v
                    }));

                    fileContent += `    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
                    fileContent += `    pub enum ${inlineType.typeName} {\n`;

                    const seenNames = new Set<string>();
                    let isFirstVariant = true;
                    for (const variant of variants) {
                        if (!seenNames.has(variant.name)) {
                            seenNames.add(variant.name);
                            if (isFirstVariant) {
                                fileContent += `        #[default]\n`;
                                isFirstVariant = false;
                            }
                            if (variant.name !== variant.value) {
                                fileContent += `        #[serde(rename = "${variant.value}")]\n`;
                                fileContent += `        #[strum(serialize = "${variant.value}")]\n`;
                            }
                            fileContent += `        ${variant.name},\n`;
                        }
                    }

                    fileContent += `    }\n\n`;
                }
            }

            fileContent += `}\n`;
        }

        fs.writeFileSync(outputPath, fileContent);
    }

    /**
     * Generate mod.rs file for a directory
     */
    generateModFile(dirPath: string, modules: string[]): void {
        if (modules.length === 0) return;

        const modPath = path.join(this.outputDir, dirPath, "mod.rs");
        modules.sort();
        let modContent = `// Auto-generated module declarations\n\n`;

        // Special handling for client/mod.rs
        if (dirPath === "client") {
            modContent = `// Client module declarations\n\n`;
            modContent += `// Type aliases and error types\n`;
            modContent += `pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}

// Client configuration types (extracted from TypeScript client files)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeferredPromiseRef(pub String);

/// Configurable options specific to only the REST-like WebsocketAPIClient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSAPIClientConfigurableOptions {
    pub attachEventListeners: bool,
}

// Core infrastructure
pub mod config;
pub mod signing;

// Base client modules and re-exports (hand-written, not generated)
#[path = "BaseRestClient.rs"]
mod base_rest_client;
pub use base_rest_client::BaseRestClient;

#[path = "BaseWebsocketClient.rs"]
mod base_websocket_client;
pub use base_websocket_client::BaseWebsocketClient;

// Generated API clients
`;
            // Filter out hand-written modules
            const generatedModules = modules.filter(m => 
                !['config', 'signing', 'BaseRestClient', 'BaseWebsocketClient'].includes(m)
            );
            
            for (const mod of generatedModules) {
                const escapedMod = escapeRustKeyword(mod);
                modContent += `pub mod ${escapedMod};\n`;
            }
        } else {
            // For non-client directories, just list modules
            for (const mod of modules) {
                const escapedMod = escapeRustKeyword(mod);
                modContent += `pub mod ${escapedMod};\n`;
            }
        }

        fs.writeFileSync(modPath, modContent);
    }

    /**
     * Generate lib.rs file
     */
    generateLibFile(topLevelModules: string[]): void {
        const libPath = path.join(this.outputDir, "lib.rs");

        let libContent = `//! Auto-generated Bybit API SDK for Rust
//! 
//! This library provides type-safe Rust bindings for the Bybit API.
//! All types are automatically generated from the official TypeScript SDK.

pub use serde;
pub use serde_json;

`;

        topLevelModules.sort();

        for (const mod of topLevelModules) {
            const escapedMod = escapeRustKeyword(mod);
            libContent += `pub mod ${escapedMod};\n`;
        }

        fs.writeFileSync(libPath, libContent);
    }

    /**
     * Discover modules in a directory
     */
    discoverModules(dirPath: string): string[] {
        const fullPath = path.join(this.outputDir, dirPath);
        if (!fs.existsSync(fullPath)) {
            return [];
        }

        const items = fs.readdirSync(fullPath);
        const modules: string[] = [];

        for (const item of items) {
            const itemPath = path.join(fullPath, item);
            const stat = fs.statSync(itemPath);

            if (stat.isDirectory()) {
                modules.push(item);
            } else if (item.endsWith(".rs") && item !== "mod.rs" && item !== "lib.rs") {
                const moduleName = item.replace(/\.rs$/, "");
                modules.push(moduleName);
            }
        }

        return modules;
    }
}

