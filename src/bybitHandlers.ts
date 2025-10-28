/**
 * Bybit-specific type handling logic
 * 
 * This module contains special case handlers for the Bybit API TypeScript codebase.
 * These handlers are plugged into the generic TypeScript->Rust transpiler.
 */

import { InterfaceDeclaration, Node, TypeAliasDeclaration, SourceFile } from "ts-morph";
import { makeValidRustIdent } from "./utils";
import * as console from "./console";
import { TypeRegistry } from "./typeRegistry";
import { convertTypeAlias, convertInterface } from "./codeGenerator";
import { TypeConverter } from "./typeConverter";
import { RustType } from "./types";

/**
 * Check if a TypeScript interface should be skipped (not generated in Rust)
 * 
 * Bybit-specific pattern: Type-level map interfaces that have string literal property keys
 * These are compile-time only constructs used for type safety in TypeScript
 * Examples: WsAPIOperationResponseMap, WsAPITopicRequestParamMap
 */
export function shouldSkipInterface(
    interfaceDecl: InterfaceDeclaration,
    interfaceName: string
): { skip: boolean; reason?: string } {
    const properties = interfaceDecl.getProperties();
    const isMapInterface = interfaceName.toLowerCase().endsWith('map');
    
    // For all *Map interfaces, check if they're type-level maps
    if (isMapInterface) {
        // Count how many properties are string literal keys vs regular identifiers
        let literalKeyCount = 0;
        for (const prop of properties) {
            const name = prop.getName();
            // TypeScript string literal property names contain dots or dashes
            if (name.includes('.') || name.includes('-')) {
                literalKeyCount++;
            }
        }
        
        // If more than half the properties are string literals, or multiple literals exist, skip it
        const mostlyStringLiterals = literalKeyCount > properties.length / 2;
        const hasMultipleLiterals = literalKeyCount >= 2;
        
        if (mostlyStringLiterals || hasMultipleLiterals) {
            const sample = properties.slice(0, 3).map(p => p.getName()).join(', ');
            console.info(`  ⊘  Interface '${interfaceName}': Type-level map (literals=${literalKeyCount}/${properties.length}, sample=[${sample}])`);
            return {
                skip: true,
                reason: `Type-level map interface (TypeScript compile-time construct only)`
            };
        }
    }
    
    return { skip: false };
}

/**
 * Extract types (interfaces, type aliases) from client TypeScript files
 * and generate them into a separate types file
 * 
 * @param sourceFile - The TypeScript source file to extract types from
 * @param typeRegistry - Type registry for tracking dependencies
 * @param typeConverter - Type converter for converting TypeScript types to Rust
 * @param processedTypes - Set of already processed type names
 * @param relativePath - Relative path of the source file
 * @returns Array of generated type definitions
 */
export function extractClientTypes(
    sourceFile: SourceFile,
    typeRegistry: TypeRegistry,
    typeConverter: TypeConverter,
    processedTypes: Set<string>,
    relativePath: string
): RustType[] {
    const extractedTypes: RustType[] = [];
    const outputFilePath = "client/mod.rs";
    
    console.info(`  📋 Extracting types from client file: ${relativePath}`);
    
    // Process type aliases in client file
    for (const typeAlias of sourceFile.getTypeAliases()) {
        const typeName = typeAlias.getName();
        if (processedTypes.has(typeName)) continue;
        processedTypes.add(typeName);
        
        try {
            const typeNode = typeAlias.getTypeNode();
            if (typeNode) {
                const jsDocs = typeAlias.getJsDocs();
                const result = convertTypeAlias(typeName, typeNode, jsDocs, relativePath);
                
                if (result.code) {
                    extractedTypes.push({
                        name: typeName,
                        content: result.code,
                        category: "type_alias",
                        sourceFile: relativePath,
                    });
                    // Register in type registry
                    typeRegistry.registerType(typeName, outputFilePath);
                } else if (result.skipReason) {
                    extractedTypes.push({
                        name: typeName,
                        content: `// Type alias '${typeName}' skipped: ${result.skipReason}\n`,
                        category: "skipped",
                        sourceFile: relativePath,
                        skipReason: result.skipReason
                    });
                }
            }
        } catch (error) {
            console.warn(`  ⚠️  Error processing type alias '${typeName}' in client file: ${error}`);
        }
    }
    
    // Process interfaces in client file
    for (const interfaceDecl of sourceFile.getInterfaces()) {
        const interfaceName = interfaceDecl.getName();
        if (processedTypes.has(interfaceName)) continue;
        processedTypes.add(interfaceName);
        
        try {
            const typeParams = interfaceDecl.getTypeParameters();
            const generics = typeParams.map(tp => tp.getName());
            
            const members = interfaceDecl.getProperties().map(prop => {
                const propName = prop.getName();
                const isOptional = prop.hasQuestionToken();
                const propType = prop.getType();
                const rustType = typeConverter.convert(propType, isOptional, propName, generics, interfaceName, outputFilePath);
                
                return {
                    name: propName,
                    type: rustType,
                    optional: isOptional,
                    docComment: "",
                    isDeprecated: false
                };
            });
            
            if (members.length > 0) {
                const jsDocs = interfaceDecl.getJsDocs();
                const rustCode = convertInterface(interfaceName, members, generics, jsDocs);
                
                if (rustCode) {
                    extractedTypes.push({
                        name: interfaceName,
                        content: rustCode,
                        category: "struct",
                        sourceFile: relativePath,
                    });
                    // Register in type registry
                    typeRegistry.registerType(interfaceName, outputFilePath);
                }
            }
        } catch (error) {
            console.warn(`  ⚠️  Error processing interface '${interfaceName}' in client file: ${error}`);
        }
    }
    
    return extractedTypes;
}

/**
 * Try to resolve typeof/keyof patterns to concrete enum values
 * 
 * Bybit-specific pattern: type WsKey = (typeof WS_KEY_MAP)[keyof typeof WS_KEY_MAP]
 * This extracts the value type from a const object map.
 */
export function resolveTypeofKeyofPattern(
    typeAlias: TypeAliasDeclaration,
    typeName: string,
    typeText: string
): { resolved: boolean; values?: string[]; error?: string } {
    // Match pattern: (typeof CONST_NAME)[keyof typeof CONST_NAME]
    const typeofMatch = typeText.match(/\(typeof\s+(\w+)\)\s*\[keyof\s+typeof\s+\1\]/);
    
    if (!typeofMatch) {
        return { resolved: false };
    }
    
    const constName = typeofMatch[1];
    console.debug(`  → Attempting to resolve typeof/keyof pattern: ${constName}`);
    
    const typeNode = typeAlias.getTypeNode();
    if (!typeNode) {
        return { resolved: false, error: 'No type node found' };
    }
    
    const sourceFile = typeNode.getSourceFile();
    const importDecls = sourceFile.getImportDeclarations();
    
    console.debug(`  → Found ${importDecls.length} imports in source file`);
    
    // Look for the constant in imports
    let constValues: string[] = [];
    
    for (const importDecl of importDecls) {
        const namedImports = importDecl.getNamedImports();
        
        for (const namedImport of namedImports) {
            const importName = namedImport.getName();
            
            if (importName === constName) {
                console.debug(`  → Found import for ${constName}`);
                
                try {
                    const importedSourceFile = importDecl.getModuleSpecifierSourceFile();
                    if (!importedSourceFile) {
                        console.debug(`  → Could not resolve source file`);
                        continue;
                    }
                    
                    console.debug(`  → Resolved to: ${importedSourceFile.getFilePath()}`);
                    
                    // Find the constant declaration (may need to search through re-exports)
                    const constDecl = findConstantDeclaration(importedSourceFile, constName);
                    
                    if (constDecl) {
                        console.debug(`  → Found constant declaration`);
                        let initializer = constDecl.getInitializer();
                        
                        // Handle 'as const' assertion - unwrap it
                        if (initializer && Node.isAsExpression(initializer)) {
                            console.debug(`  → Unwrapping 'as const'`);
                            initializer = initializer.getExpression();
                        }
                        
                        if (initializer && Node.isObjectLiteralExpression(initializer)) {
                            const properties = initializer.getProperties();
                            console.debug(`  → Object has ${properties.length} properties`);
                            
                            for (const prop of properties) {
                                if (Node.isPropertyAssignment(prop)) {
                                    const initValue = prop.getInitializer();
                                    if (initValue && Node.isStringLiteral(initValue)) {
                                        const value = initValue.getLiteralValue();
                                        constValues.push(value);
                                    }
                                }
                            }
                        }
                    }
                } catch (e) {
                    console.debug(`  → Error resolving ${constName}: ${e}`);
                }
            }
        }
    }
    
    if (constValues.length > 0) {
        console.info(`  ℹ️  Resolved typeof/keyof pattern for '${typeName}' to ${constValues.length} values`);
        return { resolved: true, values: constValues };
    }
    
    return {
        resolved: false,
        error: `Could not resolve constant ${constName} (found 0 values)`
    };
}

/**
 * Helper: Find a constant declaration, following re-exports if necessary
 */
function findConstantDeclaration(sourceFile: any, constName: string, depth: number = 0): any {
    if (depth > 10) return null; // Prevent infinite loops
    
    // Try direct lookup
    let constDecl = sourceFile.getVariableDeclaration(constName);
    if (constDecl) return constDecl;
    
    // Try all variable statements
    for (const statement of sourceFile.getVariableStatements()) {
        for (const decl of statement.getDeclarations()) {
            if (decl.getName() === constName) {
                return decl;
            }
        }
    }
    
    // Follow re-exports recursively
    for (const exportDecl of sourceFile.getExportDeclarations()) {
        const moduleSpec = exportDecl.getModuleSpecifier();
        if (moduleSpec && Node.isStringLiteral(moduleSpec)) {
            try {
                const reexportedFile = exportDecl.getModuleSpecifierSourceFile();
                if (reexportedFile) {
                    const found = findConstantDeclaration(reexportedFile, constName, depth + 1);
                    if (found) return found;
                }
            } catch (e) {
                // Continue searching
            }
        }
    }
    
    return null;
}

/**
 * Generate Rust enum from typeof/keyof pattern values
 */
export function generateEnumFromValues(
    typeName: string,
    values: string[],
    jsDocs: any[]
): string {
    // Convert JSDoc to Rust doc comments
    let docComment = '';
    let isDeprecated = false;
    
    if (jsDocs.length > 0) {
        const lines: string[] = [];
        for (const jsDoc of jsDocs) {
            const comment = jsDoc.getComment();
            if (!comment) continue;
            
            const commentText = typeof comment === 'string' ? comment :
                comment.map((part: any) => part.getText()).join('');
            
            if (jsDoc.getTags().some((tag: any) => tag.getTagName() === 'deprecated')) {
                isDeprecated = true;
            }
            
            const commentLines = commentText.split('\n').map(line => line.trim());
            for (const line of commentLines) {
                if (line) {
                    lines.push(`/// ${line}`);
                }
            }
        }
        
        if (lines.length > 0) {
            docComment = lines.join('\n') + '\n';
        }
    }
    
    // Generate variants
    const variants = values.map(value => {
        let variantName = value
            .replace(/[-_\.]/g, " ")
            .split(" ")
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join("");
        
        variantName = makeValidRustIdent(variantName || value);
        return { name: variantName, value };
    });
    
    // Deduplicate by name
    const seenNames = new Set<string>();
    const uniqueVariants = variants.filter(v => {
        if (seenNames.has(v.name)) return false;
        seenNames.add(v.name);
        return true;
    });
    
    // Generate Rust code
    let rustCode = docComment;
    if (isDeprecated) {
        rustCode += `#[deprecated]\n`;
    }
    
    rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
    rustCode += `pub enum ${typeName} {\n`;
    
    for (let i = 0; i < uniqueVariants.length; i++) {
        const variant = uniqueVariants[i];
        if (i === 0) {
            rustCode += `    #[default]\n`;
        }
        if (variant.name !== variant.value) {
            rustCode += `    #[serde(rename = "${variant.value}")]\n`;
            rustCode += `    #[strum(serialize = "${variant.value}")]\n`;
        }
        rustCode += `    ${variant.name},\n`;
    }
    
    rustCode += `}\n`;
    
    return rustCode;
}



