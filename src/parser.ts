/**
 * Main parser orchestration
 * Parses TypeScript files and generates Rust code
 */

import { Project } from "ts-morph";
import * as path from "path";
import * as fs from "fs";
import { RustType } from "./types";
import { tsFileToRustFile, rustFileToModulePath, makeValidRustIdent } from "./utils";
import { TypeRegistry } from "./typeRegistry";
import { InlineTypeRegistry } from "./inlineTypeRegistry";
import { TypeConverter } from "./typeConverter";
import { convertTypeAlias, convertInterface, convertEnum } from "./codeGenerator";
import { FileGenerator } from "./fileGenerator";
import { FileStructure } from "./fileStructure";
import { ParserErrorCollector } from "./errors";
import * as console from "./console";
import * as BybitHandlers from "./bybitHandlers";
import { ClientTranspiler } from "./clientTranspiler";
import { BybitClientHandlers } from "./bybitClientHandlers";

const BYBIT_API_DIR = path.join(__dirname, "../bybit-api");
const GEN_DIR = path.join(__dirname, "../bybit-rust-sdk/src");

function toBybitSourcePath(relativePath: string): string {
    return `bybit-api/src/${relativePath.replace(/\\/g, "/")}`;
}

// Create output directory if it doesn't exist
if (!fs.existsSync(GEN_DIR)) {
    fs.mkdirSync(GEN_DIR, { recursive: true });
}

const project = new Project({
    tsConfigFilePath: path.join(BYBIT_API_DIR, "tsconfig.json"),
});

// Add source files from the types directory
project.addSourceFilesAtPaths(`${BYBIT_API_DIR}/src/**/*.ts`);

// Initialize core components
const typeRegistry = new TypeRegistry();
const inlineTypeRegistry = new InlineTypeRegistry();
const typeConverter = new TypeConverter(typeRegistry, inlineTypeRegistry);
const fileGenerator = new FileGenerator(typeRegistry, GEN_DIR);
const errorCollector = new ParserErrorCollector();

// Map from output file path to FileStructure
const fileStructures = new Map<string, FileStructure>();
const processedTypes = new Set<string>();
const directories = new Set<string>();

/**
 * Get or create a FileStructure for the given file
 */
function getFileStructure(rustFilePath: string): FileStructure {
    if (!fileStructures.has(rustFilePath)) {
        fileStructures.set(rustFilePath, new FileStructure(rustFilePath));
    }
    
    return fileStructures.get(rustFilePath)!;
}

/**
 * Add a type to the appropriate file
 */
function addTypeToFile(rustType: RustType, tsRelativePath: string) {
    const rustFilePath = tsFileToRustFile(tsRelativePath);
    const fileStructure = getFileStructure(rustFilePath);
    
    if (rustType.category === 'skipped') {
        fileStructure.skippedTypes.push(rustType);
    } else {
        fileStructure.mainContent.push(rustType);
    }

    // Track directories
    const dir = path.dirname(rustFilePath);
    if (dir !== ".") {
        const parts = dir.split(path.sep);
        for (let i = 0; i < parts.length; i++) {
            directories.add(parts.slice(0, i + 1).join(path.sep));
        }
    }
}

/**
 * Check if a file should be skipped
 */
function shouldSkipFile(relativePath: string): boolean {
    return relativePath.startsWith("util/") ||
        relativePath.includes("client") ||
        relativePath === "index.ts" ||
        (relativePath.includes("websocket-") && !relativePath.includes("types/websockets/"));
}

// PASS 1: Collect all type names and register them
console.info("Pass 1: Collecting type names...\n");

project.getSourceFiles().forEach(sourceFile => {
    const filePath = sourceFile.getFilePath();

    // Skip node_modules and non-type files
    if (filePath.includes("node_modules") || !filePath.includes("/src/")) {
        return;
    }

    const relativePath = path.relative(path.join(BYBIT_API_DIR, "src"), filePath);

    if (shouldSkipFile(relativePath)) {
        return;
    }

    const rustFilePath = tsFileToRustFile(relativePath);
    const modulePath = rustFileToModulePath(rustFilePath);

    // Register type aliases (skip generic ones)
    sourceFile.getTypeAliases().forEach(typeAlias => {
        const typeName = typeAlias.getName();
        // Skip generic type aliases
        if (typeAlias.getTypeParameters().length > 0) {
            return;
        }
        if (!processedTypes.has(typeName)) {
            typeRegistry.registerType(typeName, modulePath);
        }
    });

    // Register interfaces (including generic ones now)
    sourceFile.getInterfaces().forEach(interfaceDecl => {
        const interfaceName = interfaceDecl.getName();
        if (!processedTypes.has(interfaceName)) {
            typeRegistry.registerType(interfaceName, modulePath);
        }
    });

    // Register enums
    sourceFile.getEnums().forEach(enumDecl => {
        const enumName = enumDecl.getName();
        if (!processedTypes.has(enumName)) {
            typeRegistry.registerType(enumName, modulePath);
        }
    });
});

// PASS 2: Process all types and track dependencies
console.info("\nPass 2: Processing types and tracking dependencies...\n");

project.getSourceFiles().forEach(sourceFile => {
    const filePath = sourceFile.getFilePath();

    // Skip node_modules and non-type files
    if (filePath.includes("node_modules") || !filePath.includes("/src/")) {
        return;
    }

    const relativePath = path.relative(path.join(BYBIT_API_DIR, "src"), filePath);

    if (shouldSkipFile(relativePath)) {
        return;
    }

    console.log(`Processing: ${relativePath}`);

    // Process type aliases
    sourceFile.getTypeAliases().forEach(typeAlias => {
        const typeName = typeAlias.getName();

        // Skip generic type aliases - Rust doesn't support them the same way
        if (typeAlias.getTypeParameters().length > 0) {
            console.debug(`  ⊘ Type alias: ${typeName} (generic - skipped)`);
            return;
        }

        if (processedTypes.has(typeName)) return;
        processedTypes.add(typeName);

        const typeNode = typeAlias.getTypeNode();
        if (typeNode) {
            try {
                const jsDocs = typeAlias.getJsDocs();
                const result = convertTypeAlias(typeName, typeNode, jsDocs, relativePath);

                if (result.code) {
                    // Determine category: "enum" for string literal unions, "struct" for object types, "type_alias" for tuples
                    let category: string;
                    if (result.code.includes('pub enum')) {
                        category = 'enum';
                    } else if (result.code.includes('pub struct')) {
                        category = 'struct';
                    } else {
                        category = 'type_alias';
                    }
                    addTypeToFile({
                        name: typeName,
                        content: result.code,
                        category: category as any,
                        sourceFile: toBybitSourcePath(relativePath)
                    }, relativePath);

                    // Track type dependencies from generated code (especially for type aliases
                    // that reference other types, e.g., pub type X = Y<Z>)
                    const aliasRustFilePath = tsFileToRustFile(relativePath);
                    const typeRefPattern = /\b([A-Z][A-Za-z0-9_]*)\b/g;
                    let match;
                    while ((match = typeRefPattern.exec(result.code)) !== null) {
                        const ref = match[1];
                        if (ref !== typeName && typeRegistry.isKnownType(ref)) {
                            typeRegistry.trackTypeDependency(aliasRustFilePath, ref);
                        }
                    }

                    console.success(`  ✓ Type alias: ${typeName}`);
                } else if (result.skipReason) {
                    // Add skipped type as a comment
                    // Make sure the comment is properly formatted (single line)
                    const commentText = result.skipReason.replace(/\n/g, ' ');
                    addTypeToFile({
                        name: typeName,
                        content: `// Type alias '${typeName}' skipped: ${commentText}\n`,
                        category: "skipped",
                        sourceFile: toBybitSourcePath(relativePath),
                        skipReason: result.skipReason
                    }, relativePath);
                }
                // Warnings are now printed by convertTypeAlias itself
            } catch (error) {
                errorCollector.addTypeConversionError(
                    typeName,
                    relativePath,
                    "Type alias conversion failed",
                    error instanceof Error ? error.message : String(error)
                );
            }
        }
    });

    // Process interfaces
    sourceFile.getInterfaces().forEach(interfaceDecl => {
        const interfaceName = interfaceDecl.getName();

        if (processedTypes.has(interfaceName)) return;
        processedTypes.add(interfaceName);

        // Check if this is a generic interface
        const typeParams = interfaceDecl.getTypeParameters();
        const isGeneric = typeParams.length > 0;

        const rustFilePath = tsFileToRustFile(relativePath);
        
        // BYBIT-SPECIFIC: Check if this interface should be skipped
        const skipCheck = BybitHandlers.shouldSkipInterface(interfaceDecl, interfaceName);
        if (skipCheck.skip) {
            addTypeToFile({
                name: interfaceName,
                content: `// Interface '${interfaceName}' skipped: ${skipCheck.reason}\n`,
                category: "skipped",
                sourceFile: toBybitSourcePath(relativePath),
                skipReason: skipCheck.reason
            }, relativePath);
            return;
        }

        try {
            // Extract generic type parameters with defaults
            let generics: string[] = [];
            let genericParamNames: string[] = []; // Just the parameter names for comparison
            if (isGeneric) {
                const genericsWithDefaults: string[] = [];
                const genericsWithoutDefaults: string[] = [];
                
                typeParams.forEach(tp => {
                    const paramName = tp.getName();
                    genericParamNames.push(paramName); // Store the bare name

                    const defaultType = tp.getDefault();

                    // If there's a default type, convert it to Rust syntax
                    if (defaultType) {
                        const defaultTypeText = defaultType.getText();

                        // Handle special cases
                        // Skip defaults for 'object' and 'any' - just keep the generic parameter
                        if (defaultTypeText === 'object' || defaultTypeText === 'any') {
                            genericsWithoutDefaults.push(paramName);
                            return;
                        }
                        
                        let rustDefault: string;
                        if (defaultTypeText === 'undefined' || defaultTypeText === '{}') {
                            // undefined and empty object in TypeScript map to unit type in Rust
                            rustDefault = '()';
                        } else if (defaultTypeText === 'string') {
                            rustDefault = 'String';
                        } else if (defaultTypeText === 'number') {
                            rustDefault = 'f64';
                        } else if (defaultTypeText === 'boolean') {
                            rustDefault = 'bool';
                        } else if (defaultTypeText.endsWith('[]')) {
                            // Array type
                            const elementType = defaultTypeText.slice(0, -2);
                            rustDefault = `Vec<${elementType}>`;
                        } else {
                            // For other types, use as-is (assumes it's a known type)
                            rustDefault = defaultTypeText;

                            // Track the default type as a dependency
                            if (typeRegistry.isKnownType(defaultTypeText)) {
                                typeRegistry.trackTypeDependency(rustFilePath, defaultTypeText);
                            }
                        }

                        genericsWithDefaults.push(`${paramName} = ${rustDefault}`);
                    } else {
                        genericsWithoutDefaults.push(paramName);
                    }
                });
                
                // In Rust, parameters with defaults must come last
                generics = [...genericsWithoutDefaults, ...genericsWithDefaults];
            }

            const members = interfaceDecl.getProperties().map(prop => {
                const propName = prop.getName();
                const isOptional = prop.hasQuestionToken();

                // Extract JSDoc for the property
                const propJsDocs = prop.getJsDocs();
                let docComment = '';
                let isDeprecated = false;

                if (propJsDocs.length > 0) {
                    const docLines: string[] = [];
                    for (const jsDoc of propJsDocs) {
                        // Check for @deprecated tag
                        const deprecatedTag = jsDoc.getTags().find(tag => tag.getTagName() === 'deprecated');
                        if (deprecatedTag) {
                            isDeprecated = true;
                            // Check if there's comment text with the @deprecated tag
                            const tagComment = deprecatedTag.getComment();
                            if (tagComment) {
                                const tagText = typeof tagComment === 'string' ? tagComment :
                                    tagComment.map(part => part.getText()).join('');
                                if (tagText.trim()) {
                                    docLines.push(`    /// @deprecated ${tagText.trim()}`);
                                }
                            }
                        }

                        // Get the main comment (excluding tags)
                        const comment = jsDoc.getComment();
                        if (comment) {
                            const commentText = typeof comment === 'string' ? comment :
                                comment.map(part => part.getText()).join('');

                            // Remove @deprecated from comment text if present (already handled above)
                            const cleanedText = commentText.replace(/@deprecated\s*/g, '').trim();

                            if (cleanedText) {
                                const lines = cleanedText.split('\n').map(line => line.trim());
                                for (const line of lines) {
                                    if (line) {
                                        docLines.push(`    /// ${line}`);
                                    }
                                }
                            }
                        }
                    }
                    if (docLines.length > 0) {
                        docComment = docLines.join('\n') + '\n';
                    }
                }

                // Also check for trailing comments (e.g., "field: type; // comment")
                if (!docComment) {
                    // Get the full source line to capture trailing comments
                    const sourceFile = prop.getSourceFile();
                    const startLine = prop.getStartLineNumber();
                    const fullLine = sourceFile.getFullText().split('\n')[startLine - 1];

                    // Match both // and /* */ style comments at the end of the line
                    const trailingCommentMatch = fullLine.match(/\/\/\s*(.+)$/) ||
                        fullLine.match(/\/\*\s*(.+?)\s*\*\/\s*$/);
                    if (trailingCommentMatch) {
                        const trailingComment = trailingCommentMatch[1].trim();
                        if (trailingComment) {
                            docComment = `    /// ${trailingComment}\n`;
                        }
                    }
                }

                // Get the type node to check if it's a direct type parameter reference
                const typeNode = prop.getTypeNode();
                let rustType: string;

                // Check if the type node is a direct reference to a known type
                if (typeNode && typeNode.getText) {
                    const typeText = typeNode.getText();
                    
                    // If it's a generic parameter, use it directly
                    if (generics.length > 0 && genericParamNames.includes(typeText)) {
                        rustType = typeText;
                    }
                    // If it's a reference to a known type alias/interface/enum, use that directly
                    else if (typeRegistry.isKnownType(typeText)) {
                        rustType = typeText;
                        typeRegistry.trackTypeDependency(rustFilePath, typeText);
                    } else {
                        // Otherwise, convert the resolved type normally
                        const propType = prop.getType();
                        rustType = typeConverter.convert(propType, isOptional, propName, generics, interfaceName, rustFilePath);
                        
                        // Extract and track type dependencies (including inline types)
                        const typeRefs = typeConverter.extractTypeReferences(propType);
                        for (const typeRef of typeRefs) {
                            typeRegistry.trackTypeDependency(rustFilePath, typeRef);
                        }
                        
                        // Also check if rustType directly contains a known type reference
                        // (e.g., from enum literal union detection)
                        // Extract type from Option<T> or Vec<T> wrappers
                        let cleanType = rustType.replace(/^Option<(.+)>$/, '$1').replace(/^Vec<(.+)>$/, '$1');
                        if (typeRegistry.isKnownType(cleanType)) {
                            typeRegistry.trackTypeDependency(rustFilePath, cleanType);
                        }
                    }
                } else {
                    // No type node or no generics, convert normally
                    const propType = prop.getType();
                    rustType = typeConverter.convert(propType, isOptional, propName, generics, interfaceName, rustFilePath);
                    
                    // Extract and track type dependencies (including inline types)
                    const typeRefs = typeConverter.extractTypeReferences(propType);
                    for (const typeRef of typeRefs) {
                        typeRegistry.trackTypeDependency(rustFilePath, typeRef);
                    }
                    
                    // Also check if rustType directly contains a known type reference
                    // (e.g., from enum literal union detection)
                    // Extract type from Option<T> or Vec<T> wrappers
                    let cleanType = rustType.replace(/^Option<(.+)>$/, '$1').replace(/^Vec<(.+)>$/, '$1');
                    if (typeRegistry.isKnownType(cleanType)) {
                        typeRegistry.trackTypeDependency(rustFilePath, cleanType);
                    }
                }

                // Check if we used any inline types and track them as dependencies
                // Match types ending with "Type" or "InlineType" followed by numbers
                const inlineTypePattern = /\b([a-zA-Z_][a-zA-Z0-9_]*Type\d*)\b/g;
                let match;
                while ((match = inlineTypePattern.exec(rustType)) !== null) {
                    const potentialInlineType = match[1];
                    if (typeRegistry.isKnownType(potentialInlineType)) {
                        typeRegistry.trackTypeDependency(rustFilePath, potentialInlineType);
                    }
                }

                return {
                    name: propName,
                    type: rustType,
                    optional: isOptional,
                    docComment,
                    isDeprecated
                };
            });

            const interfaceJsDocs = interfaceDecl.getJsDocs();
            const rustCode = convertInterface(interfaceName, members, generics, interfaceJsDocs);

            if (rustCode) {
                addTypeToFile({
                    name: interfaceName,
                    content: rustCode,
                    category: "struct",
                    sourceFile: toBybitSourcePath(relativePath)
                }, relativePath);
                console.success(`  ✓ Interface: ${interfaceName}${isGeneric ? ' (generic)' : ''}`);
            }
            // Warnings are now printed by convertInterface itself
        } catch (error) {
            errorCollector.addTypeConversionError(
                interfaceName,
                relativePath,
                "Interface conversion failed",
                error instanceof Error ? error.message : String(error)
            );
        }
    });

    // Process enums
    sourceFile.getEnums().forEach(enumDecl => {
        const enumName = enumDecl.getName();

        if (processedTypes.has(enumName)) return;
        processedTypes.add(enumName);

        try {
            const members = enumDecl.getMembers().map(member => ({
                name: member.getName(),
                value: member.getValue()
            }));

            const enumJsDocs = enumDecl.getJsDocs();
            const rustCode = convertEnum(enumName, members, enumJsDocs);

            addTypeToFile({
                name: enumName,
                content: rustCode,
                category: "enum",
                sourceFile: toBybitSourcePath(relativePath)
            }, relativePath);
            console.success(`  ✓ Enum: ${enumName}`);
        } catch (error) {
            errorCollector.addTypeConversionError(
                enumName,
                relativePath,
                "Enum conversion failed",
                error instanceof Error ? error.message : String(error)
            );
        }
    });

    // Process inline object types generated during type conversion
    const inlineObjects = typeConverter.getInlineObjects();
    if (inlineObjects.size > 0) {
        for (const [typeName, objInfo] of inlineObjects) {
            if (processedTypes.has(typeName)) continue;
            processedTypes.add(typeName);

            try {
                // Convert the fields to StructMember format
                const members = objInfo.fields.map(field => ({
                    name: field.name,
                    type: field.type,
                    optional: field.optional,
                    docComment: '',
                    isDeprecated: false
                }));

                // Generate the struct
                const rustCode = convertInterface(typeName, members, []);
                if (rustCode) {
                    addTypeToFile({
                        name: typeName,
                        content: rustCode,
                        category: "struct",
                        sourceFile: toBybitSourcePath(relativePath)
                    }, relativePath);
                    console.debug(`  → Generated inline object struct: ${typeName}`);
                }
            } catch (error) {
                console.warn(`  ⚠️  Failed to generate inline object struct '${typeName}': ${error}`);
            }
        }
    }

    // Clear inline objects for next file
    typeConverter.clearInlineObjects();
});

// Don't exit early - collect all errors and print them at the end
// This allows us to generate files for types that succeeded even if some failed

/**
 * Extract common inline types BEFORE cross-file consolidation
 * This finds types with same field name + signature across multiple usages
 */
function extractCommonInlineTypesEarly(
    inlineTypes: ReturnType<typeof inlineTypeRegistry.getAllInlineTypes>,
    inlineTypesByFile: Map<string, typeof inlineTypes>
) {
    // Build signature map: fieldName:signature -> candidate
    interface CommonTypeCandidate {
        fieldName: string;
        signature: string;
        sources: Array<{ file: string; interface: string }>;
        representativeType: typeof inlineTypes[0];
        matchingTypes: typeof inlineTypes;
    }
    
    const signatureMap = new Map<string, CommonTypeCandidate>();
    
    // Analyze all inline types
    for (const inlineType of inlineTypes) {
        const fieldName = (inlineType.sourceProperty || '').toLowerCase();
        if (!fieldName) continue;
        
        const signature = inlineType.variants.slice().sort().join('|');
        const key = `${fieldName}:${signature}`;
        
        if (!signatureMap.has(key)) {
            signatureMap.set(key, {
                fieldName,
                signature,
                sources: [],
                representativeType: inlineType,
                matchingTypes: []
            });
        }
        
        const candidate = signatureMap.get(key)!;
        candidate.matchingTypes.push(inlineType);
        
        if (inlineType.sourceInterface && inlineType.sourceFile) {
            candidate.sources.push({
                file: inlineType.sourceFile,
                interface: inlineType.sourceInterface
            });
        }
    }
    
    // Find common types (same field name + signature, used in 2+ locations)
    const commonTypeCandidates = Array.from(signatureMap.values()).filter(candidate => {
        return candidate.sources.length >= 2;
    });
    
    console.info(`  Found ${signatureMap.size} unique field+signature combinations`);
    console.info(`  ${commonTypeCandidates.length} are used in multiple places`);
    
    if (commonTypeCandidates.length === 0) {
        return; // No common types to extract
    }
    
    console.info(`\nExtracting ${commonTypeCandidates.length} common inline types...`);
    
    // Determine common types file name
    const commonFileName = fs.existsSync(path.join(GEN_DIR, "types/common.rs"))
        ? "types/common_inline.rs"
        : "types/common.rs";
    
    // Track which types to remove from individual files
    const typesToRemove = new Set<string>();
    
    // Process each common type
    for (const candidate of commonTypeCandidates) {
        // Generate type name: capitalize field name
        const typeName = candidate.fieldName
            .split(/[_-]/)
            .filter(word => word.length > 0)
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join('');
        
        console.info(`  → ${typeName} from field "${candidate.fieldName}" (${candidate.sources.length} uses)`);
        
        // Create doc comment listing all sources
        const sourceComments = candidate.sources
            .map(s => `/// ${s.file}:${s.interface}`)
            .join('\n');
        
        // Create the common inline type
        const commonInlineType = {
            ...candidate.representativeType,
            typeName: typeName,
            sourceInterface: undefined,
            sourceProperty: sourceComments,
            sourceFile: commonFileName
        };
        
        // Add to common file
        if (!inlineTypesByFile.has(commonFileName)) {
            inlineTypesByFile.set(commonFileName, []);
        }
        inlineTypesByFile.get(commonFileName)!.push(commonInlineType);
        
        // Mark matching types for removal
        for (const matchingType of candidate.matchingTypes) {
            typesToRemove.add(matchingType.typeName);
        }
        
        // Register the common type
        const modulePath = commonFileName.replace(/\.rs$/, '').replace(/\//g, '::');
        typeRegistry.registerType(typeName, `crate::${modulePath}`);
        
        // Track dependencies for files that use this common type
        for (const source of candidate.sources) {
            if (source.file && source.file !== commonFileName) {
                typeRegistry.trackTypeDependency(source.file, typeName);
            }
        }
    }
    
    // Remove extracted types from individual files
    for (const [file, types] of inlineTypesByFile.entries()) {
        if (file === commonFileName) continue; // Don't remove from common file
        
        inlineTypesByFile.set(file, types.filter(t => !typesToRemove.has(t.typeName)));
    }
}

// ========================================
// Parse util folder for utility types (WebSocket utilities, etc.)
// NOTE: Must run BEFORE inline type collection so inline types from util
// interfaces (e.g., EmittableEvent_EventType) are included.
// ========================================
console.info(`\nParsing util folder for utility types...`);

const utilFiles = project.getSourceFiles().filter(sf => {
    const filePath = sf.getFilePath();
    const relativePath = path.relative(path.join(BYBIT_API_DIR, "src"), filePath);
    const isBaseClient = relativePath.includes("BaseRestClient") || relativePath.includes("BaseWebsocketClient") || relativePath.includes("BaseWSClient");
    const isWsStore = relativePath.includes("WsStore");
    const isLogger = relativePath.includes("logger");
    const isRequestUtils = relativePath.includes("requestUtils");
    return relativePath.startsWith("util/") && !relativePath.includes("node_modules") && !isBaseClient && !isWsStore && !isLogger && !isRequestUtils;
});

const utilTypes: RustType[] = [];
const utilOutputFile = "util/mod.rs";
const utilModulePath = rustFileToModulePath(utilOutputFile);

const baseWSClientFile = project.getSourceFile(path.join(BYBIT_API_DIR, "src/util/BaseWSClient.ts"));
if (baseWSClientFile) {
    utilFiles.push(baseWSClientFile);
}

for (const sourceFile of utilFiles) {
    const relativePath = path.relative(path.join(BYBIT_API_DIR, "src"), sourceFile.getFilePath());
    console.info(`  Processing: ${relativePath}`);
    
    for (const interfaceDecl of sourceFile.getInterfaces()) {
        const interfaceName = interfaceDecl.getName();
        if (processedTypes.has(interfaceName)) continue;
        if (interfaceName.includes("EventMap") || interfaceName.includes("PendingTopicSubscriptions")) continue;
        const skipCheck = BybitHandlers.shouldSkipInterface(interfaceDecl, interfaceName);
        if (skipCheck.skip) continue;
        processedTypes.add(interfaceName);
        
        const typeParams = interfaceDecl.getTypeParameters();
        const generics = typeParams.map(tp => tp.getName());
        
        const members = interfaceDecl.getProperties().map(prop => {
            const propName = prop.getName();
            const isOptional = prop.hasQuestionToken();
            const propType = prop.getType();
            const rustType = typeConverter.convert(propType, isOptional, propName, generics, interfaceName, utilOutputFile);
            return { name: propName, type: rustType, optional: isOptional, docComment: "", isDeprecated: false };
        });
        
        if (members.length > 0) {
            const jsDocs = interfaceDecl.getJsDocs();
            const rustCode = convertInterface(interfaceName, members, generics, jsDocs);
            if (rustCode) {
                utilTypes.push({ name: interfaceName, content: rustCode, category: "struct", sourceFile: toBybitSourcePath(relativePath) });
                typeRegistry.registerType(interfaceName, utilModulePath);
                console.success(`  ✓ Interface: ${interfaceName}`);
            }
        }
    }
    
    for (const typeAlias of sourceFile.getTypeAliases()) {
        const typeName = typeAlias.getName();
        if (processedTypes.has(typeName)) continue;
        processedTypes.add(typeName);
        
        const typeNode = typeAlias.getTypeNode();
        if (typeNode) {
            const jsDocs = typeAlias.getJsDocs();
            const result = convertTypeAlias(typeName, typeNode, jsDocs, relativePath);
            if (result.code) {
                utilTypes.push({ name: typeName, content: result.code, category: "type_alias", sourceFile: toBybitSourcePath(relativePath) });
                typeRegistry.registerType(typeName, utilModulePath);
                console.success(`  ✓ Type alias: ${typeName}`);
            } else if (result.skipReason) {
                utilTypes.push({ name: typeName, content: `// Type alias '${typeName}' skipped: ${result.skipReason}\n`, category: "skipped", sourceFile: toBybitSourcePath(relativePath), skipReason: result.skipReason });
            }
        }
    }
}

console.info(`  Found ${utilTypes.length} types in util folder`);

if (utilTypes.length > 0) {
    const utilFileStructure = getFileStructure(utilOutputFile);
    utilTypes.forEach(type => utilFileStructure.mainContent.push(type));
}

// Group inline types by their source file
const inlineTypes = inlineTypeRegistry.getAllInlineTypes();
const inlineTypesByFile = new Map<string, typeof inlineTypes>();

// Load shared type overrides
const sharedTypesPath = path.join(path.dirname(GEN_DIR), "..", "shared-types.json");
const sharedTypeOverrides: Record<string, [string, string, string]> = fs.existsSync(sharedTypesPath)
    ? JSON.parse(fs.readFileSync(sharedTypesPath, "utf8"))
    : {};
const overrideCount = Object.keys(sharedTypeOverrides).length;
if (overrideCount > 0) {
    console.info(`\nLoaded ${overrideCount} shared type overrides from shared-types.json`);
}

// Glob-style matching: * matches any substring
function globMatch(pattern: string, value: string, caseInsensitive: boolean = false): boolean {
    const flags = caseInsensitive ? 'i' : '';
    const regex = new RegExp('^' + pattern.split('*').map(s => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('.*') + '$', flags);
    return regex.test(value);
}

// Apply overrides: promote matching types to shared
const typeRenames = new Map<string, string>();
const overrideMatches = new Map<string, number>(); // track match count per override
const overrideMatchedTypes = new Map<string, typeof inlineTypes>(); // track matched types for report
{
    for (const sharedName of Object.keys(sharedTypeOverrides)) {
        overrideMatches.set(sharedName, 0);
    }

    for (const [sharedName, [ifacePattern, fieldPattern, sigPattern]] of Object.entries(sharedTypeOverrides)) {
        const matched: typeof inlineTypes = [];

        for (const t of inlineTypes) {
            if (globMatch(sigPattern, t.signature)
                && globMatch(ifacePattern, t.sourceInterface || '', true)
                && globMatch(fieldPattern, t.sourceProperty || '', true)) {
                matched.push(t);
            }
        }

        overrideMatches.set(sharedName, matched.length);

        if (matched.length === 0) continue;

        overrideMatchedTypes.set(sharedName, [...matched]);

        console.info(`  → Override: ${sharedName} (${matched.length} matches)`);

        const allFiles = new Set<string>();
        for (const t of matched) {
            if (t.sourceFile) allFiles.add(t.sourceFile);
        }

        const first = matched[0];
        const sharedType: typeof inlineTypes[0] = {
            ...first,
            typeName: sharedName,
            signature: first.signature,
            sourceFile: "types/shared.rs",
            usedInFiles: allFiles,
        };

        for (const t of matched) {
            inlineTypeRegistry.removeType(t.typeName);
            typeRenames.set(t.typeName, sharedName);
        }

        inlineTypeRegistry.addType(sharedType);
    }

    // Error on overrides that never matched
    const errors: string[] = [];
    for (const [name, count] of overrideMatches) {
        if (count === 0) {
            const [ifacePattern, fieldPattern, sigPattern] = sharedTypeOverrides[name];
            errors.push(`Override "${name}" [${ifacePattern}, ${fieldPattern}, ${sigPattern}] matched no inline types`);
        }
    }
    if (errors.length > 0) {
        for (const err of errors) {
            console.error(`  ✗ ${err}`);
        }
        throw new Error(`${errors.length} shared type override(s) matched nothing — fix or remove them from shared-types.json`);
    }
}

// Re-fetch after overrides
const resolvedInlineTypes = inlineTypeRegistry.getAllInlineTypes();

// Generate inline type report
{
    const rustToTs = (p: string) => p.replace(/\.rs$/, ".ts").replace(/_/g, "-");
    const lines: string[] = ["# Inline Type Report", ""];

    // Group by signature to find shared vs unique
    const bySignature = new Map<string, typeof resolvedInlineTypes>();
    for (const t of resolvedInlineTypes) {
        if (!bySignature.has(t.signature)) bySignature.set(t.signature, []);
        bySignature.get(t.signature)!.push(t);
    }

    const sharedSigs = [...bySignature.entries()].filter(([, types]) => types.length > 1)
        .sort((a, b) => b[1].length - a[1].length);
    const uniqueTypes = [...bySignature.entries()].filter(([, types]) => types.length === 1)
        .map(([, types]) => types[0]);

    const overrideNames = new Set(Object.keys(sharedTypeOverrides));

    // Overridden types appear as single entries with the shared name
    const overriddenTypes = resolvedInlineTypes.filter(t => overrideNames.has(t.typeName));
    // Shared sigs that aren't overridden
    const candidateSigs = sharedSigs.filter(([sig]) => {
        const types = bySignature.get(sig)!;
        return !types.some(t => overrideNames.has(t.typeName));
    });

    if (overriddenTypes.length > 0) {
        lines.push(`## Overridden shared types (${overriddenTypes.length})`, "");
        for (const t of overriddenTypes) {
            const [ifaceP, fieldP, sigP] = sharedTypeOverrides[t.typeName];
            const matched = overrideMatchedTypes.get(t.typeName) || [];
            lines.push(`### \`${t.typeName}\` ✅ [${ifaceP}, ${fieldP}, ${sigP}] (${matched.length} matches)`, "");
            for (const m of matched) {
                lines.push(`- ${m.sourceProperty}, ${m.sourceInterface}, ${rustToTs(m.sourceFile || '?')} — ~~\`${m.typeName}\`~~`);
            }
            lines.push("");
        }
    }

    if (candidateSigs.length > 0) {
        // Split into string literal and number literal, sort each alphabetically
        const isNumericSig = (sig: string) => !sig.includes("'");
        const stringSigs = candidateSigs.filter(([sig]) => !isNumericSig(sig)).sort((a, b) => a[0].localeCompare(b[0]));
        const numberSigs = candidateSigs.filter(([sig]) => isNumericSig(sig)).sort((a, b) => a[0].localeCompare(b[0]));

        lines.push(`## Shared signatures without overrides (${candidateSigs.length}, candidates for shared-types.json)`, "");

        if (stringSigs.length > 0) {
            lines.push(`### String literals`, "");
            for (const [sig, types] of stringSigs) {
                lines.push(`#### \`${sig}\` (${types.length} definitions)`, "");
                for (const t of types) {
                    lines.push(`- ${t.sourceProperty}, ${t.sourceInterface}, ${rustToTs(t.sourceFile || '?')}`);
                }
                lines.push("");
            }
        }

        if (numberSigs.length > 0) {
            lines.push(`### Number literals`, "");
            for (const [sig, types] of numberSigs) {
                lines.push(`#### \`${sig}\` (${types.length} definitions)`, "");
                for (const t of types) {
                    lines.push(`- ${t.sourceProperty}, ${t.sourceInterface}, ${rustToTs(t.sourceFile || '?')}`);
                }
                lines.push("");
            }
        }
    }

    uniqueTypes.sort((a, b) => a.signature.localeCompare(b.signature));
    const uniqueNonOverridden = uniqueTypes.filter(t => !overrideNames.has(t.typeName));
    lines.push(`## Unique inline types (${uniqueNonOverridden.length}, single definition)`, "");
    for (const t of uniqueNonOverridden) {
        lines.push(`- \`${t.signature}\` → \`${t.typeName}\` — ${t.sourceProperty}, ${t.sourceInterface}, ${rustToTs(t.sourceFile || '?')}`);
    }

    const reportPath = path.join(path.dirname(GEN_DIR), "..", "reports", "inline-type-report.md");
    fs.mkdirSync(path.dirname(reportPath), { recursive: true });
    fs.writeFileSync(reportPath, lines.join("\n"));
    console.info(`\nInline type report: ${reportPath}`);
    console.info(`  ${overriddenTypes.length} overridden, ${candidateSigs.length} candidates, ${uniqueTypes.length} unique`);
}

if (resolvedInlineTypes.length > 0) {
    console.info(`\nRegistering ${resolvedInlineTypes.length} inline types...`);

    // Set renames on all file structures — applied at write time
    if (typeRenames.size > 0) {
        for (const [, fileStructure] of fileStructures.entries()) {
            for (const [oldName, newName] of typeRenames) {
                fileStructure.typeRenames.set(oldName, newName);
            }
        }
        // Update type dependencies
        for (const [oldName, newName] of typeRenames) {
            for (const [filePath,] of fileStructures.entries()) {
                const deps = typeRegistry.getFileDependencies(filePath);
                if (deps && deps.has(oldName)) {
                    deps.delete(oldName);
                    deps.add(newName);
                }
            }
        }
    }

    // First pass: group by file
    for (const inlineType of resolvedInlineTypes) {
        const targetFile = inlineType.sourceFile || "types/shared.rs"; // fallback
        
        if (!inlineTypesByFile.has(targetFile)) {
            inlineTypesByFile.set(targetFile, []);
        }
        inlineTypesByFile.get(targetFile)!.push(inlineType);
    }

    // Per-struct inline types: each struct gets its own enum, no common extraction needed
    // extractCommonInlineTypesEarly(inlineTypes, inlineTypesByFile);

    // Second pass: detect inline types used across multiple files and move them to shared.rs
    const sharedInlineTypes: typeof inlineTypes = [];
    const crossFileTypes = new Set<string>();

    for (const inlineType of inlineTypes) {
        // Check if this type is used in multiple files
        if (inlineType.usedInFiles && inlineType.usedInFiles.size > 1) {
            crossFileTypes.add(inlineType.typeName);
            console.info(`  → Moving ${inlineType.typeName} to shared.rs (used in ${inlineType.usedInFiles.size} files)`);
        }
    }

    // Move cross-file types to shared.rs
    if (crossFileTypes.size > 0) {
        for (const [file, types] of Array.from(inlineTypesByFile.entries())) {
            const remaining = types.filter(t => !crossFileTypes.has(t.typeName));
            const moved = types.filter(t => crossFileTypes.has(t.typeName));
            
            inlineTypesByFile.set(file, remaining);
            sharedInlineTypes.push(...moved);
        }

        // Add to types/shared.rs
        if (!inlineTypesByFile.has("types/shared.rs")) {
            inlineTypesByFile.set("types/shared.rs", []);
        }
        
        // Deduplicate shared types by name
        const seenNames = new Set<string>();
        for (const sharedType of sharedInlineTypes) {
            if (!seenNames.has(sharedType.typeName)) {
                seenNames.add(sharedType.typeName);
                inlineTypesByFile.get("types/shared.rs")!.push(sharedType);
            }
        }
    }

    // Register inline types with their final locations and add to FileStructures
    for (const [targetFile, types] of inlineTypesByFile.entries()) {
        const modulePath = targetFile.replace(/\.rs$/, '').replace(/\//g, '::').replace(/::mod$/, '');
        const fileStructure = getFileStructure(targetFile);
        
        for (const inlineType of types) {
            const finalModulePath = `crate::${modulePath}::inline`;
            // Re-register the type with its final location
            typeRegistry.registerType(inlineType.typeName, finalModulePath);
            
            // Add to FileStructure
            fileStructure.inlineTypes.push(inlineType);
            
            // If this type was moved to shared.rs and is used in other files,
            // make sure all those files track it as a dependency
            if (targetFile === "types/shared.rs" && inlineType.usedInFiles) {
                for (const usedInFile of inlineType.usedInFiles) {
                    if (usedInFile !== targetFile) {
                        typeRegistry.trackTypeDependency(usedInFile, inlineType.typeName);
                    }
                }
            }
        }
    }
}

// Common types are now extracted earlier in the process (before cross-file consolidation)

// ========================================
// Phase 3: Client Transpilation
// ========================================
console.info(`\n\nTranspiling client classes...`);

const clientTranspiler = new ClientTranspiler(typeRegistry, BybitClientHandlers);
let totalClientsTranspiled = 0;

// Ensure client directory exists in file structures
if (!fs.existsSync(path.join(GEN_DIR, "client"))) {
    fs.mkdirSync(path.join(GEN_DIR, "client"), { recursive: true });
}
directories.add("client");

for (const sourceFile of project.getSourceFiles()) {
    const filePath = sourceFile.getFilePath();
    const sourcePathRelativeToBybitSrc = path.relative(path.join(BYBIT_API_DIR, "src"), filePath);
    
    // Skip node_modules and test files
    if (filePath.includes("node_modules") || filePath.includes(".test.") || filePath.includes(".spec.")) {
        continue;
    }

    // Extract types from client files before processing classes
    const hasClientClass = sourceFile.getClasses().some(c => {
        const name = c.getName();
        return name && name !== 'BaseRestClient' && name !== 'BaseWebsocketClient';
    });
    
    if (hasClientClass) {
        const relativePath = path.relative(BYBIT_API_DIR, filePath);
        const clientTypes = BybitHandlers.extractClientTypes(
            sourceFile,
            typeRegistry,
            typeConverter,
            processedTypes,
            relativePath
        );
        
        // Add extracted types to the client/mod.rs file
        for (const typeData of clientTypes) {
            addTypeToFile(typeData, "client/mod.rs");
        }
    }

    // Look for client classes
    for (const classDecl of sourceFile.getClasses()) {
        const className = classDecl.getName();
        
        // Skip base client classes (we have hand-written implementations)
        if (className === 'BaseRestClient' || className === 'BaseWebsocketClient') {
            console.info(`  Skipping base client class: ${className} (hand-written)`);
            continue;
        }
        
        const rustClient = clientTranspiler.transpileClient(classDecl);
        
        if (rustClient) {
            totalClientsTranspiled++;
            
            // Create a file structure for this client
            const clientFilePath = `client/${rustClient.structName}.rs`;
            const clientFileStructure = getFileStructure(clientFilePath);
            
            // Add imports
            for (const importLine of rustClient.imports) {
                clientFileStructure.imports.add(importLine);
            }
            
            // Check if any method uses WebSocket
            const hasWsMethods = rustClient.methods.some(m => m.body.includes("ws_client"));
            
            // Generate the client struct and impl block
            let clientCode = `\n`;
            
            // Determine if this is a WebSocket client or REST client
            const isRestClient = !rustClient.name.includes("Websocket") && !rustClient.name.includes("WebSocket");
            const isWebsocketAPIClient = rustClient.name === "WebsocketAPIClient";
            
            // Use lifetime parameters and references instead of generic trait bounds
            if (isWebsocketAPIClient) {
                // WebsocketAPIClient wraps WebsocketClient, not a base client
                clientCode += `// Generated client: ${rustClient.name}\n`;
                clientCode += `pub struct ${rustClient.structName}<'a> {\n`;
                clientCode += `    ws_client: &'a WebsocketClient<'a>,\n`;
                clientCode += `}\n\n`;
                clientCode += `impl<'a> ${rustClient.structName}<'a> {\n`;
                clientCode += `    /// Create a new instance of ${rustClient.structName}\n`;
                clientCode += `    pub fn new(ws_client: &'a WebsocketClient<'a>) -> Self {\n`;
                clientCode += `        Self { ws_client }\n`;
                clientCode += `    }\n\n`;
            } else if (isRestClient) {
                // REST API clients own BaseRestClient
                clientCode += `// Generated client: ${rustClient.name}\n`;
                clientCode += `pub struct ${rustClient.structName} {\n`;
                clientCode += `    pub base: BaseRestClient,\n`;
                clientCode += `}\n\n`;
                clientCode += `impl ${rustClient.structName} {\n`;
                clientCode += `    /// Create a new instance of ${rustClient.structName}\n`;
                clientCode += `    pub fn new(base: BaseRestClient) -> Self {\n`;
                clientCode += `        Self { base }\n`;
                clientCode += `    }\n\n`;
            } else {
                // WebSocket clients use BaseWebsocketClient
                clientCode += `// Generated client: ${rustClient.name}\n`;
                clientCode += `pub struct ${rustClient.structName}<'a> {\n`;
                clientCode += `    pub base: &'a BaseWebsocketClient,\n`;
                clientCode += `}\n\n`;
                clientCode += `impl<'a> ${rustClient.structName}<'a> {\n`;
                clientCode += `    /// Create a new instance of ${rustClient.structName}\n`;
                clientCode += `    pub fn new(base: &'a BaseWebsocketClient) -> Self {\n`;
                clientCode += `        Self { base }\n`;
                clientCode += `    }\n\n`;
            }
            
            for (const method of rustClient.methods) {
                // Add doc comments
                for (const doc of method.docs) {
                    if (doc) {
                        const docLines = doc.split('\n');
                        let prevWasListItem = false;
                        for (const line of docLines) {
                            if (line.trim()) {
                                const escapedLine = line.replace(/\\/g, '\\\\').replace(/`/g, '\\`').replace(/\t/g, '  ');
                                const isListItem = /^\s*-\s/.test(escapedLine);
                                if (prevWasListItem && !isListItem) {
                                    clientCode += `    ///\n`;
                                }
                                clientCode += `    /// ${escapedLine}\n`;
                                prevWasListItem = isListItem;
                            }
                        }
                    }
                }
                
                // Generate method signature
                const asyncKeyword = method.isAsync ? "async " : "";
                const paramStr = method.params
                    .map(p => p.isSelf ? p.name : `${p.name}: ${p.type}`)
                    .join(", ");
                
                clientCode += `    pub ${asyncKeyword}fn ${method.name}(${paramStr}) -> ${method.rawReturnType ? method.returnType : `ClientResult<${method.returnType}>`} {\n`;
                clientCode += `        ${method.body}\n`;
                clientCode += `    }\n\n`;
            }
            
            clientCode += `}\n`;
            
            // Add to file structure
            clientFileStructure.mainContent.push({
                name: rustClient.structName,
                content: clientCode,
                category: "struct",
                sourceFile: toBybitSourcePath(sourcePathRelativeToBybitSrc),
            });
            
            console.success(`  ✓ ${rustClient.name} -> ${clientFilePath}`);
        }
    }
}

console.info(`  Transpiled ${totalClientsTranspiled} client classes`);

// Add imports to FileStructures
for (const [rustFilePath, fileStructure] of fileStructures.entries()) {
    const deps = typeRegistry.getFileDependencies(rustFilePath);
    if (deps && deps.size > 0) {
        for (const depType of deps) {
            const depModulePath = typeRegistry.getTypeModulePath(depType);
            if (depModulePath) {
                fileStructure.imports.add(`use ${depModulePath}::${depType};`);
            }
        }
    }
}

// Generate Rust files
console.info(`\n\nGenerating Rust code...`);

let totalTypes = 0;
let totalSkipped = 0;
let totalFiles = 0;

// Write each file using FileStructure
for (const [rustFilePath, fileStructure] of fileStructures.entries()) {
    fileStructure.write(GEN_DIR, typeRegistry);

    const actualTypes = fileStructure.mainContent.length;
    const skippedCount = fileStructure.skippedTypes.length;
    const inlineCount = fileStructure.inlineTypes.length;

    totalTypes += actualTypes + inlineCount;
    totalSkipped += skippedCount;
    totalFiles++;

    if (inlineCount > 0 && skippedCount > 0) {
        console.success(`✓ ${rustFilePath} (${actualTypes} types, ${inlineCount} inline, ${skippedCount} skipped)`);
    } else if (inlineCount > 0) {
        console.success(`✓ ${rustFilePath} (${actualTypes} types, ${inlineCount} inline)`);
    } else if (skippedCount > 0) {
        console.success(`✓ ${rustFilePath} (${actualTypes} types, ${skippedCount} skipped)`);
    } else {
        console.success(`✓ ${rustFilePath} (${actualTypes} types)`);
    }
}

// Generate mod.rs files for each directory
console.info(`\nGenerating module files...`);

const sortedDirs = Array.from(directories).sort();

for (const dir of sortedDirs) {
    const fullDirPath = path.join(GEN_DIR, dir);

    if (!fs.existsSync(fullDirPath)) {
        fs.mkdirSync(fullDirPath, { recursive: true });
    }

    const modules = fileGenerator.discoverModules(dir);

    if (modules.length > 0) {
        fileGenerator.generateModFile(dir, modules);
        
        // For client/mod.rs, append extracted types from the file structure
        if (dir === "client" && fileStructures.has("client/mod.rs")) {
            const clientModTypes = fileStructures.get("client/mod.rs")!;
            if (clientModTypes.mainContent.length > 0) {
                const modPath = path.join(GEN_DIR, "client", "mod.rs");
                let extraContent = "\n// Extracted types from client files\nuse serde::{Deserialize, Serialize};\nuse derive_builder::Builder;\n\n";
                for (const rustType of clientModTypes.mainContent) {
                    extraContent += rustType.content + "\n";
                }
                fs.appendFileSync(modPath, extraContent);
            }
        }
        
        console.success(`✓ ${dir}/mod.rs (${modules.length} modules)`);
    }
}

// Update or create lib.rs
const topLevelModules = fileGenerator.discoverModules("");

fileGenerator.generateLibFile(topLevelModules);
console.success(`✓ lib.rs (${topLevelModules.length} top-level modules)`);

console.success(`\n✨ Generation complete!`);
console.info(`  Total files: ${totalFiles}`);
console.info(`  Total types: ${totalTypes}`);
if (totalSkipped > 0) {
    console.info(`  Skipped types: ${totalSkipped} (see generated files for details)`);
}
console.info(`  Output directory: ${GEN_DIR}`);

// Print all errors and warnings at the end
if (errorCollector.hasErrors() || errorCollector.hasWarnings()) {
    console.info("\n");
    errorCollector.printSummary();

    // Exit with error code if there were errors
    if (errorCollector.hasErrors()) {
        process.exit(1);
    }
}
