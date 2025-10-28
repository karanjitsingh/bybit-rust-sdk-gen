// Generic client transpiler - works with any TypeScript API client
import {
  ClassDeclaration,
  MethodDeclaration,
  ParameterDeclaration,
  Node,
  Type,
  SyntaxKind,
} from "ts-morph";
import { TypeRegistry } from "./typeRegistry";
import * as console from "./console";

export interface ParsedMethod {
  name: string;
  methodDecl: MethodDeclaration;
  params: ParsedParameter[];
  returnType: string;
  jsDocs: string[];
  isAsync: boolean;
}

export interface ParsedParameter {
  name: string;
  type: string;
  isOptional: boolean;
  isRest: boolean;
}

export interface RustMethod {
  name: string;
  params: RustParameter[];
  returnType: string;
  body: string;
  docs: string[];
  isAsync: boolean;
}

export interface RustParameter {
  name: string;
  type: string;
  isSelf: boolean;
}

export interface RustClient {
  name: string;
  structName: string;
  traitName: string;  // Name of the trait (e.g., RestClientV5Api)
  methods: RustMethod[];
  dependencies: Set<string>;
  imports: string[];
  modulePath: string;
  usesWebSocket: boolean;  // Whether this client uses WebSocket functionality
}

export interface ClientHandlers {
  /**
   * Determine if a class should be transpiled as a client
   */
  isClientClass(classDecl: ClassDeclaration): boolean;

  /**
   * Generate Rust client code from parsed TypeScript class
   */
  generateClient(
    className: string,
    methods: ParsedMethod[],
    baseClass: string | undefined,
    typeRegistry: TypeRegistry
  ): RustClient;

  /**
   * Extract method implementation details (endpoint, operation, etc.)
   */
  parseMethodImplementation(method: MethodDeclaration): MethodImplementation;
}

export interface MethodImplementation {
  type: "rest" | "websocket" | "base_call" | "abstract" | "unknown";
  endpoint?: string;
  httpMethod?: "GET" | "POST" | "PUT" | "DELETE";
  wsOperation?: string;
  wsKey?: string;
  baseMethod?: string; // e.g., "getPrivate", "postPrivate"
  usesEndpointParam?: boolean; // For base_call: uses endpoint parameter instead of literal
  usesParamsParam?: boolean; // For base_call: uses params parameter
  isPublic?: boolean; // For base_call: whether it's a public or private API call
}

export class ClientTranspiler {
  constructor(
    private typeRegistry: TypeRegistry,
    private projectHandlers?: ClientHandlers
  ) {}

  /**
   * Parse a TypeScript class and extract method information
   */
  parseMethods(classDecl: ClassDeclaration): ParsedMethod[] {
    const methods: ParsedMethod[] = [];

    for (const method of classDecl.getMethods()) {
      // Skip private methods (starting with # or private modifier)
      if (method.hasModifier(SyntaxKind.PrivateKeyword)) {
        continue;
      }

      const methodName = method.getName();
      if (methodName.startsWith("_")) {
        continue;
      }

      const parsedMethod = this.parseMethod(method);
      methods.push(parsedMethod);
    }

    return methods;
  }

  /**
   * Parse a single method
   */
  private parseMethod(method: MethodDeclaration): ParsedMethod {
    const name = method.getName();
    const params = this.parseParameters(method);
    const returnType = this.extractReturnType(method);
    const jsDocs = this.extractJSDocs(method);
    const isAsync = method.isAsync() || returnType.startsWith("Promise<");

    return {
      name,
      methodDecl: method,
      params,
      returnType,
      jsDocs,
      isAsync,
    };
  }

  /**
   * Parse method parameters
   */
  private parseParameters(method: MethodDeclaration): ParsedParameter[] {
    return method.getParameters().map((param) => {
      const name = param.getName();
      const type = param.getType().getText();
      const isOptional = param.isOptional() || param.hasQuestionToken();
      const isRest = param.isRestParameter();

      return { name, type, isOptional, isRest };
    });
  }

  /**
   * Extract return type from method
   */
  private extractReturnType(method: MethodDeclaration): string {
    const returnType = method.getReturnType();
    let typeText = returnType.getText();

    // Remove 'import("...").' prefix if present
    typeText = typeText.replace(/import\([^)]+\)\./g, "");

    return typeText;
  }

  /**
   * Extract JSDoc comments
   */
  private extractJSDocs(method: MethodDeclaration): string[] {
    const jsDocs = method.getJsDocs();
    return jsDocs.map((doc) => doc.getComment()?.toString() || "");
  }

  /**
   * Get base class name if extends something
   */
  getBaseClass(classDecl: ClassDeclaration): string | undefined {
    const baseClass = classDecl.getBaseClass();
    return baseClass?.getName();
  }

  /**
   * Transpile a TypeScript client class to Rust
   */
  transpileClient(classDecl: ClassDeclaration): RustClient | null {
    const className = classDecl.getName();
    if (!className) {
      console.warn("⚠️  Skipping unnamed class");
      return null;
    }

    // Check if this is a client class
    if (
      this.projectHandlers &&
      !this.projectHandlers.isClientClass(classDecl)
    ) {
      return null;
    }

    console.log(`📦 Transpiling client: ${className}`);

    // Parse all methods
    const methods = this.parseMethods(classDecl);
    const baseClass = this.getBaseClass(classDecl);

    // Use project-specific handlers if available
    if (this.projectHandlers) {
      return this.projectHandlers.generateClient(
        className,
        methods,
        baseClass,
        this.typeRegistry
      );
    }

    // Default generic generation (basic)
    return this.generateGenericClient(className, methods);
  }

  /**
   * Default generic client generation (fallback)
   */
  private generateGenericClient(
    className: string,
    methods: ParsedMethod[]
  ): RustClient {
    const structName = this.toSnakeCase(className);
    const rustMethods: RustMethod[] = [];

    for (const method of methods) {
      const rustMethod: RustMethod = {
        name: this.toSnakeCase(method.name),
        params: [
          { name: "&self", type: "", isSelf: true },
          ...method.params.map((p) => ({
            name: p.name,
            type: p.type,
            isSelf: false,
          })),
        ],
        returnType: method.returnType,
        body: "todo!()", // Generic fallback
        docs: method.jsDocs,
        isAsync: method.isAsync,
      };
      rustMethods.push(rustMethod);
    }

    return {
      name: className,
      structName,
      traitName: `${structName}Methods`,  // Not used in new architecture but required by interface
      methods: rustMethods,
      dependencies: new Set(),
      imports: [],
      modulePath: structName,
      usesWebSocket: false,
    };
  }

  /**
   * Convert camelCase to snake_case
   */
  private toSnakeCase(str: string): string {
    return str
      .replace(/([A-Z])/g, "_$1")
      .toLowerCase()
      .replace(/^_/, "");
  }
}



