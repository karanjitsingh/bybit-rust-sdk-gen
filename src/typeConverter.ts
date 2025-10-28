/**
 * TypeScript to Rust type conversion logic
 */

import { Type, SyntaxKind } from "ts-morph";
import { TypeRegistry } from "./typeRegistry";
import { InlineTypeRegistry } from "./inlineTypeRegistry";
import * as console from "./console";

// Known external/library types that should become serde_json::Value
const EXTERNAL_TYPES = ["WebSocket", "RestClientOptions", "EventEmitter"];

export class TypeConverter {
    // Store generated inline object structs
    private inlineObjects: Map<string, { name: string; fields: Array<{ name: string; type: string; optional: boolean }> }> = new Map();

    constructor(
        private registry: TypeRegistry,
        private inlineRegistry: InlineTypeRegistry
    ) { }

    /**
     * Convert a TypeScript type to its Rust equivalent
     */
    convert(type: Type, isOptional: boolean = false, fieldName: string = "", genericsContext: string[] = [], sourceInterface?: string, rustFilePath?: string): string {
        const typeText = type.getText();

        // Handle primitives
        if (type.isString() || type.isStringLiteral()) return "String";
        if (type.isNumber() || type.isNumberLiteral()) return "f64";
        if (type.isBoolean() || type.isBooleanLiteral()) return "bool";
        if (typeText === "null" || typeText === "undefined") return "()";
        if (typeText === "any") {
            console.warn(`  ⚠️  Field '${fieldName}' has 'any' type - using serde_json::Value`);
            return "serde_json::Value";
        }

        // Handle arrays
        if (type.isArray()) {
            const arrayType = type.getArrayElementType();
            if (arrayType) {
                return `Vec<${this.convert(arrayType, false, fieldName, genericsContext, sourceInterface, rustFilePath)}>`;
            }
            throw new Error(`Array type without element type information. Cannot convert to Rust.`);
        }

        // Handle unions
        if (type.isUnion()) {
            const unionTypes = type.getUnionTypes();
            const nonNullTypes = unionTypes.filter(t => !t.isNull() && !t.isUndefined());

            // Special case: if the union includes undefined/null, it's optional
            const hasUndefined = unionTypes.some(t => t.isUndefined() || t.isNull());

            // Special case: boolean | undefined becomes true | false | undefined
            // Check if all non-null types are boolean literals (true/false)
            const allBooleanLiterals = nonNullTypes.length === 2 &&
                nonNullTypes.every(t => t.isBooleanLiteral());

            if (allBooleanLiterals) {
                // This is really just `boolean` (or `boolean | undefined`)
                if (hasUndefined && !isOptional) {
                    return "Option<bool>";
                }
                if (isOptional) {
                    return "bool";
                }
                return hasUndefined ? "Option<bool>" : "bool";
            }

            // If it's `type | undefined` or `type | null`, treat as Option<type>
            if (nonNullTypes.length === 1) {
                try {
                    const innerType = this.convert(nonNullTypes[0], false, fieldName, genericsContext, sourceInterface, rustFilePath);

                    // If union has undefined/null and not already marked optional, wrap in Option
                    if (hasUndefined && !isOptional) {
                        return `Option<${innerType}>`;
                    }

                    // Already marked optional from question mark, just return inner type
                    if (isOptional) {
                        return innerType;
                    }

                    // No undefined in union, wrap in Option anyway (union with single non-null type)
                    return `Option<${innerType}>`;
                } catch (innerError) {
                    // If we can't convert the inner type, throw a more informative error
                    throw new Error(`Failed to convert union type ${typeText}: ${innerError instanceof Error ? innerError.message : String(innerError)}`);
                }
            }

            // No non-null types means it's just null/undefined (shouldn't happen but handle it)
            if (nonNullTypes.length === 0) {
                return "()";
            }

            // Check if any of the union types are generic parameters
            const hasGenericParameter = nonNullTypes.some(t => {
                const tText = t.getText();
                return genericsContext.includes(tText);
            });

            // If we have a union that includes a generic parameter mixed with other types,
            // create a generic inline enum
            if (hasGenericParameter && nonNullTypes.length > 1) {
                // Extract generic parameters from the union
                const genericParams = nonNullTypes
                    .filter(t => genericsContext.includes(t.getText()))
                    .map(t => t.getText());

                // Create a generic inline type
                const inlineTypeName = this.inlineRegistry.registerInlineUnionType(type, fieldName, true, genericParams, sourceInterface, rustFilePath);
                if (inlineTypeName) {
                    // DON'T register as a known type here - it will be registered later
                    // with its final location (either in the same file's inline module or in shared.rs)
                    
                    // But DO track it as a dependency so imports are generated correctly
                    if (rustFilePath) {
                        this.registry.trackTypeDependency(rustFilePath, inlineTypeName);
                    }

                    // Use the inline type with its generic parameters
                    const genericArgs = genericParams.join(", ");
                    const fullTypeName = `${inlineTypeName}<${genericArgs}>`;

                    if (hasUndefined && !isOptional) {
                        return `Option<${fullTypeName}>`;
                    }
                    if (isOptional) {
                        return fullTypeName;
                    }
                    return hasUndefined ? `Option<${fullTypeName}>` : fullTypeName;
                }

                // Fallback if registration failed
                console.warn(`  ⚠️  Field '${fieldName}' has union with generic parameter (${typeText}) - using serde_json::Value`);
                return "serde_json::Value";
            }

            // If the only non-null type is a generic parameter, use it directly
            if (hasGenericParameter && nonNullTypes.length === 1) {
                const genericType = nonNullTypes[0].getText();
                if (hasUndefined && !isOptional) {
                    return `Option<${genericType}>`;
                }
                if (isOptional) {
                    return genericType;
                }
                return hasUndefined ? `Option<${genericType}>` : genericType;
            }

            // Check if all are string literals or enum literals - register as inline type
            const allStringLiterals = nonNullTypes.every(t => t.isStringLiteral());
            const allEnumLiterals = nonNullTypes.every(t => t.isEnumLiteral());

            if (allEnumLiterals && nonNullTypes.length > 0) {
                // Check if all enum literals belong to the same enum
                const firstSymbol = nonNullTypes[0].getSymbol();
                if (firstSymbol) {
                    const valueDecl = firstSymbol.getValueDeclaration();
                    const parentEnumDecl = valueDecl?.getParent();
                    if (parentEnumDecl && parentEnumDecl.getKind() === SyntaxKind.EnumDeclaration) {
                        const allSameEnum = nonNullTypes.every(t => {
                            const sym = t.getSymbol();
                            if (!sym) return false;
                            const parent = sym.getValueDeclaration()?.getParent();
                            return parent === parentEnumDecl;
                        });

                        if (allSameEnum) {
                            // Get the enum name
                            const enumSymbol = parentEnumDecl.getSymbol();
                            if (enumSymbol) {
                                const enumName = enumSymbol.getName();
                                // Use the enum type directly
                                if (hasUndefined && !isOptional) {
                                    return `Option<${enumName}>`;
                                }
                                if (isOptional) {
                                    return enumName;
                                }
                                return hasUndefined ? `Option<${enumName}>` : enumName;
                            }
                        }
                    }
                }

                // If not all from same enum, try to create inline type
                const inlineTypeName = this.inlineRegistry.registerInlineUnionType(type, fieldName, false, [], sourceInterface, rustFilePath);
                if (inlineTypeName) {
                    // DON'T register as a known type here - it will be registered later
                    // with its final location (either in the same file's inline module or in shared.rs)
                    
                    // But DO track it as a dependency so imports are generated correctly
                    if (rustFilePath) {
                        this.registry.trackTypeDependency(rustFilePath, inlineTypeName);
                    }
                    
                    if (hasUndefined) {
                        return `Option<${inlineTypeName}>`;
                    }
                    return inlineTypeName;
                }

                // If registerInlineUnionType returned null, it means enum literals are from same enum
                // This shouldn't happen if the above check worked, but handle as fallback
                console.warn(`  ⚠️  Enum literal union '${fieldName}' could not be resolved to enum name - using serde_json::Value`);
                return "serde_json::Value";
            }

            if (allStringLiterals && nonNullTypes.length > 0) {
                const inlineTypeName = this.inlineRegistry.registerInlineUnionType(type, fieldName, false, [], sourceInterface, rustFilePath);
                if (inlineTypeName) {
                    // DON'T register as a known type here - it will be registered later
                    // with its final location (either in the same file's inline module or in shared.rs)
                    // This prevents the type from being registered multiple times with different module paths
                    
                    // But DO track it as a dependency so imports are generated correctly
                    if (rustFilePath) {
                        this.registry.trackTypeDependency(rustFilePath, inlineTypeName);
                    }

                    // If union also has undefined, make it optional
                    if (hasUndefined) {
                        return `Option<${inlineTypeName}>`;
                    }
                    return inlineTypeName;
                }
            }

            // Check if all are number literals - register as inline type
            const allNumberLiterals = nonNullTypes.every(t => t.isNumberLiteral());
            if (allNumberLiterals && nonNullTypes.length > 0) {
                // For number literals, create an enum with numeric discriminants
                const variants = nonNullTypes.map(t => t.getText());
                const signature = variants.sort().join("|");

                let inlineTypeName = this.inlineRegistry.getTypeName(signature);
                if (!inlineTypeName) {
                    inlineTypeName = this.inlineRegistry.registerInlineUnionType(type, fieldName, false, [], sourceInterface, rustFilePath);
                }

                if (inlineTypeName) {
                    // DON'T register as a known type here - it will be registered later
                    // with its final location (either in the same file's inline module or in shared.rs)
                    
                    // But DO track it as a dependency so imports are generated correctly
                    if (rustFilePath) {
                        this.registry.trackTypeDependency(rustFilePath, inlineTypeName);
                    }

                    // If union also has undefined, make it optional
                    if (hasUndefined) {
                        return `Option<${inlineTypeName}>`;
                    }
                    return inlineTypeName;
                }
            }

            // For complex heterogeneous unions (string | number | Type), create a Rust enum
            const inlineTypeName = this.inlineRegistry.registerInlineUnionType(type, fieldName, true, [], sourceInterface, rustFilePath);
            if (inlineTypeName) {
                // DON'T register as a known type here - it will be registered later
                // with its final location (either in the same file's inline module or in shared.rs)
                
                // But DO track it as a dependency so imports are generated correctly
                if (rustFilePath) {
                    this.registry.trackTypeDependency(rustFilePath, inlineTypeName);
                }

                if (hasUndefined && !isOptional) {
                    return `Option<${inlineTypeName}>`;
                }
                if (isOptional) {
                    return inlineTypeName;
                }
                return hasUndefined ? `Option<${inlineTypeName}>` : inlineTypeName;
            }

            // Failed to create inline type for heterogeneous union
            throw new Error(`Failed to create inline type for heterogeneous union: ${typeText}`);
        }

        // Handle objects
        if (type.isObject()) {
            const objTypeText = type.getText();

            // Handle function types - these can't be serialized, skip them
            if (objTypeText.includes("=>") || objTypeText.includes("function")) {
                console.warn(`  ⚠️  Field '${fieldName}' is a function type - using serde_json::Value`);
                return "serde_json::Value";
            }

            // Check for index signature types (e.g., { [key: string]: T })
            // Try to get the type's properties to detect index signatures
            const properties = type.getProperties();
            const stringIndexType = type.getStringIndexType();
            const numberIndexType = type.getNumberIndexType();
            
            // If it has an index signature but no regular properties, it's a pure index signature type
            if ((stringIndexType || numberIndexType) && properties.length === 0) {
                if (stringIndexType) {
                    // String index signature: { [key: string]: T }
                    const valueRustType = this.convert(stringIndexType, false, fieldName, genericsContext, sourceInterface, rustFilePath);
                    return `indexmap::IndexMap<String, ${valueRustType}>`;
                } else if (numberIndexType) {
                    // Number index signature: { [key: number]: T }
                    const valueRustType = this.convert(numberIndexType, false, fieldName, genericsContext, sourceInterface, rustFilePath);
                    return `indexmap::IndexMap<i64, ${valueRustType}>`;
                }
            }

            // Check for Record<K, V> utility type
            if (objTypeText.startsWith("Record<")) {
                // Extract key and value types
                const match = objTypeText.match(/Record<([^,]+),\s*([^>]+)>/);
                if (match) {
                    const keyType = match[1].trim();
                    const valueType = match[2].trim();

                    // Use IndexMap for string-keyed records to maintain insertion order
                    if (keyType === "string") {
                        if (valueType === "string") {
                            return "indexmap::IndexMap<String, String>";
                        } else {
                            return "indexmap::IndexMap<String, serde_json::Value>";
                        }
                    }
                    throw new Error(`Record type with non-string key '${keyType}' is not supported. Consider using a HashMap or custom type.`);
                }
            }

            // Check for tuple types
            if (type.isTuple()) {
                const tupleElements = type.getTupleElements();
                if (tupleElements.length > 0) {
                    const rustTypes = tupleElements.map(t => this.convert(t, false, fieldName, genericsContext, sourceInterface, rustFilePath));
                    return `(${rustTypes.join(", ")})`;
                }
                // Empty tuple [] - treat as empty vector
                return "Vec<serde_json::Value>";
            }

            const symbol = type.getSymbol();
            if (symbol) {
                const name = symbol.getName();
                if (name && name !== "__type") {
                    // Check for known external types
                    if (EXTERNAL_TYPES.includes(name)) {
                        return "serde_json::Value";
                    }

                    // Check if this is a type we're generating
                    if (this.registry.isKnownType(name)) {
                        return name;
                    }

                    // Unknown type - check if it contains import() which means it's a type reference
                    if (objTypeText.includes("import(")) {
                        // Extract the actual type name from import statement
                        // e.g., import("/path/to/shared-v5").PositionIdx -> PositionIdx
                        const match = objTypeText.match(/\)\.(\w+)/);
                        if (match && match[1]) {
                            const extractedTypeName = match[1];
                            if (this.registry.isKnownType(extractedTypeName)) {
                                return extractedTypeName;
                            }
                            throw new Error(`Type '${extractedTypeName}' extracted from import but not found in registry.`);
                        }
                    }

                    // Unknown type - throw error instead of silently converting
                    throw new Error(`Unknown type '${name}'. Type not found in registry.`);
                }
            }

            // Anonymous inline object type (e.g., { code: number; msg: string })
            // Generate a struct for it on the fly
            console.debug(`  → Anonymous inline object in field '${fieldName}' - generating struct`);
            
            // Generate a unique type name based on the context
            const generatedTypeName = this.generateInlineObjectTypeName(fieldName, sourceInterface);
            
            // Register the inline object struct for later generation
            this.registerInlineObjectType(type, generatedTypeName, fieldName, genericsContext, sourceInterface, rustFilePath);
            
            // Register this type in the registry so it can be referenced
            // We'll generate it inline in the same file
            return generatedTypeName;
        }

        // Handle any/unknown - use serde_json::Value with a warning
        if (type.getText() === "any" || type.getText() === "unknown") {
            console.warn(`  ⚠️  Field '${fieldName}' has 'any' or 'unknown' type - using serde_json::Value`);
            return "serde_json::Value";
        }

        // Handle type parameters (generics)
        if (type.isTypeParameter()) {
            const paramName = type.getText();

            // If this is a known generic parameter (passed from interface), use it directly
            if (genericsContext.includes(paramName)) {
                return paramName;
            }

            // Check if it's a constraint on an array type (e.g., T extends unknown[])
            const constraint = type.getConstraint();
            if (constraint) {
                // If the constraint is an array, use the generic parameter directly
                // This handles cases like T extends unknown[] where T should map to Vec<T>
                if (constraint.isArray() || constraint.getText().includes('[]')) {
                    return paramName;
                }

                try {
                    return this.convert(constraint, isOptional, fieldName, genericsContext, sourceInterface, rustFilePath);
                } catch (e) {
                    // If we can't convert the constraint, just use the parameter name
                    console.warn(`  ⚠️  Using generic parameter '${paramName}' directly (constraint conversion failed)`);
                    return paramName;
                }
            }

            // No constraint, use the parameter name directly
            console.warn(`  ⚠️  Using generic parameter '${paramName}' without constraint`);
            return paramName;
        }

        // Default to the original type name
        const cleaned = typeText.replace(/\s*\|\s*undefined/g, "");

        // If it contains import() statements, try to extract the type
        if (cleaned.includes("import(")) {
            const match = cleaned.match(/\)\.(\w+)/);
            if (match && match[1]) {
                const extractedTypeName = match[1];
                if (this.registry.isKnownType(extractedTypeName)) {
                    return extractedTypeName;
                }
                throw new Error(`Type '${extractedTypeName}' in import statement not found in registry.`);
            }
            throw new Error(`Complex import statement '${cleaned}' cannot be parsed.`);
        }

        // If it looks like a type parameter (single letter or starts with T), throw error
        if (/^[A-Z]$/.test(cleaned) || /^T[A-Z]/.test(cleaned)) {
            throw new Error(`Type parameter '${cleaned}' detected. Generic types need special handling.`);
        }

        // Check for known external types - these should use serde_json::Value but warn
        if (EXTERNAL_TYPES.includes(cleaned)) {
            console.warn(`  ⚠️  External library type '${cleaned}' - using serde_json::Value`);
            return "serde_json::Value";
        }
        
        // Handle never type
        if (cleaned === 'never') {
            return '()';
        }

        // If it's not a known type we're generating, throw error
        if (!this.registry.isKnownType(cleaned)) {
            throw new Error(`Unknown type '${cleaned}'. Type not found in registry.`);
        }

        return cleaned;
    }

    /**
     * Extract type references from a TypeScript type
     */
    extractTypeReferences(type: Type): string[] {
        const refs: string[] = [];
        const typeText = type.getText();

        // Don't extract from primitives or serde_json::Value
        if (type.isString() || type.isNumber() || type.isBoolean() ||
            typeText === "any" || typeText === "unknown") {
            return refs;
        }

        const symbol = type.getSymbol();
        if (symbol) {
            const name = symbol.getName();
            if (name && name !== "__type" && this.registry.isKnownType(name)) {
                refs.push(name);
            }
        }

        // Check for array element types
        if (type.isArray()) {
            const arrayType = type.getArrayElementType();
            if (arrayType) {
                refs.push(...this.extractTypeReferences(arrayType));
            }
        }

        // Check union types
        if (type.isUnion()) {
            const unionTypes = type.getUnionTypes();
            for (const t of unionTypes) {
                refs.push(...this.extractTypeReferences(t));
            }
        }

        return refs;
    }

    /**
     * Generate a unique type name for an inline object
     * Format: InterfaceName_FieldName
     */
    private generateInlineObjectTypeName(fieldName: string, sourceInterface?: string): string {
        // Remove quotes and other invalid characters, then convert to PascalCase
        const sanitized = fieldName
            .replace(/['"]/g, '')  // Remove quotes
            .replace(/[^a-zA-Z0-9_-]/g, '_');  // Replace invalid chars with underscore
        
        const pascalFieldName = sanitized
            .split(/[_-]/)
            .filter(word => word.length > 0)  // Remove empty strings
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join('');
        
        if (sourceInterface) {
            return `${sourceInterface}_${pascalFieldName}`;
        }
        return `${pascalFieldName}Object`;
    }

    /**
     * Register an inline object type for later generation
     */
    private registerInlineObjectType(type: Type, typeName: string, fieldName: string, genericsContext: string[], sourceInterface?: string, rustFilePath?: string): void {
        // Skip if already registered
        if (this.inlineObjects.has(typeName)) {
            return;
        }

        const fields: Array<{ name: string; type: string; optional: boolean }> = [];

        // Get properties from the type
        const properties = type.getProperties();
        for (const prop of properties) {
            const propName = prop.getName();
            const propType = prop.getTypeAtLocation(prop.getValueDeclarationOrThrow());
            
            // Check if property is optional
            const declarations = prop.getDeclarations();
            let isOptional = false;
            if (declarations.length > 0) {
                const decl = declarations[0];
                if ('hasQuestionToken' in decl && typeof decl.hasQuestionToken === 'function') {
                    isOptional = decl.hasQuestionToken();
                }
            }

            // Convert the property type
            // IMPORTANT: Pass typeName as sourceInterface so nested inline types get proper names
            // e.g., MovePositionParamsV5_List_Category instead of MovePositionParamsV5_Category
            const rustType = this.convert(propType, isOptional, propName, genericsContext, typeName, rustFilePath);

            // Extract and track type dependencies from this field
            if (rustFilePath) {
                const typeRefs = this.extractTypeReferences(propType);
                for (const typeRef of typeRefs) {
                    this.registry.trackTypeDependency(rustFilePath, typeRef);
                }
                
                // Also check if rustType directly contains a known type reference
                // Extract type from Option<T>, Vec<T>, or IndexMap<K, V> wrappers
                const typePattern = /(?:Option|Vec|IndexMap)<([^<>]+(?:<[^<>]+>)?)>/g;
                let match;
                while ((match = typePattern.exec(rustType)) !== null) {
                    const innerType = match[1].trim();
                    // Split by comma for IndexMap<K, V>
                    const types = innerType.split(',').map(t => t.trim());
                    for (const t of types) {
                        // Remove any additional wrapper types
                        const cleanType = t.replace(/^(?:Option|Vec|IndexMap)<(.+)>$/, '$1');
                        if (this.registry.isKnownType(cleanType)) {
                            this.registry.trackTypeDependency(rustFilePath, cleanType);
                        }
                    }
                }
                
                // Also check for direct type references (not wrapped)
                if (this.registry.isKnownType(rustType)) {
                    this.registry.trackTypeDependency(rustFilePath, rustType);
                }
            }

            fields.push({
                name: propName,
                type: rustType,
                optional: isOptional
            });
        }

        this.inlineObjects.set(typeName, { name: typeName, fields });
    }

    /**
     * Get all registered inline object types
     */
    getInlineObjects(): Map<string, { name: string; fields: Array<{ name: string; type: string; optional: boolean }> }> {
        return this.inlineObjects;
    }

    /**
     * Clear inline objects (useful between file generations)
     */
    clearInlineObjects(): void {
        this.inlineObjects.clear();
    }

    /**
     * Convert a Rust file path to a module path
     * e.g., "types/request/account_asset.rs" -> "crate::types::request::account_asset"
     */
    private rustFileToModulePath(rustFilePath: string): string {
        // Remove .rs extension
        let modulePath = rustFilePath.replace(/\.rs$/, '');
        
        // Replace slashes with ::
        modulePath = modulePath.replace(/\//g, '::');
        
        // Add crate:: prefix
        return `crate::${modulePath}`;
    }
}

