/**
 * Type definitions for the parser
 */

export interface RustType {
    name: string;
    content: string;
    category: "enum" | "struct" | "type_alias" | "const" | "skipped";
    sourceFile: string;
    skipReason?: string; // For skipped types, why it was skipped
}

export interface StructMember {
    name: string;
    type: string;
    optional: boolean;
    docComment?: string;
    isDeprecated?: boolean;
}

export interface EnumVariant {
    name: string;
    value: string;
}

