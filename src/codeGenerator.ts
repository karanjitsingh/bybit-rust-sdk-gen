/**
 * Rust code generation from TypeScript AST nodes
 */

import { Node, JSDoc } from "ts-morph";
import { StructMember, EnumVariant } from "./types";
import { makeValidRustIdent, escapeRustKeyword, RUST_KEYWORDS, toSnakeCase } from "./utils";
import * as console from "./console";
import * as BybitHandlers from "./bybitHandlers";

/**
 * Convert JSDoc comments to Rust doc comments
 * Returns { docComment: string, isDeprecated: boolean }
 */
function convertJSDocToRust(jsDocs: JSDoc[]): { docComment: string, isDeprecated: boolean } {
    if (jsDocs.length === 0) {
        return { docComment: "", isDeprecated: false };
    }

    let isDeprecated = false;
    const lines: string[] = [];

    for (const jsDoc of jsDocs) {
        const comment = jsDoc.getComment();
        if (!comment) continue;

        // Get the comment text
        const commentText = typeof comment === 'string' ? comment :
            comment.map(part => part.getText()).join('');

        // Check for @deprecated tag
        if (jsDoc.getTags().some(tag => tag.getTagName() === 'deprecated')) {
            isDeprecated = true;
        }

        // Split into lines and convert to Rust doc comments
        const commentLines = commentText.split('\n').map(line => line.trim());
        let prevWasListItem = false;
        for (const line of commentLines) {
            if (line) {
                const cleaned = line.replace(/\t/g, '  ');
                const isListItem = /^\s*-\s/.test(cleaned);
                if (prevWasListItem && !isListItem) {
                    lines.push(`///`);
                }
                lines.push(`/// ${cleaned}`);
                prevWasListItem = isListItem;
            }
        }
    }

    const docComment = lines.length > 0 ? lines.join('\n') + '\n' : '';
    return { docComment, isDeprecated };
}

/**
 * Convert TypeScript type alias to Rust enum (for string literal unions)
 * Returns null if the type alias cannot be converted (not a string literal union)
 * Throws an error if conversion is attempted but fails
 */
export function convertTypeAlias(typeName: string, typeNode: Node, jsDocs: JSDoc[] = [], sourceFile?: string): { code: string | null, skipReason?: string } {
    const typeText = typeNode.getText();

    // Check if it's a union of string literals (enum-like)
    if (Node.isUnionTypeNode(typeNode)) {
        const types = typeNode.getTypeNodes();
        const allStringLiterals = types.every(t => Node.isLiteralTypeNode(t));

        if (allStringLiterals) {
            // Validate we have at least one variant
            if (types.length === 0) {
                throw new Error(`Type alias '${typeName}' is an empty union`);
            }
            // Generate Rust enum for string literal unions
            const variants = types.map(t => {
                const literal = t.getText().replace(/['"]/g, "");

                // Check if the literal is already a valid Rust identifier
                // Valid: alphanumeric + underscores, not starting with a number, not a keyword
                const isValidIdent = /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(literal) && !RUST_KEYWORDS.has(literal);

                let variantName: string;
                if (isValidIdent) {
                    // Use the literal as-is if it's already valid (e.g., "PRINCIPLE_REPAYMENT_INS_LOAN")
                    variantName = literal;
                } else {
                    // Transform to PascalCase for invalid identifiers (e.g., "spot" -> "Spot")
                    variantName = literal
                        .replace(/[-_\.]/g, " ")
                        .split(" ")
                        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
                        .join("");

                    // Make it a valid Rust identifier
                    variantName = makeValidRustIdent(variantName || literal);
                }

                return { name: variantName, value: literal };
            });

            // Deduplicate variants by name
            const seenNames = new Set<string>();
            const uniqueVariants = variants.filter(v => {
                if (seenNames.has(v.name)) {
                    return false;
                }
                seenNames.add(v.name);
                return true;
            });

            // Add JSDoc comments
            const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
            let rustCode = docComment;

            if (isDeprecated) {
                rustCode += `#[deprecated]\n`;
            }

            rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
            rustCode += `pub enum ${typeName} {\n`;

            for (let i = 0; i < uniqueVariants.length; i++) {
                const variant = uniqueVariants[i];
                // Mark first variant as default
                if (i === 0) {
                    rustCode += `    #[default]\n`;
                }
                // Only add rename attributes if the variant name differs from the original value
                if (variant.name !== variant.value) {
                    rustCode += `    #[serde(rename = "${variant.value}")]\n`;
                    rustCode += `    #[strum(serialize = "${variant.value}")]\n`;
                }
                rustCode += `    ${variant.name},\n`;
            }

            rustCode += `}\n`;
            return { code: rustCode };
        } else {
            // It's a union but not all string literals
            // This is not supported - we only convert string literal unions to enums
            const compactTypeText = typeText.replace(/\s+/g, ' ').trim();
            const preview = compactTypeText.length > 100 ? compactTypeText.substring(0, 100) + '...' : compactTypeText;
            console.warn(`Type alias '${typeName}': Union contains non-string-literal types (${preview})\n` +
                `Supported: String literal unions only (e.g., "value1" | "value2")`);
            return {
                code: null,
                skipReason: `Union with non-string-literal types: ${preview}`
            };
        }
    }

    // Check if it's a single literal type (e.g., type APIMarket = 'v5')
    if (Node.isLiteralTypeNode(typeNode)) {
        const literal = typeNode.getText().replace(/['"]/g, "");
        
        // Generate enum with single variant
        const variantName = literal
            .replace(/[-_\.]/g, " ")
            .split(" ")
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join("");
        
        const validVariantName = makeValidRustIdent(variantName || literal);
        
        // Add JSDoc comments
        const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
        let rustCode = docComment;

        if (isDeprecated) {
            rustCode += `#[deprecated]\n`;
        }

        rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
        rustCode += `pub enum ${typeName} {\n`;
        
        // Mark the single variant as default
        rustCode += `    #[default]\n`;
        // Only add rename attribute if the variant name differs from the original value
        if (validVariantName !== literal) {
            rustCode += `    #[serde(rename = "${literal}")]\n`;
            rustCode += `    #[strum(serialize = "${literal}")]\n`;
        }
        rustCode += `    ${validVariantName},\n`;
        rustCode += `}\n`;
        
        return { code: rustCode };
    }

    // Check if it's a simple type alias (e.g., type numberInString = string)
    if (typeText === 'string' || typeText === 'number' || typeText === 'boolean') {
        // Generate struct wrapper
        const innerType = typeText === 'string' ? 'String' : 
                          typeText === 'number' ? 'Number' : 
                          'bool';
        
        // Add JSDoc comments
        const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
        let rustCode = docComment;

        if (isDeprecated) {
            rustCode += `#[deprecated]\n`;
        }

        rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]\n`;
        rustCode += `pub struct ${typeName}(pub ${innerType});\n`;
        
        return { code: rustCode };
    }

    // Check if it's a tuple type
    if (Node.isTupleTypeNode(typeNode)) {
        const elements = typeNode.getElements();

        if (elements.length === 0) {
            throw new Error(`Type alias '${typeName}' is an empty tuple`);
        }

        // Convert each tuple element to Rust type
        const rustTypes: string[] = [];
        for (const element of elements) {
            const elementText = element.getText();
            let rustType: string;

            // Handle common TypeScript types
            if (elementText === 'string') {
                rustType = 'String';
            } else if (elementText === 'number') {
                rustType = 'Number';
            } else if (elementText === 'boolean') {
                rustType = 'bool';
            } else if (elementText.endsWith('[]')) {
                // Array type
                const innerType = elementText.slice(0, -2);
                if (innerType === 'string') {
                    rustType = 'Vec<String>';
                } else if (innerType === 'number') {
                    rustType = 'Vec<Number>';
                } else {
                    rustType = `Vec<${innerType}>`;
                }
            } else {
                // Assume it's a custom type name
                rustType = elementText;
            }

            rustTypes.push(rustType);
        }

        // Add JSDoc comments
        const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
        let rustCode = docComment;

        if (isDeprecated) {
            rustCode += `#[deprecated]\n`;
        }

        // Generate Rust type alias
        rustCode += `pub type ${typeName} = (${rustTypes.join(', ')});\n`;

        return { code: rustCode };
    }

    // Check if it's an object type literal (should be converted to a struct)
    if (Node.isTypeLiteral(typeNode)) {
        try {
            // Get properties from the type literal
            const members: StructMember[] = [];
            const properties = typeNode.getMembers();

            for (const prop of properties) {
                if (Node.isPropertySignature(prop)) {
                    const propName = prop.getName();
                    const propType = prop.getType();
                    const isOptional = prop.hasQuestionToken();

                    // Get JSDoc for this property
                    const propJsDocs = prop.getJsDocs();
                    const { docComment: propDocComment, isDeprecated: propIsDeprecated } = convertJSDocToRust(propJsDocs);

                    // Determine Rust type - we'll need basic type conversion
                    let rustType: string;
                    const propTypeText = prop.getTypeNode()?.getText() || propType.getText();

                    // Basic type conversion
                    if (propTypeText === 'string') {
                        rustType = 'String';
                    } else if (propTypeText === 'number') {
                        rustType = 'Number';
                    } else if (propTypeText === 'boolean') {
                        rustType = 'bool';
                    } else if (propTypeText.endsWith('[]')) {
                        const innerType = propTypeText.slice(0, -2);
                        if (innerType === 'string') {
                            rustType = 'Vec<String>';
                        } else if (innerType === 'number') {
                            rustType = 'Vec<Number>';
                        } else if (innerType === 'boolean') {
                            rustType = 'Vec<bool>';
                        } else {
                            rustType = `Vec<${innerType}>`;
                        }
                    } else if (propTypeText.includes('|')) {
                        // Union type - for now, use serde_json::Value
                        // This will be handled by TypeConverter in parser.ts later
                        rustType = 'serde_json::Value';
                    } else {
                        // Assume it's a custom type
                        rustType = propTypeText;
                    }

                    // Wrap in Option if optional (but avoid double-wrapping if already Option)
                    const alreadyOptional = rustType.startsWith('Option<');
                    if (isOptional && !alreadyOptional) {
                        rustType = `Option<${rustType}>`;
                    }

                    members.push({
                        name: propName,
                        type: rustType,
                        optional: alreadyOptional || isOptional,
                        docComment: propDocComment,
                        isDeprecated: propIsDeprecated
                    });
                }
            }

            if (members.length === 0) {
                throw new Error(`Type alias '${typeName}' is an empty object type`);
            }

            // Convert to Rust struct using the interface converter
            const rustCode = convertInterface(typeName, members, [], jsDocs);
            if (rustCode) {
                return { code: rustCode };
            } else {
                throw new Error(`Failed to generate struct for type alias '${typeName}'`);
            }
        } catch (error) {
            const errorMsg = error instanceof Error ? error.message : String(error);
            console.error(`  ✗ Failed to convert object type alias '${typeName}': ${errorMsg}`);
            return {
                code: null,
                skipReason: `Object type alias conversion failed: ${errorMsg}`
            };
        }
    }

    // Not a union type or object type - categorize and skip with reason
    // Normalize the type text - replace newlines with spaces for compact display
    const compactTypeText = typeText.replace(/\s+/g, ' ').trim();
    const preview = compactTypeText.length > 100 ? compactTypeText.substring(0, 100) + '...' : compactTypeText;

    // Handle generic instantiation aliases: type X = SomeGeneric<A, B, C>
    // These are concrete uses of generic types (e.g., WS event types)
    const genericInstMatch = compactTypeText.match(/^([A-Z]\w*)<(.+)>$/);
    if (genericInstMatch) {
        const [, baseName, argsStr] = genericInstMatch;
        
        // Convert generic args to Rust types (simple mapping)
        const convertGenericArg = (arg: string): string => {
            arg = arg.trim();
            // String literal union like 'delta' | 'snapshot'
            if (arg.includes("'") || arg.includes('"')) {
                return "String";
            }
            if (arg === "string") return "String";
            if (arg === "number") return "Number";
            if (arg === "boolean") return "bool";
            if (arg.endsWith("[]")) {
                const inner = convertGenericArg(arg.slice(0, -2).trim());
                return `Vec<${inner}>`;
            }
            // Union of types like T1 | T2 | T3
            if (arg.includes(" | ")) {
                return "serde_json::Value";
            }
            // Named type — pass through
            return arg;
        };

        // Parse args respecting nested angle brackets
        const args: string[] = [];
        let depth = 0;
        let current = "";
        for (const ch of argsStr) {
            if (ch === '<') depth++;
            else if (ch === '>') depth--;
            else if (ch === ',' && depth === 0) {
                args.push(current.trim());
                current = "";
                continue;
            }
            current += ch;
        }
        if (current.trim()) args.push(current.trim());

        const rustArgs = args.map(convertGenericArg);
        
        const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
        let rustCode = docComment;
        if (isDeprecated) rustCode += `#[deprecated]\n`;
        rustCode += `pub type ${typeName} = ${baseName}<${rustArgs.join(", ")}>;\n`;
        return { code: rustCode };
    }

    // Detect what kind of type alias this is
    let reason: string;

    if (compactTypeText.includes('<') && compactTypeText.match(/^[A-Z]\w*</)) {
        // Generic type alias
        reason = `Generic type alias: ${preview}`;
    } else if (Node.isLiteralTypeNode(typeNode)) {
        // Single literal like 'v5' or 123
        reason = `Single literal: ${preview}`;
    } else if (typeText === 'string' || typeText === 'number' || typeText === 'boolean') {
        // Simple type alias like `type numberInString = string`
        reason = `Simple type alias: ${preview}`;
    } else if (compactTypeText.includes('typeof') || compactTypeText.includes('keyof')) {
        // BYBIT-SPECIFIC: Try to resolve typeof/keyof pattern to enum
        // Pattern: type WsKey = (typeof WS_KEY_MAP)[keyof typeof WS_KEY_MAP]
        const sourceFile = typeNode.getSourceFile();
        const typeAliasDecl = sourceFile.getTypeAlias(typeName);
        
        if (typeAliasDecl) {
            const resolveResult = BybitHandlers.resolveTypeofKeyofPattern(
                typeAliasDecl,
                typeName,
                compactTypeText
            );
            
            if (resolveResult.resolved && resolveResult.values) {
                // Successfully resolved - generate enum
                const rustCode = BybitHandlers.generateEnumFromValues(
                    typeName,
                    resolveResult.values,
                    jsDocs
                );
                return { code: rustCode };
            } else if (resolveResult.error) {
                console.warn(`  ⚠️  Type alias '${typeName}': ${resolveResult.error}`);
            }
        }
        
        // Fallback: couldn't resolve the typeof/keyof pattern, use String wrapper
        console.warn(`  ⚠️  Type alias '${typeName}': typeof/keyof expression could not be resolved - using String wrapper`);
        
        // Generate struct wrapper with String
        const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
        let rustCode = docComment;

        if (isDeprecated) {
            rustCode += `#[deprecated]\n`;
        }

        rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]\n`;
        rustCode += `pub struct ${typeName}(pub String);\n`;
        
        return { code: rustCode };
    } else {
        // Unknown/complex type
        reason = `Complex expression: ${preview}`;
    }

    console.warn(`  ⊘  Type alias '${typeName}': Not a union type (= ${preview})\n` +
        `Note: ${reason}`);
    return { code: null, skipReason: reason };
}

/**
 * Convert TypeScript interface to Rust struct
 * Returns null if the interface cannot be converted (e.g., empty interface)
 * Throws an error if conversion is attempted but fails
 */
export function convertInterface(interfaceName: string, members: StructMember[], generics: string[] = [], jsDocs: JSDoc[] = []): string | null {
    if (members.length === 0) {
        console.warn(`Interface '${interfaceName}': Empty interface (no properties)`);
        console.warn(`Note: Empty interfaces are often used as type markers in TypeScript`);
        return null;
    }

    // Add JSDoc comments
    const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
    let rustCode = docComment;

    if (isDeprecated) {
        rustCode += `#[deprecated]\n`;
    }

    // // Check if struct has optional fields (for derive_builder support)
    // const hasOptionalFields = members.some(m => m.optional);
    
    // Generic structs need Default trait bounds when deriving Default
    const isGeneric = generics.length > 0;
    const shouldDeriveDefault = true;

    // // Add derive macros
    // if (hasOptionalFields) {
    rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]\n`;
    rustCode += `#[builder(setter(into, strip_option), default)]\n`;
    // } else {
    //     rustCode += `#[derive(Debug, Clone, Serialize, Deserialize)]\n`;
    // }

    // Add generic parameters if present
    // First, filter out generics that aren't actually used in any field type
    let usedGenerics = generics;
    if (generics.length > 0) {
        const allFieldTypes = members.map(m => m.type).join(" ");
        usedGenerics = generics.filter(g => {
            const paramName = g.includes('=') ? g.split('=')[0].trim() : g.trim();
            return new RegExp(`\\b${paramName}\\b`).test(allFieldTypes);
        });
    }

    if (usedGenerics.length > 0) {
        // If we're deriving Default, add Default trait bounds to generic parameters
        let processedGenerics = usedGenerics;
        if (shouldDeriveDefault) {
            processedGenerics = usedGenerics.map(g => {
                // Check if generic already has a default value (e.g., "T = String")
                if (g.includes('=')) {
                    const parts = g.split('=');
                    const paramName = parts[0].trim();
                    const defaultValue = parts[1].trim();
                    // Add Default trait bound
                    return `${paramName}: Default = ${defaultValue}`;
                } else {
                    // Just the parameter name, add Default trait bound
                    return `${g.trim()}: Default`;
                }
            });
        }
        rustCode += `pub struct ${interfaceName}<${processedGenerics.join(", ")}> {\n`;
    } else {
        rustCode += `pub struct ${interfaceName} {\n`;
    }

    for (const member of members) {
        const memberName = member.name;
        const memberType = member.type;
        const isOptional = member.optional;

        // Add member doc comment if present
        if (member.docComment) {
            rustCode += member.docComment;
        }

        // Add deprecated attribute if needed
        if (member.isDeprecated) {
            rustCode += `    #[deprecated]\n`;
        }

        // Start with the original name
        let rustFieldName = memberName;
        let needsRename = false;

        // Convert camelCase to snake_case, but only for multi-char names with mixed case
        // Skip single chars and names that are already snake_case or UPPER_CASE
        if (memberName.length > 1 && /[a-z][A-Z]/.test(memberName)) {
            const snakeName = toSnakeCase(memberName);
            if (snakeName !== memberName) {
                rustFieldName = snakeName;
                needsRename = true;
            }
        }

        // Only transform if it contains invalid Rust identifier characters
        if (/[^a-zA-Z0-9_]/.test(rustFieldName)) {
            rustFieldName = makeValidRustIdent(rustFieldName);
            needsRename = true;
        }
        // Or if it starts with a number
        else if (/^[0-9]/.test(rustFieldName)) {
            rustFieldName = makeValidRustIdent(rustFieldName);
            needsRename = true;
        }
        // Or if it's a Rust keyword
        else if (RUST_KEYWORDS.has(rustFieldName)) {
            rustFieldName = `r#${rustFieldName}`;
            needsRename = true;
        }

        // Only add serde rename if we had to change the name
        if (needsRename) {
            rustCode += `    #[serde(rename = "${memberName}")]\n`;
        }

        // Check if the type is already wrapped in Option to avoid double-wrapping
        const alreadyOptional = memberType.startsWith('Option<');
        
        // Remove unnecessary parentheses around single types (but keep tuple syntax)
        let cleanedType = memberType;
        if (memberType.startsWith('(') && memberType.endsWith(')') && !memberType.includes(',') && memberType !== '()') {
            cleanedType = memberType.slice(1, -1);
        }
        
        if (isOptional && !alreadyOptional) {
            rustCode += `    pub ${rustFieldName}: Option<${cleanedType}>,\n`;
        } else {
            rustCode += `    pub ${rustFieldName}: ${cleanedType},\n`;
        }
    }

    rustCode += `}\n`;

    // Add a convenience ::builder() associated function
    if (usedGenerics.length > 0) {
        // In impl blocks, generic params cannot have defaults
        const implParams = usedGenerics.map(g => {
            const paramName = g.includes('=') ? g.split('=')[0].trim() : g.trim();
            return `${paramName}: Default + Clone`;
        });
        const paramNames = usedGenerics.map(g => g.includes('=') ? g.split('=')[0].trim() : g.trim());
        rustCode += `\nimpl<${implParams.join(", ")}> ${interfaceName}<${paramNames.join(", ")}> {\n`;
        rustCode += `    pub fn builder() -> ${interfaceName}Builder<${paramNames.join(", ")}> {\n`;
        rustCode += `        ${interfaceName}Builder::default()\n`;
        rustCode += `    }\n`;
        rustCode += `}\n`;
    } else {
        rustCode += `\nimpl ${interfaceName} {\n`;
        rustCode += `    pub fn builder() -> ${interfaceName}Builder {\n`;
        rustCode += `        ${interfaceName}Builder::default()\n`;
        rustCode += `    }\n`;
        rustCode += `}\n`;
    }

    return rustCode;
}

/**
 * Convert TypeScript enum to Rust enum
 * Throws an error if conversion fails
 */
export function convertEnum(enumName: string, members: Array<{ name: string, value?: number | string }>, jsDocs: JSDoc[] = []): string {
    if (members.length === 0) {
        throw new Error(`Enum '${enumName}' has no members`);
    }

    // Add JSDoc comments
    const { docComment, isDeprecated } = convertJSDocToRust(jsDocs);
    let rustCode = docComment;

    if (isDeprecated) {
        rustCode += `#[deprecated]\n`;
    }

    rustCode += `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, EnumString, Display)]\n`;
    rustCode += `#[serde(rename_all = "UPPERCASE")]\n`;
    rustCode += `pub enum ${enumName} {\n`;

    for (let i = 0; i < members.length; i++) {
        const member = members[i];
        let memberName = member.name;
        const value = member.value;

        // Make the member name a valid Rust identifier
        const validMemberName = makeValidRustIdent(memberName);

        // Mark first variant as default
        if (i === 0) {
            rustCode += `    #[default]\n`;
        }

        // Add serde rename if the name was changed
        if (validMemberName !== memberName) {
            rustCode += `    #[serde(rename = "${memberName}")]\n`;
        }

        if (value !== undefined) {
            rustCode += `    ${validMemberName} = ${value},\n`;
        } else {
            rustCode += `    ${validMemberName},\n`;
        }
    }

    rustCode += `}\n`;
    return rustCode;
}
