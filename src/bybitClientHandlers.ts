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
import { makeValidRustIdent, toSnakeCase } from "./utils";
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

  // Check for REST patterns: this.getPrivate, this.postPrivate, etc.
  // Match: return this.XXXX('/endpoint', params)
  const restMatch = bodyText.match(
    /return\s+this\.(get|post|put|delete)(?:Private|Public)?\s*\(\s*['"]([^'"]+)['"]\s*(?:,\s*([\s\S]+?))?\s*\)/
  );

  if (restMatch) {
    const [, httpMethod, endpoint, paramsExpr] = restMatch;
    const methodName = bodyText.match(/this\.(get|post|put|delete)(?:Private|Public)?(?=\s*\()/)?.[0];
    return {
      type: "rest",
      httpMethod: httpMethod.toUpperCase() as "GET" | "POST" | "PUT" | "DELETE",
      endpoint,
      baseMethod: methodName?.replace("this.", "") || `${httpMethod}Private`,
    };
  }

  // Check for WebSocket patterns: this.wsClient.sendWSAPIRequest or sendWSAPIRequest
  // Supports: simple var params, object literal params, or no params
  const wsMatch = bodyText.match(
    /(?:this\.wsClient\.)?sendWSAPIRequest\s*\(\s*(?:WS_KEY_MAP\.)?(\w+)\s*,\s*['"]([^'"]+)['"]\s*(?:,\s*(?:(\w+)|\{[\s\S]*?\}))?\s*[,\s]*\)/
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

  // Handle keyof expressions — TS-only type construct, map to String
  if (cleanType.startsWith("keyof ")) {
    cleanType = "String";
    if (isOptional) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Handle indexed access types like Parameters<...>[N]
  if (/\]\s*$/.test(cleanType) && cleanType.includes('<')) {
    cleanType = "serde_json::Value";
    if (isOptional) {
      cleanType = `Option<${cleanType}>`;
    }
    return cleanType;
  }

  // Handle generic types with parameters: Foo<Bar, Baz>
  const genericMatch = cleanType.match(/^([A-Za-z_]\w*)<(.+)>$/);
  if (genericMatch) {
    const [, baseName, argsStr] = genericMatch;

    // If the base type isn't in the registry, fall back to serde_json::Value
    // This handles cases like InstrumentInfoResponseV5<C> where the generic
    // type alias was skipped during transpilation
    if (!typeRegistry.isKnownType(baseName) && !["Vec", "Option", "Box", "Arc", "HashMap", "IndexMap"].includes(baseName)) {
      cleanType = "serde_json::Value";
      if (isOptional && !cleanType.startsWith("Option<")) {
        cleanType = `Option<${cleanType}>`;
      }
      return cleanType;
    }

    const args = parseGenericArgs(argsStr);
    
    // Convert each argument recursively
    const convertedArgs = args.map(arg => convertTypeToRust(arg, typeRegistry, false));
    
    // Filter out unit types and empty object types from generic arguments
    // e.g., ClientResult<undefined, {}> -> ClientResult<()>
    const filteredArgs = convertedArgs.filter(arg => arg !== "()" && arg.trim() !== "");

    // If a known type gets 3+ generic args, it's likely from TS overload merging
    // (no generated structs have 3+ generic params) — fall back to serde_json::Value
    // Also fall back for 2+ args on response wrapper types where a generic param
    // may have been removed during inline object conversion
    if (filteredArgs.length > 2 && typeRegistry.isKnownType(baseName)) {
      cleanType = "serde_json::Value";
      if (isOptional && !cleanType.startsWith("Option<")) {
        cleanType = `Option<${cleanType}>`;
      }
      return cleanType;
    }
    if (filteredArgs.length === 2 && typeRegistry.isKnownType(baseName) && /Response|API/.test(baseName)) {
      // Response wrappers may have had generic params removed — use just the last arg
      cleanType = `${baseName}<${filteredArgs[filteredArgs.length - 1]}>`;
      if (isOptional && !cleanType.startsWith("Option<")) {
        cleanType = `Option<${cleanType}>`;
      }
      return cleanType;
    }
    
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
  if (cleanType === "number") cleanType = "Number";
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
    .split(/[.\-]/)
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
      body = `${callExpr}.and_then(|v| serde::Deserialize::deserialize(&v).map_err(|_| crate::client::ClientError::SerializationError(v.to_string())))`;
    } else if (returnType === "()") {
      // void return — discard the response value
      body = `${callExpr}.map(|_| ())`;
    } else {
      body = callExpr;
    }
  } else if (impl.type === "websocket") {
    const wsOperation = impl.wsOperation || "";
    const hasParams = parsedMethod.params.length > 0;

    let paramsArg: string;
    if (!hasParams) {
      paramsArg = "serde_json::Value::Null";
    } else if (parsedMethod.params.length === 1) {
      const paramName = makeValidRustIdent(parsedMethod.params[0].name);
      paramsArg = `serde_json::to_value(${paramName}).unwrap_or_default()`;
    } else {
      // Multiple params — build a JSON object (e.g. batch methods with category + orders)
      paramsArg = `serde_json::json!({${parsedMethod.params.map(p => {
        const name = makeValidRustIdent(p.name);
        return `"${p.name}": ${name}`;
      }).join(", ")}})`;
    }

    body = `self.ws_client.send_ws_api_request(Some(WsKey::${wsKeyToEnumVariant(impl.wsKey || "")}), serde_json::Value::String("${wsOperation}".to_string()), ${paramsArg}, None).await`;
  } else {
    // Check for known implementations (including abstract overrides)
    const knownImpls: Record<string, string> = {
      // RestClientV5 / SpotClientV3
      "getClientType": `Ok("v5".to_string())`,
      "fetchServerTime": `let res = self.get_server_time().await?;\nlet time_str = res.get("time").and_then(|v| v.as_str()).unwrap_or("0");\nOk(Number::from(time_str.parse::<f64>().unwrap_or(0.0) / 1000.0))`,
      "fetchLatencySummary": `use std::time::{SystemTime, UNIX_EPOCH};\nlet now_ms = || SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;\nlet t0 = now_ms();\nlet srv = self.get_server_time().await?;\nlet t1 = now_ms();\nlet srv_ms = srv.get("time").and_then(|v| v.as_str()).unwrap_or("0").parse::<f64>().unwrap_or(0.0);\nlet rtt = t1 - t0;\nlet one_way = (rtt / 2.0).floor();\nlet adjusted = srv_ms + one_way;\nlet diff = adjusted - t1;\nOk(serde_json::json!({"localTime": t1, "serverTime": srv_ms, "roundTripTime": rtt, "estimatedOneWayLatency": one_way, "adjustedServerTime": adjusted, "timeDifference": diff}))`,
      "uploadP2PChatFile": `Err(crate::client::ClientError::ApiError("uploadP2PChatFile requires multipart upload - not yet implemented".to_string()))`,
      // WebsocketAPIClient
      "setTimeOffsetMs": `Ok(())`,
      "getWSClient": `self.ws_client`,
      // WebsocketClient simple methods
      "sendPingEvent": `self.base.try_ws_send(&format!("{:?}", wsKey), &serde_json::json!({"op": "ping"}).to_string())`,
      "sendPongEvent": `self.base.try_ws_send(&format!("{:?}", wsKey), &serde_json::json!({"op": "pong"}).to_string())`,
      "getPrivateWSKeys": `Ok(vec![WsKey::V5Private, WsKey::V5PrivateTrade])`,
      "isAuthOnConnectWsKey": `Ok(matches!(wsKey, WsKey::V5Private | WsKey::V5PrivateTrade))`,
      "authPrivateConnectionsOnConnect": `Ok(true)`,
      "isCustomReconnectionNeeded": `Ok(false)`,
      "triggerCustomReconnectionWorkflow": `Ok(())`,
      "isWsPing": `let s = serde_json::to_string(&msg).unwrap_or_default();\nOk(s.contains(r#"op":"ping"#))`,
      "isWsPong": `let s = serde_json::to_string(&msg).unwrap_or_default();\nOk(s.contains(r#"ret_msg":"pong"#) || s.contains(r#"op":"pong"#))`,
      "getMaxTopicsPerSubscribeEvent": `Ok(serde_json::json!(500))`,
      "isPrivateTopicRequest": `let topic = request.topic.to_lowercase();\nlet private_topics = ["stop_order","outboundaccountinfo","executionreport","ticketinfo","copytradeposition","copytradeorder","copytradeexecution","copytradewallet","user.openapi.option.position","user.openapi.option.trade","user.order","user.execution","user.position","user.wallet","wallet","position","execution","order","dv5.position","dv5.order","dv5.execution","dv5.wallet"];\nOk(private_topics.iter().any(|t| *t == topic))`,
      // WebsocketClient connect methods
      "connectWSAPI": `self.base.connect("v5PrivateTrade").await.map(|_| serde_json::Value::Null)`,
      "connectPublic": `let keys = ["v5SpotPublic", "v5LinearPublic", "v5InversePublic", "v5OptionPublic"];\nlet mut results = Vec::new();\nfor k in &keys {\n    match self.base.connect(k).await {\n        Ok(_) => results.push(Some(serde_json::Value::Null)),\n        Err(_) => results.push(None),\n    }\n}\nOk(results)`,
      "connectPrivate": `self.base.connect("v5Private").await.map(|_| serde_json::Value::Null)`,
      "getWsUrl": `let base_url = match wsKey {\n    WsKey::V5Private | WsKey::V5PrivateTrade => "wss://stream.bybit.com/v5/private",\n    WsKey::V5SpotPublic => "wss://stream.bybit.com/v5/public/spot",\n    WsKey::V5LinearPublic => "wss://stream.bybit.com/v5/public/linear",\n    WsKey::V5InversePublic => "wss://stream.bybit.com/v5/public/inverse",\n    WsKey::V5OptionPublic => "wss://stream.bybit.com/v5/public/option",\n};\nOk(base_url.to_string())`,
      // WebsocketClient subscribe/unsubscribe (delegate to base)
      "subscribeV5": `let topics: Vec<String> = wsTopics.iter().filter_map(|t| t.as_str().map(String::from)).collect();\nself.base.subscribe(topics).await.map(|_| vec![])`,
      "unsubscribeV5": `let topics: Vec<String> = wsTopics.iter().filter_map(|t| t.as_str().map(String::from)).collect();\nself.base.subscribe(topics).await.map(|_| vec![])`,
      "subscribe": `Ok(())`,
      "unsubscribe": `Ok(())`,
      // Abstract method implementations (were unimplemented!, now real)
      "connectAll": `let mut results = Vec::new();\nfor k in &["v5SpotPublic", "v5LinearPublic", "v5InversePublic", "v5OptionPublic"] {\n    match self.base.connect(k).await {\n        Ok(_) => results.push(Some(serde_json::Value::Null)),\n        Err(_) => results.push(None),\n    }\n}\nlet _ = self.base.connect("v5Private").await;\nOk(results)`,
      "sendWSAPIRequest": `let ws_key_str = match wsKey {\n    Some(WsKey::V5Private) => "v5Private",\n    Some(WsKey::V5PrivateTrade) => "v5PrivateTrade",\n    _ => "v5PrivateTrade",\n};\nlet op_str = operation.as_str().unwrap_or("unknown");\nself.base.send_ws_api_request(ws_key_str, op_str, params).await`,
      "getWsAuthRequestEvent": `use crate::client::signing::{get_timestamp_ms, sign_hmac_sha256};\nuse crate::types::websockets::ws_api::inline::WsRequestOperationBybit_Args;\nlet api_key = self.base.config().api_key.as_ref()\n    .ok_or_else(|| crate::client::ClientError::ApiError("API key required".to_string()))?.clone();\nlet api_secret = self.base.config().api_secret.as_ref()\n    .ok_or_else(|| crate::client::ClientError::ApiError("API secret required".to_string()))?.clone();\nlet expires = get_timestamp_ms() + self.base.config().recv_window;\nlet sign_str = format!("GET/realtime{}", expires);\nlet signature = sign_hmac_sha256(&api_secret, &sign_str)\n    .map_err(|e| crate::client::ClientError::ApiError(e))?;\nOk(WsRequestOperationBybit {\n    req_id: format!("{:?}-auth", wsKey),\n    op: WsOperation::auth,\n    args: Some(vec![\n        WsRequestOperationBybit_Args::StringValue(api_key),\n        WsRequestOperationBybit_Args::NumberValue(Number::from(expires)),\n        WsRequestOperationBybit_Args::StringValue(signature),\n    ]),\n})`,
      "getWsRequestEvents": `let topics: Vec<String> = requests.iter().map(|r| r.topic.clone()).collect();\nlet req_id = if matches!(operation, WsOperation::subscribe | WsOperation::unsubscribe) && !topics.is_empty() {\n    topics.join(",")\n} else {\n    format!("req_{}", crate::client::signing::get_timestamp_ms())\n};\nlet ws_event = WsRequestOperationBybit {\n    req_id: req_id.clone(),\n    op: operation,\n    args: Some(topics.into_iter().map(|t| crate::types::websockets::ws_api::inline::WsRequestOperationBybit_Args::StringValue(t)).collect()),\n};\nOk(vec![MidflightWsRequestEvent {\n    request_key: req_id,\n    request_event: ws_event,\n}])`,
      // resolveEmittableEvents
      "resolveEmittableEvents": `let topic = event.get("topic").and_then(|v| v.as_str());\nlet op = event.get("op").and_then(|v| v.as_str());\nlet ret_code = event.get("retCode").and_then(|v| v.as_i64());\nlet req_id = event.get("reqId");\nlet make_event = |et: &str, ev: serde_json::Value, ws_api: Option<bool>| -> EmittableEvent<String> {\n    EmittableEvent { event_type: crate::util::inline::EmittableEvent_EventType::connectionReady(et.to_string()), event: ev, is_ws_api_response: ws_api }\n};\n// WS API response\nif let (Some(rc), Some(_)) = (ret_code, req_id) {\n    let et = if rc != 0 { "exception" } else { "response" };\n    return Ok(vec![make_event(et, event, Some(true))]);\n}\n// Topic update\nif topic.is_some() {\n    return Ok(vec![make_event("update", event, None)]);\n}\n// Operation response\nif let Some(op_str) = op {\n    let et = match op_str {\n        "auth" => "authenticated",\n        "subscribe" | "unsubscribe" | "ping" | "pong" | "COMMAND_RESP" => "response",\n        _ => if event.get("success") == Some(&serde_json::Value::Bool(false)) { "exception" } else { "update" },\n    };\n    return Ok(vec![make_event(et, event, None)]);\n}\nOk(vec![make_event("update", event, None)])`,
    };

    if (knownImpls[parsedMethod.name]) {
      body = knownImpls[parsedMethod.name];
    } else if (impl.type === "abstract") {
      body = `unimplemented!("Abstract method '${parsedMethod.name}' must be implemented by subclass")`;
    } else {
      body = `todo!("Method implementation: ${parsedMethod.name}")`;
    }
  }

  // Return type overrides for methods that need different signatures
  const returnTypeOverrides: Record<string, { type: string, raw?: boolean }> = {
    "getWSClient": { type: `&'a WebsocketClient<'a>`, raw: true },
  };
  const override = returnTypeOverrides[parsedMethod.name];
  let rawReturnType = false;
  if (override) {
    returnType = override.type;
    rawReturnType = override.raw || false;
  }

  return {
    name: rustName,
    params,
    returnType,
    body,
    docs: parsedMethod.jsDocs,
    isAsync: parsedMethod.isAsync,
    rawReturnType,
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
      
      let cleanType = param.type
        .replace(/\s*\|\s*undefined$/, "")
        .replace(/Promise<(.+)>$/, "$1")
        .replace(/import\([^)]+\)\./g, "")
        .trim();
      
      // Strip array suffix and generic args for registry lookup
      cleanType = cleanType.replace(/\[\]$/, "").replace(/<.+>$/, "").replace(/ & .+$/, "").trim();
      
      // Check if it's a known type (skip primitives, inline objects, literals)
      if (
        cleanType &&
        !cleanType.match(/^(string|number|boolean|void|any|unknown)$/) &&
        !cleanType.startsWith("{") &&
        !cleanType.startsWith('"') &&
        !cleanType.startsWith("'") &&
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
  if (usedTypes.has("Number")) {
    imports.push(`use crate::client::Number;`);
  }
  
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
    if (["String", "f64", "bool", "Vec", "Option", "ClientResult", "BaseRestClient", "BaseWebsocketClient", "WebsocketClient", "Number"].includes(typeName)) {
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

  const dedupedImports = Array.from(new Set(imports));

  return {
    name: className,
    structName,
    traitName: `${structName}Methods`,  // Not used in new architecture but required by interface
    methods: rustMethods,
    dependencies,
    imports: dedupedImports,
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


