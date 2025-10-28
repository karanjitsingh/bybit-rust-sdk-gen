/**
 * Manages imports and dependencies for Rust code generation
 * Provides a structural approach to tracking what types need to be imported
 */

export interface ImportInfo {
    typeName: string;
    modulePath: string;
}

export class ImportManager {
    private imports: Map<string, Set<string>> = new Map(); // file -> set of type names
    private typeToModule: Map<string, string> = new Map(); // type -> module path

    /**
     * Register that a type is defined in a module
     */
    registerTypeDefinition(typeName: string, modulePath: string): void {
        this.typeToModule.set(typeName, modulePath);
    }

    /**
     * Register that a file uses a type (creates import dependency)
     */
    registerTypeUsage(filePath: string, typeName: string): void {
        if (!this.imports.has(filePath)) {
            this.imports.set(filePath, new Set());
        }
        this.imports.get(filePath)!.add(typeName);
    }

    /**
     * Get all imports needed for a file
     * Returns array of ImportInfo with module paths
     */
    getImportsForFile(filePath: string, currentModulePath: string): ImportInfo[] {
        const typeNames = this.imports.get(filePath) || new Set();
        const imports: ImportInfo[] = [];

        for (const typeName of typeNames) {
            const modulePath = this.typeToModule.get(typeName);
            if (modulePath && modulePath !== currentModulePath) {
                imports.push({ typeName, modulePath });
            }
        }

        return imports.sort((a, b) => a.modulePath.localeCompare(b.modulePath));
    }

    /**
     * Generate Rust import statements for a file
     */
    generateImportStatements(filePath: string, currentModulePath: string): string {
        const imports = this.getImportsForFile(filePath, currentModulePath);
        if (imports.length === 0) {
            return '';
        }

        // Group imports by module
        const byModule = new Map<string, string[]>();
        for (const imp of imports) {
            if (!byModule.has(imp.modulePath)) {
                byModule.set(imp.modulePath, []);
            }
            byModule.get(imp.modulePath)!.push(imp.typeName);
        }

        // Generate use statements
        const statements: string[] = [];
        for (const [modulePath, types] of byModule.entries()) {
            for (const typeName of types.sort()) {
                statements.push(`use ${modulePath}::${typeName};`);
            }
        }

        return statements.join('\n');
    }

    /**
     * Check if a type is known/registered
     */
    isKnownType(typeName: string): boolean {
        return this.typeToModule.has(typeName);
    }

    /**
     * Get module path for a type
     */
    getModuleForType(typeName: string): string | undefined {
        return this.typeToModule.get(typeName);
    }

    /**
     * Get statistics about imports
     */
    getStats(): { totalTypes: number; totalFiles: number; totalImports: number } {
        let totalImports = 0;
        for (const imports of this.imports.values()) {
            totalImports += imports.size;
        }

        return {
            totalTypes: this.typeToModule.size,
            totalFiles: this.imports.size,
            totalImports
        };
    }
}

