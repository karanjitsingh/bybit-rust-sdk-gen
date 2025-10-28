/**
 * FileStructure class for organizing Rust file generation
 */

import * as fs from "fs";
import * as path from "path";
import { RustType } from "./types";
import { InlineTypeInfo } from "./inlineTypeRegistry";
import { TypeRegistry } from "./typeRegistry";
import { rustFileToModulePath, makeValidRustIdent } from "./utils";

export class FileStructure {
    public filePath: string;
    public imports: Set<string>;
    public inlineTypes: InlineTypeInfo[];
    public mainContent: RustType[];
    public skippedTypes: RustType[];

    constructor(filePath: string) {
        this.filePath = filePath;
        this.imports = new Set<string>();
        this.inlineTypes = [];
        this.mainContent = [];
        this.skippedTypes = [];
    }

    /**
     * Write the file to disk
     */
    write(outputDir: string, registry: TypeRegistry): void {
        const outputPath = path.join(outputDir, this.filePath);
        const outputDirPath = path.dirname(outputPath);

        // Create directory if needed
        if (!fs.existsSync(outputDirPath)) {
            fs.mkdirSync(outputDirPath, { recursive: true });
        }

        let fileContent = this.generateContent(registry);
        fs.writeFileSync(outputPath, fileContent);
    }

    /**
     * Generate the complete file content
     */
    private generateContent(registry: TypeRegistry): string {
        let content = "";

        // Check if this is a common types file (no main content, only inline types)
        const isCommonFile = this.mainContent.length === 0 && this.inlineTypes.length > 0;

        // File header
        const sourceFile = this.mainContent.length > 0 ? this.mainContent[0].sourceFile : "common inline types";
        content += `// Auto-generated from TypeScript definitions\n`;
        content += `// Source: ${sourceFile}\n\n`;
        content += `#![allow(non_camel_case_types)]\n`;
        content += `#![allow(non_snake_case)]\n\n`;

        // Generate main content first to check what's used
        let mainTypeContent = this.generateMainContent();
        let inlineContent = isCommonFile ? this.generateCommonTypes() : "";
        let fullContent = mainTypeContent + inlineContent;
        
        // Only add serde/builder imports if actually used in the generated content
        const needsSerde = fullContent.includes("Deserialize") || fullContent.includes("Serialize");
        const needsBuilder = fullContent.includes("Builder");
        
        if (needsSerde) {
            content += `use serde::{Deserialize, Serialize};\n`;
        }
        if (needsBuilder) {
            content += `use derive_builder::Builder;\n`;
        }
        if (needsSerde || needsBuilder) {
            content += `\n`;
        }

        // Add imports for dependencies
        const currentModulePath = rustFileToModulePath(this.filePath);
        const currentInlineModulePath = `${currentModulePath}::inline`;
        
        if (this.imports.size > 0) {
            const sortedImports = Array.from(this.imports)
                .filter(imp => {
                    // Skip imports from the same file or this file's inline submodule
                    const match = imp.match(/use ([^:]+::[^:]+(?:::[^:]+)*)::/);
                    if (match) {
                        const impModulePath = match[1];
                        return impModulePath !== currentModulePath && 
                               impModulePath !== currentInlineModulePath;
                    }
                    return true;
                })
                .sort();
            
            if (sortedImports.length > 0) {
                content += sortedImports.join("\n");
                content += "\n";
            }
        }

        // Add strum_macros import if needed
        const hasEnumStringDerive = /#\[derive\([^\]]*EnumString[^\]]*\)\]/.test(fullContent);
        const hasDisplayDerive = /#\[derive\([^\]]*Display[^\]]*\)\]/.test(fullContent);

        if (hasEnumStringDerive || hasDisplayDerive) {
            const imports: string[] = [];
            if (hasEnumStringDerive) imports.push('EnumString');
            if (hasDisplayDerive) imports.push('Display');
            content += `use strum_macros::{${imports.join(', ')}};\n`;
        }

        // Import inline types if this file has any (and it's not a common file)
        if (!isCommonFile && this.inlineTypes.length > 0) {
            content += `\n// Import inline types from the submodule\n`;
            content += `use self::inline::*;\n`;
        }

        content += "\n";
        content += mainTypeContent;

        // Add inline types submodule or common types
        if (isCommonFile) {
            content += inlineContent;
        } else if (this.inlineTypes.length > 0) {
            content += this.generateInlineModule();
        }

        return content;
    }

    /**
     * Generate the main content section
     */
    private generateMainContent(): string {
        let content = "";

        // Add skipped types as comments
        if (this.skippedTypes.length > 0) {
            content += `// ============================================================================\n`;
            content += `// Skipped Type Aliases (${this.skippedTypes.length})\n`;
            content += `// ============================================================================\n`;
            content += `// The following type aliases were not converted. See reasons below:\n`;
            content += `//\n`;
            this.skippedTypes.forEach(rustType => {
                content += rustType.content;
            });
            content += `\n`;
        }

        // Group by category
        const enums = this.mainContent.filter(t => t.category === "enum");
        const typeAliases = this.mainContent.filter(t => t.category === "type_alias");
        const structs = this.mainContent.filter(t => t.category === "struct");

        // Add type aliases
        if (typeAliases.length > 0) {
            typeAliases.forEach(rustType => {
                content += rustType.content;
                content += `\n`;
            });
        }

        // Add enums
        if (enums.length > 0) {
            enums.forEach(rustType => {
                content += rustType.content;
                content += `\n`;
            });
        }

        // Add structs
        if (structs.length > 0) {
            structs.forEach(rustType => {
                content += rustType.content;
                content += `\n`;
            });
        }

        return content;
    }

    /**
     * Generate common types (at top level, not in submodule)
     */
    private generateCommonTypes(): string {
        let content = "";

        content += `// ============================================================================\n`;
        content += `// Common Inline Types\n`;
        content += `// ============================================================================\n`;
        content += `// These types are common across multiple interfaces\n`;
        content += `\n`;

        for (const inlineType of this.inlineTypes) {
            // Generate doc comment with all sources
            if (inlineType.sourceProperty && inlineType.sourceProperty.includes('///')) {
                // Already formatted as doc comments
                content += inlineType.sourceProperty + '\n';
            }

            if (inlineType.isHeterogeneous) {
                // Heterogeneous union
                const isGeneric = inlineType.genericParams && inlineType.genericParams.length > 0;
                const genericDecl = isGeneric ? `<${inlineType.genericParams.join(", ")}>` : '';

                content += `#[derive(Debug, Clone, Serialize, Deserialize)]\n`;
                content += `#[serde(untagged)]\n`;
                content += `pub enum ${inlineType.typeName}${genericDecl} {\n`;

                const seenVariants = new Set<string>();
                for (const variant of inlineType.variants) {
                    let rustType: string;
                    let variantName: string;

                    if (isGeneric && inlineType.genericParams!.includes(variant)) {
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
                        content += `    ${variantName}(${rustType}),\n`;
                    }
                }

                content += `}\n\n`;
            } else {
                // Homogeneous union
                const variants = inlineType.variants.map((v: string) => ({
                    name: makeValidRustIdent(v),
                    value: v
                }));

                content += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
                content += `pub enum ${inlineType.typeName} {\n`;

                const seenNames = new Set<string>();
                let isFirstVariant = true;
                for (const variant of variants) {
                    if (!seenNames.has(variant.name)) {
                        seenNames.add(variant.name);
                        if (isFirstVariant) {
                            content += `    #[default]\n`;
                            isFirstVariant = false;
                        }
                        if (variant.name !== variant.value) {
                            content += `    #[serde(rename = "${variant.value}")]\n`;
                            content += `    #[strum(serialize = "${variant.value}")]\n`;
                        }
                        content += `    ${variant.name},\n`;
                    }
                }

                content += `}\n\n`;
            }
        }

        return content;
    }

    /**
     * Generate the inline types submodule
     */
    private generateInlineModule(): string {
        let content = "";

        content += `// ============================================================================\n`;
        content += `// Inline Types Submodule\n`;
        content += `// ============================================================================\n`;
        content += `// These types are inline unions/literals used only within this file\n`;
        content += `\n`;
        content += `pub mod inline {\n`;
        content += `    use serde::{Deserialize, Serialize};\n`;
        
        // Check if we need strum imports
        const needsStrum = this.inlineTypes.some(t => !t.isHeterogeneous);
        if (needsStrum) {
            content += `    use strum_macros::{EnumString, Display};\n`;
        }
        content += `\n`;

        for (const inlineType of this.inlineTypes) {
            // Generate doc comment
            if (inlineType.sourceInterface && inlineType.sourceProperty) {
                content += `    /// \`${inlineType.sourceInterface}:${inlineType.sourceProperty}\`\n`;
            } else if (inlineType.sourceProperty) {
                // Check if sourceProperty contains multiple source comments (for common types)
                if (inlineType.sourceProperty.includes('///')) {
                    // Already formatted as doc comments
                    content += inlineType.sourceProperty.split('\n').map(line => `    ${line}`).join('\n') + '\n';
                } else {
                    content += `    /// \`${inlineType.sourceProperty}\`\n`;
                }
            }

            if (inlineType.isHeterogeneous) {
                // Heterogeneous union
                const isGeneric = inlineType.genericParams && inlineType.genericParams.length > 0;
                const genericDecl = isGeneric ? `<${inlineType.genericParams.join(", ")}>` : '';

                content += `    #[derive(Debug, Clone, Serialize, Deserialize)]\n`;
                content += `    #[serde(untagged)]\n`;
                content += `    pub enum ${inlineType.typeName}${genericDecl} {\n`;

                const seenVariants = new Set<string>();
                for (const variant of inlineType.variants) {
                    let rustType: string;
                    let variantName: string;

                    if (isGeneric && inlineType.genericParams!.includes(variant)) {
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
                        content += `        ${variantName}(${rustType}),\n`;
                    }
                }

                content += `    }\n\n`;
            } else {
                // Homogeneous union
                const variants = inlineType.variants.map((v: string) => ({
                    name: makeValidRustIdent(v),
                    value: v
                }));

                content += `    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
                content += `    pub enum ${inlineType.typeName} {\n`;

                const seenNames = new Set<string>();
                let isFirstVariant = true;
                for (const variant of variants) {
                    if (!seenNames.has(variant.name)) {
                        seenNames.add(variant.name);
                        if (isFirstVariant) {
                            content += `        #[default]\n`;
                            isFirstVariant = false;
                        }
                        if (variant.name !== variant.value) {
                            content += `        #[serde(rename = "${variant.value}")]\n`;
                            content += `        #[strum(serialize = "${variant.value}")]\n`;
                        }
                        content += `        ${variant.name},\n`;
                    }
                }

                content += `    }\n\n`;
            }
        }

        content += `}\n`;

        return content;
    }
}

