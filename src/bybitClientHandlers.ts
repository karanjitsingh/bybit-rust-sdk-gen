// Bybit-specific client transpilation logic
import {
  ClassDeclaration,
  MethodDeclaration,
  Node,
  SyntaxKind,
  CallExpression,
} from "ts-morph";
import {
  ClientHandlers,
  ParsedMethod,
  RustClient,
  RustMethod,
  RustParameter,
  MethodImplementation,
} from "./clientTranspiler";
import { TypeRegistry } from "./typeRegistry";
import { makeValidRustIdent } from "./utils";
import * as console from "./console";

/**
 * Check if a class is a Bybit API client
 */
export function isClientClass(classDecl: ClassDeclaration): boolean {
  const className = classDecl.getName() || "";
  
  // Match client class names
  if (
    className.includes("Client") ||
    className.includes("API") ||
    className.endsWith("V5") ||
    className.endsWith("V3")
  ) {
    return true;
  }

  // Check if extends BaseRestClient or similar
  const baseClass = classDecl.getBaseClass();
  if (baseClass) {
    const baseName = baseClass.getName();
    if (baseName && baseName.includes("BaseRestClient")) {
      return true;
    }
  }

  return false;
}

/**
 * Parse method body to extract implementation details
 */
export function parseMethodImplementation(
  method: MethodDeclaration
): MethodImplementation {
  const body = method.getBody();
  if (!body) {
    return { type: "unknown" };
  }

  const bodyText = body.getText();

  // Check for BaseRestClient pattern: this._call(METHOD, endpoint, params, isPublic)
  // Example: return this._call('GET', endpoint, params, false)
  const baseCallMatch = bodyText.match(
    /return\s+this\._call\s*\(\s*['"]([^'"]+)['"]\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(true|false)\s*\)/
  );

  if (baseCallMatch) {
    const [, httpMethod, endpointParam, paramsParam, isPublic] = baseCallMatch;
    // For base client methods like get(), getPrivate(), etc., we use the endpoint parameter
    return {
      type: "base_call",
      httpMethod: httpMethod as "GET" | "POST" | "PUT" | "DELETE",
      usesEndpointParam: true,
      usesParamsParam: true,
      isPublic: isPublic === "true",
    };
  }

  // Check for REST patterns: this.getPrivate, this.postPrivate, this.get, this.post, etc.
  // Match: return this.XXXX('/endpoint', params)
  const restMatch = bodyText.match(
    /return\s+this\.(get|post|put|delete)(?:Private|Public)?\s*\(\s*['"]([^'"]+)['"]\s*(?:,\s*([\s\S]+?))?\s*\)/
  );

  if (restMatch) {
    const [, httpMethod, endpoint, paramsExpr] = restMatch;
    const methodName = bodyText.match(/this\.(get|post|put|delete)(Private|Public)?/)?.[0];
    return {
      type: "rest",
      httpMethod: httpMethod.toUpperCase() as "GET" | "POST" | "PUT" | "DELETE",
      endpoint,
      baseMethod: methodName?.replace("this.", "") || `${httpMethod}Private`,
    };
  }

  // Check for WebSocket patterns: this.wsClient.sendWSAPIRequest or sendWSAPIRequest
  const wsMatch = bodyText.match(
    /(?:this\.wsClient\.)?sendWSAPIRequest\s*\(\s*(?:WS_KEY_MAP\.)?(\w+)\s*,\s*['"]([^'"]+)['"]\s*,?\s*(\w+)?\s*\)/
  );

  if (wsMatch) {
    const [, wsKey, operation, paramsVar] = wsMatch;
    return {
      type: "websocket",
      wsOperation: operation,
      wsKey,
    };
  }

  // Check for abstract methods (no body or only throws)
  if (bodyText.trim() === "" || bodyText.includes("throw")) {
    return { type: "abstract" };
  }

  return { type: "unknown" };
}

/**
 * Parse generic type arguments (e.g., "Foo<Bar, Baz>" -> ["Bar", "Baz"])
 */
function parseGenericArgs(genericPart: string): string[] {
  const args: string[] = [];
  let depth = 0;
  let currentArg = "";
  
  for (let i = 0; i < genericPart.length; i++) {
    const char = genericPart[i];
    
    if (char === "<" || char === "[") {
      depth++;
      currentArg += char;
    } else if (char === ">" || char === "]") {
      depth--;
      currentArg += char;
    } else if (char === "," && depth === 0) {
      args.push(currentArg.trim());
      currentArg = "";
    } else {
      currentArg += char;
    }
  }
  
  if (currentArg.trim()) {
    args.push(currentArg.trim());
  }
  
  return args;
}

/**
 * Convert TypeScript type to Rust type
 */
function convertTypeToRust(
  tsType: string,
  typeRegistry: TypeRegistry,
  isOptional: boolean = false
): string {
  // Remove import(...). prefix
  let cleanType = tsType.replace(/import\([^)]+\)\./g, "");

  // Remove whitespace
  cleanType = cleanType.trim();

  // Extract from Promise<T> first
  const promiseMatch = cleanType.match(/^Promise<(.+)>$/);
  if (promiseMatch) {
    cleanType = promiseMatch[1];
  }

  // Handle APIResponseV3WithTime<T> or similar wrappers
  // These wrappers might have multiple generic parameters like APIResponseV3<T, TExtInfo = {}>
  // We only want the first parameter (the actual result type)
  const responseMatch = cleanType.match(/^APIResponse\w*<(.+)>$/);
  if (responseMatch) {
    const innerTypes = parseGenericArgs(responseMatch[1]);
    // Take only the first type parameter (the result)
    cleanType = innerTypes[0] || "()";
  }

  // Handle union with undefined (T | undefined) -> Option<T>
  // Do this before array handling to catch cases like (T | undefined)[]
  const unionUndefinedMatch = cleanType.match(/^(.+?)\s*\|\s*undefined$/);
  if (unionUndefinedMatch) {
    cleanType = unionUndefinedMatch[1].trim();
    isOptional = true;
  }

  // Handle intersection types (T & U) - use the first type only for now
  // In Rust, we'd need to use trait bounds or merge the types, which is complex
  // For generated code, we'll just use the base type
  if (cleanType.includes(" & ")) {
    const firstType = cleanType.split(" & ")[0].trim();
    cleanType = firstType;
  }

  // Handle anonymous inline objects like { list: string[] }
  if (cleanType.startsWith("{") && cleanType.endsWith("}")) {
    // For now, use serde_json::Value for anonymous types
    cleanType = "serde_json::Value";
    if (isOptional) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Handle arrays (TypeScript: T[])
  // This works for both simple arrays (Foo[]) and complex arrays (Foo<Bar>[])
  if (cleanType.endsWith("[]")) {
    const innerType = cleanType.slice(0, -2).trim();
    cleanType = `Vec<${convertTypeToRust(innerType, typeRegistry, false)}>`;
    
    if (isOptional && !cleanType.startsWith("Option<")) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Handle generic types with parameters: Foo<Bar, Baz>
  const genericMatch = cleanType.match(/^([A-Za-z_]\w*)<(.+)>$/);
  if (genericMatch) {
    const [, baseName, argsStr] = genericMatch;
    const args = parseGenericArgs(argsStr);
    
    // Convert each argument recursively
    const convertedArgs = args.map(arg => convertTypeToRust(arg, typeRegistry, false));
    
    // Filter out unit types and empty object types from generic arguments
    // e.g., ClientResult<undefined, {}> -> ClientResult<()>
    const filteredArgs = convertedArgs.filter(arg => arg !== "()" && arg.trim() !== "");
    
    if (filteredArgs.length > 0) {
      cleanType = `${baseName}<${filteredArgs.join(", ")}>`;
    } else {
      // If all args were filtered out, just use the base type with unit type
      cleanType = `${baseName}<()>`;
    }
    
    if (isOptional && !cleanType.startsWith("Option<")) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Handle array index access types like Foo[Bar] - use serde_json::Value for now
  if (cleanType.includes("[") && cleanType.includes("]") && !cleanType.endsWith("[]")) {
    cleanType = "serde_json::Value";
    if (isOptional) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Convert basic types
  if (cleanType === "string") cleanType = "String";
  if (cleanType === "number") cleanType = "f64";
  if (cleanType === "boolean") cleanType = "bool";
  if (cleanType === "void") cleanType = "()";
  if (cleanType === "any") cleanType = "serde_json::Value";
  if (cleanType === "unknown") cleanType = "serde_json::Value";
  if (cleanType === "object") cleanType = "serde_json::Value";
  if (cleanType === "undefined") cleanType = "()";
  if (cleanType === "null") cleanType = "()";
  if (cleanType === "{}") cleanType = "()"; // Empty object type
  if (cleanType === "never") cleanType = "()"; // Never type - use unit type

  // Handle TypeScript generic type parameters that start with T
  // (e.g., TWSKey, TWSOperation, etc.)
  if (cleanType.match(/^T[A-Z]\w*/)) {
    cleanType = "serde_json::Value";
  }

  // Handle common unknown types (WebSocket, MessageEventLike, etc.)
  const unknownTypes = ["WebSocket", "MessageEventLike", "WsStore", "WSConnectedResult", "TWSRequestEvent"];
  if (unknownTypes.includes(cleanType)) {
    cleanType = "serde_json::Value";
  }

  // Handle literal types like "v5" or 'linear'
  if ((cleanType.startsWith('"') && cleanType.endsWith('"')) || (cleanType.startsWith("'") && cleanType.endsWith("'"))) {
    // Literal string type -> just use String for now
    cleanType = "String";
  }

  // Handle union types like 0 | 1 (but not already handled T | undefined)
  if (cleanType.includes(" | ")) {
    // For now, use serde_json::Value for complex unions
    cleanType = "serde_json::Value";
  }

  // Wrap in Option if optional
  if (isOptional && !cleanType.startsWith("Option<")) {
    cleanType = `Option<${cleanType}>`;
  }

  return cleanType;
}

/**
 * Convert camelCase to snake_case
 * Handles acronyms properly (API -> api, UID -> uid, not a_p_i or u_i_d)
 */
function toSnakeCase(str: string): string {
  // First, handle known acronyms by temporarily replacing them
  const acronyms: [string, string][] = [
    ['UID', '~uid~'],
    ['API', '~api~'],
    ['HTTP', '~http~'],
    ['HTTPS', '~https~'],
    ['URL', '~url~'],
    ['URI', '~uri~'],
    ['UUID', '~uuid~'],
    ['WS', '~ws~'],
    ['WSS', '~wss~'],
  ];
  
  let result = str;
  
  // Replace acronyms with placeholders
  for (const [acronym, placeholder] of acronyms) {
    result = result.replace(new RegExp(acronym, 'g'), placeholder);
  }
  
  // Now do standard snake_case conversion
  result = result
    // Insert underscore before uppercase letters that follow lowercase/digit  
    .replace(/([a-z\d])([A-Z~])/g, "$1_$2")
    // Insert underscore before uppercase letter followed by lowercase (handles remaining acronyms)
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toLowerCase();
  
  // Restore acronyms by removing placeholders
  result = result.replace(/~/g, '');
  
  return result;
}

/**
 * Convert className to PascalCase struct name
 * e.g., RestClientV5 -> RestClientV5, BaseRestClient -> BaseRestClient
 */
function toPascalCase(str: string): string {
  // If already PascalCase, return as-is
  if (str.match(/^[A-Z][a-zA-Z0-9]*$/)) {
    return str;
  }
  
  // Convert from snake_case or other formats
  return str
    .split(/[_-]/)
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join('');
}

/**
 * Convert operation string to enum variant (e.g., 'order.create' -> 'OrderCreate')
 */
function operationToEnumVariant(operation: string): string {
  return operation
    .split(".")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

/**
 * Convert WS_KEY_MAP key to enum variant (e.g., 'v5PrivateTrade' -> 'V5PrivateTrade')
 */
function wsKeyToEnumVariant(wsKey: string): string {
  // Handle camelCase conversion
  if (wsKey.startsWith("v5")) {
    return "V5" + wsKey.slice(2);
  }
  if (wsKey.startsWith("v3")) {
    return "V3" + wsKey.slice(2);
  }
  return wsKey.charAt(0).toUpperCase() + wsKey.slice(1);
}

/**
 * Generate Rust method from parsed TypeScript method
 */
function generateRustMethod(
  parsedMethod: ParsedMethod,
  impl: MethodImplementation,
  typeRegistry: TypeRegistry
): RustMethod {
  const rustName = toSnakeCase(parsedMethod.name);
  
  // Build parameters
  const params: RustParameter[] = [
    { name: "&self", type: "", isSelf: true },
  ];

  for (const param of parsedMethod.params) {
    if (param.isRest) {
      // Skip rest parameters for now
      continue;
    }

    const rustType = convertTypeToRust(param.type, typeRegistry, param.isOptional);
    params.push({
      name: makeValidRustIdent(param.name),
      type: rustType,
      isSelf: false,
    });
  }

  // Extract return type
  let returnType = convertTypeToRust(
    parsedMethod.returnType,
    typeRegistry,
    false
  );

  // Generate method body
  let body = "";
  if (impl.type === "base_call") {
    // For base client methods that call _call internally
    // Example: get(endpoint, params) -> calls _call('GET', endpoint, params, true)
    const httpMethod = impl.httpMethod?.toLowerCase() || "get";
    const isPublic = impl.isPublic ? "true" : "false";
    
    // These methods pass through to the internal HTTP implementation
    // In Rust, we'll delegate to the trait implementation
    body = `self._call("${httpMethod.toUpperCase()}", endpoint, params, ${isPublic}).await`;
  } else if (impl.type === "rest") {
    const endpoint = impl.endpoint || "";
    const baseMethod = impl.baseMethod || "get_private";
    const hasParams = parsedMethod.params.length > 0;
    
    // Convert params to Option<serde_json::Value>
    let paramsArg: string;
    if (hasParams) {
      const paramName = makeValidRustIdent(parsedMethod.params[0].name);
      const paramType = convertTypeToRust(parsedMethod.params[0].type, typeRegistry, parsedMethod.params[0].isOptional);
      
      if (paramType === "()") {
        paramsArg = "None::<()>";
      } else if (paramType.startsWith("Option<")) {
        // Parameter is already Option<T>, need to convert to Option<serde_json::Value>
        paramsArg = `${paramName}.map(|v| serde_json::to_value(v).unwrap_or_default())`;
      } else {
        // Parameter is T, need to convert to Option<serde_json::Value>
        paramsArg = `Some(serde_json::to_value(${paramName}).unwrap_or_default())`;
      }
    } else {
      paramsArg = "None";
    }

    const callExpr = `self.base.${toSnakeCase(baseMethod)}("${endpoint}", ${paramsArg}).await`;
    
    // Check if return type needs deserialization
    if (returnType !== "serde_json::Value" && returnType !== "()") {
      // Need to deserialize the response
      body = `${callExpr}.and_then(|v| serde_json::from_value(v).map_err(|e| crate::client::ClientError::SerializationError(e.to_string())))`;
    } else {
      body = callExpr;
    }
  } else if (impl.type === "websocket") {
    const wsKeyVariant = wsKeyToEnumVariant(impl.wsKey || "");
    const operationVariant = operationToEnumVariant(impl.wsOperation || "");
    const hasParams = parsedMethod.params.length > 0;
    const paramsArg = hasParams ? "params" : "()";

    body = `self.ws_client.send_ws_api_request(WsKey::${wsKeyVariant}, WSAPIOperation::${operationVariant}, ${paramsArg}).await`;
  } else if (impl.type === "abstract") {
    // Abstract methods should be implemented by subclasses
    body = `unimplemented!("Abstract method '${parsedMethod.name}' must be implemented by subclass")`;
  } else {
    body = `todo!("Method implementation: ${parsedMethod.name}")`;
  }

  return {
    name: rustName,
    params,
    returnType,
    body,
    docs: parsedMethod.jsDocs,
    isAsync: parsedMethod.isAsync,
  };
}

/**
 * Generate complete Rust client from TypeScript class
 */
export function generateClient(
  className: string,
  methods: ParsedMethod[],
  baseClass: string | undefined,
  typeRegistry: TypeRegistry
): RustClient {
  console.log(`  🔧 Generating Rust client for ${className}`);

  const structName = toPascalCase(className);
  const rustMethods: RustMethod[] = [];
  const dependencies = new Set<string>();

  // Track known WebSocket operations for validation
  const knownWsOperations = new Set<string>();

  // Generate each method
  for (const method of methods) {
    const impl = parseMethodImplementation(method.methodDecl);
    
    // Validate if it's a known pattern
    if (impl.type === "unknown") {
      console.warn(`  ⚠️  Unknown method pattern in ${method.name}, generating stub`);
    }

    // Cross-validate WebSocket operations
    if (impl.type === "websocket" && impl.wsOperation) {
      // Track all WS operations we encounter
      knownWsOperations.add(impl.wsOperation);
      
      // Check if operation uses valid syntax (e.g., 'order.create')
      if (!impl.wsOperation.includes(".")) {
        console.warn(`  ⚠️  WS operation '${impl.wsOperation}' doesn't follow expected pattern (expected: 'category.action')`);
      }
    }

    // Validate parameter types exist
    for (const param of method.params) {
      if (param.isRest) continue;
      
      const cleanType = param.type
        .replace(/\s*\|\s*undefined$/, "")
        .replace(/Promise<(.+)>$/, "$1")
        .replace(/import\([^)]+\)\./g, "")
        .trim();
      
      // Check if it's a known type (skip primitives and inline objects)
      if (
        cleanType &&
        !cleanType.match(/^(string|number|boolean|void|any)$/) &&
        !cleanType.startsWith("{") &&
        !cleanType.includes(" | ")
      ) {
        const isKnownType = typeRegistry.isKnownType(cleanType);
        if (!isKnownType) {
          console.warn(`  ⚠️  Parameter type '${cleanType}' in method '${method.name}' not found in type registry`);
        }
      }
    }

    const rustMethod = generateRustMethod(method, impl, typeRegistry);
    rustMethods.push(rustMethod);

    // Track dependencies from parameters and return types
    for (const param of method.params) {
      const cleanType = param.type
        .replace(/Promise<(.+)>$/, "$1")
        .replace(/import\([^)]+\)\./g, "");
      if (cleanType && !cleanType.match(/^(string|number|boolean|void)$/)) {
        dependencies.add(cleanType);
      }
    }
  }

  // Log summary of WebSocket operations found
  if (knownWsOperations.size > 0) {
    console.log(`  ℹ️  Found ${knownWsOperations.size} WebSocket operations: ${Array.from(knownWsOperations).join(", ")}`);
  }

  // Collect all type references from method signatures and bodies
  const usedTypes = new Set<string>();
  
  // Always need ClientResult
  usedTypes.add("ClientResult");
  
  // Determine which base client is needed
  const isRestClient = !className.includes("Websocket") && !className.includes("WebSocket");
  const isWebsocketAPIClient = className === "WebsocketAPIClient";
  
  if (isRestClient) {
    usedTypes.add("BaseRestClient");
  } else if (!isWebsocketAPIClient) {
    usedTypes.add("BaseWebsocketClient");
  }
  
  // Check if WebsocketClient is referenced (for WebsocketAPIClient)
  if (isWebsocketAPIClient) {
    usedTypes.add("WebsocketClient");
  }
  
  // Extract types from method parameters and return types
  for (const method of rustMethods) {
    // Extract from parameters
    for (const param of method.params) {
      if (!param.isSelf && param.type) {
        extractTypesFromString(param.type, usedTypes);
      }
    }
    
    // Extract from return type
    if (method.returnType) {
      extractTypesFromString(method.returnType, usedTypes);
    }
    
    // Extract from method body
    if (method.body) {
      extractTypesFromString(method.body, usedTypes);
    }
  }
  
  // Build minimal import list based on used types
  const imports: string[] = [];
  
  // Always add ClientResult
  imports.push(`use crate::client::ClientResult;`);
  
  // Base clients are re-exported from their modules, so import them directly
  if (usedTypes.has("BaseRestClient")) {
    imports.push(`use crate::client::BaseRestClient;`);
  }
  if (usedTypes.has("BaseWebsocketClient")) {
    imports.push(`use crate::client::BaseWebsocketClient;`);
  }
  
  // WebsocketClient is a module with a struct of the same name
  if (usedTypes.has("WebsocketClient")) {
    imports.push(`use crate::client::WebsocketClient::WebsocketClient;`);
  }
  
  // Only add serde/builder imports if we actually serialize/deserialize in the body
  const needsSerde = rustMethods.some(m => m.body.includes("serde_json::to_value") || m.body.includes("serde_json::from_value"));
  // Note: We don't actually need serde imports in client files since we use fully qualified paths
  
  // Add type imports only for types that are actually used
  // Group by module for cleaner imports
  const typeModules = new Map<string, Set<string>>();
  
  for (const typeName of usedTypes) {
    // Skip primitive types and std types
    if (["String", "f64", "bool", "Vec", "Option", "ClientResult", "BaseRestClient", "BaseWebsocketClient", "WebsocketClient"].includes(typeName)) {
      continue;
    }
    
    // Check in type registry to find module path
    const modulePath = typeRegistry.getTypeModulePath(typeName);
    if (modulePath) {
      if (!typeModules.has(modulePath)) {
        typeModules.set(modulePath, new Set());
      }
      typeModules.get(modulePath)!.add(typeName);
    }
  }
  
  // Add grouped imports
  for (const [modulePath, types] of Array.from(typeModules.entries()).sort()) {
    // If there are many types from the same module, use wildcard
    if (types.size > 5) {
      imports.push(`use ${modulePath}::*;`);
    } else {
      imports.push(`use ${modulePath}::{${Array.from(types).sort().join(", ")}};`);
    }
  }
  
  // Check for util types (WebSocket utility types, etc.)
  const utilTypeNames = ["WsTopicRequest", "MidflightWsRequestEvent", "EmittableEvent", "WsTopicRequestOrStringTopic"];
  const usedUtilTypes = utilTypeNames.filter(t => usedTypes.has(t));
  
  if (usedUtilTypes.length > 0) {
    imports.push(`use crate::util::{${usedUtilTypes.sort().join(", ")}};`);
  }
  
  // Helper function to extract type names from type strings
  function extractTypesFromString(typeStr: string, typeSet: Set<string>) {
    // Remove common wrappers and extract type names
    // Match: Type, Type<...>, Vec<Type>, Option<Type>, etc.
    const typePattern = /\b([A-Z][A-Za-z0-9_]*)\b/g;
    const matches = typeStr.matchAll(typePattern);
    
    for (const match of matches) {
      const typeName = match[1];
      // Skip Rust std types
      if (!["Vec", "Option", "Result", "String", "Box", "Arc", "Rc"].includes(typeName)) {
        typeSet.add(typeName);
      }
    }
  }

  // Check if any methods use WebSocket
  const usesWebSocket = rustMethods.some((m) => m.body.includes("ws_client"));

  return {
    name: className,
    structName,
    traitName: `${structName}Methods`,  // Not used in new architecture but required by interface
    methods: rustMethods,
    dependencies,
    imports,
    modulePath: `client/${structName}`,
    usesWebSocket,
  };
}

/**
 * Export handlers object that implements ClientHandlers interface
 */
export const BybitClientHandlers: ClientHandlers = {
  isClientClass,
  generateClient,
  parseMethodImplementation,
};


