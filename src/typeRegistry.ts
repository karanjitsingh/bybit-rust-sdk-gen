/**
 * Type registry for tracking type definitions and dependencies
 */

export class TypeRegistry {
    private typeToModulePath: Map<string, string>;
    private fileTypeDependencies: Map<string, Set<string>>;

    constructor() {
        this.typeToModulePath = new Map();
        this.fileTypeDependencies = new Map();
    }

    /**
     * Register a type with its module path
     */
    registerType(typeName: string, modulePath: string): void {
        this.typeToModulePath.set(typeName, modulePath);
    }

    /**
     * Check if a type name is registered
     */
    isKnownType(typeName: string): boolean {
        return this.typeToModulePath.has(typeName);
    }

    /**
     * Get the module path for a type
     */
    getTypeModulePath(typeName: string): string | undefined {
        return this.typeToModulePath.get(typeName);
    }

    /**
     * Track a type dependency for a file
     */
    trackTypeDependency(rustFilePath: string, typeName: string): void {
        if (!this.fileTypeDependencies.has(rustFilePath)) {
            this.fileTypeDependencies.set(rustFilePath, new Set());
        }
        this.fileTypeDependencies.get(rustFilePath)!.add(typeName);
    }

    /**
     * Get all type dependencies for a file
     */
    getFileDependencies(rustFilePath: string): Set<string> | undefined {
        return this.fileTypeDependencies.get(rustFilePath);
    }

    /**
     * Get all registered type names
     */
    getAllTypes(): string[] {
        return Array.from(this.typeToModulePath.keys());
    }

    /**
     * Get statistics about registered types
     */
    getStats(): { totalTypes: number; totalFiles: number } {
        return {
            totalTypes: this.typeToModulePath.size,
            totalFiles: this.fileTypeDependencies.size
        };
    }

    /**
     * Clear all registrations (useful for testing)
     */
    clear(): void {
        this.typeToModulePath.clear();
        this.fileTypeDependencies.clear();
    }
}

