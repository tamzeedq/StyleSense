"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
// Single instance of the language client
let client;
function activate(context) {
    console.log('=== StyleSense extension activation started ===');
    // Show immediate feedback
    vscode_1.window.showInformationMessage('StyleSense: Extension activated!');
    console.log('=== Activation message shown ===');
    // Register the hello world command for testing
    let disposable = vscode_1.commands.registerCommand('stylesense.helloWorld', () => {
        console.log('=== Hello World command executed ===');
        vscode_1.window.showInformationMessage('Hello from StyleSense!');
    });
    context.subscriptions.push(disposable);
    console.log('=== Hello World command registered ===');
    // Add document opening event listener for debugging
    const docOpenDisposable = vscode_1.workspace.onDidOpenTextDocument((document) => {
        console.log('=== Document opened ===', document.fileName, document.languageId);
        if (document.languageId === 'cpp' || document.languageId === 'c') {
            console.log(`=== C/C++ file detected: ${document.languageId} file: ${document.fileName} ===`);
            vscode_1.window.showInformationMessage(`StyleSense: Opened ${document.languageId} file - check console for parsing output`);
        }
    });
    context.subscriptions.push(docOpenDisposable);
    console.log('=== Document open listener registered ===');
    const serverCommand = path.join(__dirname, '..', '..', '..', 'lsp', 'target', 'debug', 'lsp.exe');
    console.log('=== LSP server path ===', serverCommand);
    // Check if the server executable exists
    if (!fs.existsSync(serverCommand)) {
        const errorMsg = `LSP server executable not found at: ${serverCommand}`;
        console.error('=== ERROR ===', errorMsg);
        vscode_1.window.showErrorMessage(`StyleSense: ${errorMsg}`);
        return;
    }
    console.log('=== LSP server executable found ===');
    const serverOptions = {
        command: serverCommand,
        options: { shell: true }
    };
    console.log('=== Server options configured ==='); // Client options - define the languages to handle
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    };
    console.log('=== Client options configured ===');
    // Create and start the client
    const outputChannel = vscode_1.window.createOutputChannel('StyleSense Language Server');
    console.log('=== Output channel created ===');
    client = new node_1.LanguageClient('stylesense', 'StyleSense', serverOptions, {
        ...clientOptions,
        outputChannel: outputChannel
    });
    console.log('=== Language client created ===');
    // Add error event listener
    client.onDidChangeState((event) => {
        console.log('=== LSP client state changed ===', event);
    });
    console.log('=== State change listener added ===');
    // Start the client
    console.log('=== Starting LSP client ===');
    client.start().then(() => {
        console.log('=== StyleSense LSP client started successfully ===');
        vscode_1.window.showInformationMessage('StyleSense: LSP server started - ready to parse C/C++ files');
    }).catch((error) => {
        console.error('=== Failed to start StyleSense LSP client ===', error);
        vscode_1.window.showErrorMessage(`StyleSense: Failed to start LSP server - ${error.message}`);
    });
    context.subscriptions.push(client);
    console.log('=== StyleSense extension activation completed ===');
}
function deactivate() {
    return client ? client.stop() : undefined;
}
//# sourceMappingURL=extension.js.map