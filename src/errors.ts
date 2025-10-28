/**
 * Error handling for the parser
 */

import * as console from "./console";

export enum ErrorSeverity {
    WARNING = "warning",
    ERROR = "error"
}

export interface ParserError {
    severity: ErrorSeverity;
    message: string;
    typeName: string;
    sourceFile: string;
    details?: string;
}

export class ParserErrorCollector {
    private errors: ParserError[] = [];

    addError(error: ParserError): void {
        this.errors.push(error);
    }

    addTypeConversionError(
        typeName: string,
        sourceFile: string,
        reason: string,
        details?: string
    ): void {
        this.errors.push({
            severity: ErrorSeverity.ERROR,
            message: `Failed to convert type '${typeName}'`,
            typeName,
            sourceFile,
            details: `${reason}${details ? `: ${details}` : ""}`
        });
    }

    addWarning(
        typeName: string,
        sourceFile: string,
        message: string
    ): void {
        this.errors.push({
            severity: ErrorSeverity.WARNING,
            message,
            typeName,
            sourceFile
        });
    }

    hasErrors(): boolean {
        return this.errors.some(e => e.severity === ErrorSeverity.ERROR);
    }

    hasWarnings(): boolean {
        return this.errors.some(e => e.severity === ErrorSeverity.WARNING);
    }

    getErrors(): ParserError[] {
        return this.errors.filter(e => e.severity === ErrorSeverity.ERROR);
    }

    getWarnings(): ParserError[] {
        return this.errors.filter(e => e.severity === ErrorSeverity.WARNING);
    }

    getAllIssues(): ParserError[] {
        return this.errors;
    }

    printSummary(): void {
        const errors = this.getErrors();
        const warnings = this.getWarnings();

        if (warnings.length > 0) {
            console.warn(`\n⚠️  ${warnings.length} Warning(s):\n`);
            warnings.forEach(warning => {
                console.warn(`  ${warning.sourceFile}`);
                console.warn(`    Type: ${warning.typeName}`);
                console.warn(`    Issue: ${warning.message}`);
                if (warning.details) {
                    console.warn(`    Details: ${warning.details}`);
                }
                console.warn("");
            });
        }

        if (errors.length > 0) {
            console.error(`\n❌ ${errors.length} Error(s):\n`);
            errors.forEach(error => {
                console.error(`  ${error.sourceFile}`);
                console.error(`    Type: ${error.typeName}`);
                console.error(`    Error: ${error.message}`);
                if (error.details) {
                    console.error(`    Details: ${error.details}`);
                }
                console.error("");
            });

            console.error("\n❌ Code generation failed due to errors.");
            console.error("Please fix the issues above or report them if they seem like bugs.\n");
        }
    }

    throwIfErrors(): void {
        if (this.hasErrors()) {
            throw new Error(`Parser encountered ${this.getErrors().length} error(s). See output above for details.`);
        }
    }
}

