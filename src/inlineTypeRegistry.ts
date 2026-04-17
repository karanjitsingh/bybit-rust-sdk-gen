/**
 * Registry for inline/anonymous union types
 * Maps type signatures to generated type names
 */

import { Type, Node } from "ts-morph";
import { makeValidRustIdent } from "./utils";

export interface InlineTypeReference {
    sourceInterface: string;
    sourceProperty: string;
    sourceFile: string;
}

export interface InlineTypeInfo {
    typeName: string;
    signature: string;          // Type-aware key: "'Buy' | 'Sell'" or "0 | 1"
    variants: string[];
    isHeterogeneous?: boolean;  // For unions like string | number | CustomType
    genericParams?: string[];   // For generic inline types like T | string | number
    sourceInterface?: string;   // The interface this inline type was generated from
    sourceProperty?: string;    // The property name that required this inline type
    sourceFile?: string;        // The Rust file where this type should be generated (first occurrence)
    usedInFiles?: Set<string>;  // All Rust files that use this inline type
    references: InlineTypeReference[];  // All definition sites
}

export class InlineTypeRegistry {
    private typeSignatureToName: Map<string, string> = new Map();
    private inlineTypes: Map<string, InlineTypeInfo> = new Map();
    // Maps alias name -> canonical name
    private typeAliases: Map<string, string> = new Map();
    private typeCounter = 0;

    /**
     * Register an inline union type and generate a name for it
     */
    registerInlineUnionType(
        type: Type,
        context: string,
        isHeterogeneous: boolean = false,
        genericParams: string[] = [],
        sourceInterface?: string,
        sourceFile?: string
    ): string | null {
        // Check if it's a union
        if (!type.isUnion()) {
            return null;
        }

        const unionTypes = type.getUnionTypes();
        // Filter out undefined/null but keep generic type parameters
        const nonNullTypes = unionTypes.filter(t => !t.isNull() && !t.isUndefined());

        if (nonNullTypes.length === 0) {
            return null;
        }

        // Separate generic parameters from other types
        const typeParameters = nonNullTypes.filter(t => t.isTypeParameter());
        const concreteTypes = nonNullTypes.filter(t => !t.isTypeParameter());

        // If we have generic parameters, this must be heterogeneous
        if (typeParameters.length > 0) {
            isHeterogeneous = true;
        }

        const allStringLiterals = concreteTypes.every(t => t.isStringLiteral());
        const allNumberLiterals = concreteTypes.every(t => t.isNumberLiteral());
        const allEnumLiterals = concreteTypes.every(t => t.isEnumLiteral());

        // For homogeneous unions (all string literals, number literals, or enum literals) without generics
        if (!isHeterogeneous && !allStringLiterals && !allNumberLiterals && !allEnumLiterals) {
            return null;
        }

        // Check if all enum literals belong to the same enum - if so, don't create inline type
        if (allEnumLiterals && concreteTypes.length > 0) {
            const firstSymbol = concreteTypes[0].getSymbol();
            if (firstSymbol) {
                const parentEnum = firstSymbol.getValueDeclaration()?.getParent();
                // Check if all variants belong to the same enum
                const allSameEnum = concreteTypes.every(t => {
                    const sym = t.getSymbol();
                    if (!sym) return false;
                    const parent = sym.getValueDeclaration()?.getParent();
                    return parent === parentEnum;
                });

                // If they're all from the same enum, return null - use the enum type directly
                if (allSameEnum) {
                    return null;
                }
            }
        }

        // Create a signature for this type (include all types for uniqueness)
        // signatureParts preserve type info: string literals quoted, numbers unquoted
        const signatureParts: string[] = [];
        const variants = nonNullTypes.map(t => {
            if (t.isStringLiteral()) {
                const val = t.getLiteralValue() as string;
                signatureParts.push(`'${val}'`);
                return val;
            }
            if (t.isNumberLiteral()) {
                const val = String(t.getLiteralValue());
                signatureParts.push(val);
                return val;
            }
            if (t.isEnumLiteral()) {
                const symbol = t.getSymbol();
                if (symbol) {
                    const name = symbol.getName();
                    signatureParts.push(name);
                    return name;
                }
            }
            const text = t.getText().replace(/['"]/g, "");
            signatureParts.push(text);
            return text;
        }).sort();
        signatureParts.sort();
        const signature = signatureParts.join(" | ");

        // Track references on the first registered type for reporting
        if (this.typeSignatureToName.has(signature)) {
            const firstTypeName = this.typeSignatureToName.get(signature)!;
            const firstType = this.inlineTypes.get(firstTypeName);
            if (firstType) {
                firstType.references.push({
                    sourceInterface: sourceInterface || "?",
                    sourceProperty: context || "?",
                    sourceFile: sourceFile || "?",
                });
            }
        }

        // Always generate a new per-struct type name
        const typeName = this.generateTypeName(context, variants, sourceInterface);

        // Only track the first type for a signature (for reporting)
        if (!this.typeSignatureToName.has(signature)) {
            this.typeSignatureToName.set(signature, typeName);
        }

        // Store the type info (we'll generate it later)
        const usedInFiles = sourceFile ? new Set([sourceFile]) : undefined;
        this.inlineTypes.set(typeName, {
            typeName,
            signature,
            variants,
            isHeterogeneous,
            genericParams: genericParams.length > 0 ? genericParams : undefined,
            sourceInterface,
            sourceProperty: context,
            sourceFile,
            usedInFiles,
            references: [{
                sourceInterface: sourceInterface || "?",
                sourceProperty: context || "?",
                sourceFile: sourceFile || "?",
            }],
        });

        return typeName;
    }

    /**
     * Generate a descriptive type name from context and variants
     * Format: InterfaceName_FieldName
     */
    private generateTypeName(context: string, variants: string[], sourceInterface?: string): string {
        // If we have sourceInterface, use InterfaceName_FieldName format
        if (sourceInterface) {
            // Sanitize and convert field name to PascalCase
            const sanitizedField = context
                .replace(/['"]/g, '')  // Remove quotes
                .replace(/^(get|set|is|has)/, '')  // Remove common prefixes
                .replace(/[^a-zA-Z0-9_-]/g, '_');  // Replace invalid chars with underscore
            
            const pascalFieldName = sanitizedField
                .split(/[_-]/)
                .filter(word => word.length > 0)
                .map(word => word.charAt(0).toUpperCase() + word.slice(1))
                .join('');
            
            const typeName = `${sourceInterface}_${pascalFieldName}`;
            
            // Check for collisions
            if (!this.isDuplicateName(typeName)) {
                return typeName;
            }
            
            // If collision, add a number suffix
            let counter = 2;
            while (this.isDuplicateName(`${typeName}${counter}`)) {
                counter++;
            }
            return `${typeName}${counter}`;
        }

        // Fallback: Try to find a common prefix or pattern
        const prefix = this.findCommonPrefix(variants);

        if (prefix && prefix.length > 2) {
            // Use prefix-based name
            const baseName = makeValidRustIdent(prefix);
            const typeName = `${baseName}Type`;

            // Check for collisions
            if (!this.isDuplicateName(typeName)) {
                return typeName;
            }
        }

        // Fallback: use context-based name
        const contextPart = makeValidRustIdent(context.replace(/^(get|set|is|has)/, ""));
        const typeName = `${contextPart}Type`;

        if (!this.isDuplicateName(typeName)) {
            return typeName;
        }

        // Final fallback: numbered type
        this.typeCounter++;
        return `InlineType${this.typeCounter}`;
    }

    private findCommonPrefix(strings: string[]): string {
        if (strings.length === 0) return "";

        let prefix = strings[0];
        for (let i = 1; i < strings.length; i++) {
            while (strings[i].indexOf(prefix) !== 0) {
                prefix = prefix.substring(0, prefix.length - 1);
                if (prefix === "") return "";
            }
        }

        // Remove trailing underscores or non-word characters
        return prefix.replace(/[_\W]+$/, "");
    }

    private isDuplicateName(name: string): boolean {
        return Array.from(this.inlineTypes.values()).some(t => t.typeName === name)
            || this.typeAliases.has(name);
    }

    /**
     * Get all registered inline types
     */
    getAllInlineTypes(): InlineTypeInfo[] {
        return Array.from(this.inlineTypes.values());
    }

    /**
     * Get all type aliases: alias name -> canonical name
     */
    getTypeAliases(): Map<string, string> {
        return this.typeAliases;
    }

    /**
     * Check if a type has been registered
     */
    hasInlineType(signature: string): boolean {
        return this.typeSignatureToName.has(signature);
    }

    /**
     * Get the type name for a signature
     */
    getTypeName(signature: string): string | undefined {
        return this.typeSignatureToName.get(signature);
    }

    removeType(typeName: string): void {
        this.inlineTypes.delete(typeName);
    }

    addType(info: InlineTypeInfo): void {
        this.inlineTypes.set(info.typeName, info);
    }
}

