/**
 * Console utility functions with colors
 */

// ANSI color codes
const colors = {
    reset: '\x1b[0m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    green: '\x1b[32m',
    blue: '\x1b[34m',
    gray: '\x1b[90m',
};

export function error(message: string): void {
    console.error(`${colors.red}${message}${colors.reset}`);
}

export function warn(message: string): void {
    console.warn(`${colors.yellow}${message}${colors.reset}`);
}

export function success(message: string): void {
    console.log(`${colors.green}${message}${colors.reset}`);
}

export function info(message: string): void {
    console.log(`${colors.blue}${message}${colors.reset}`);
}

export function log(message: string): void {
    console.log(message);
}

export function debug(message: string): void {
    console.log(`${colors.gray}${message}${colors.reset}`);
}

