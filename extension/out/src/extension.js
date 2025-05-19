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
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
// Single instance of the language client
let client;
// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
function activate(context) {
    // Register the hello world command for testing
    let disposable = vscode_1.commands.registerCommand('stylesense.helloWorld', () => {
        vscode_1.window.showInformationMessage('Hello from StyleSense!');
    });
    context.subscriptions.push(disposable);
    // Server options - specify how to launch the server
    const serverOptions = {
        command: path.join(__dirname, '..', '..', 'lsp', 'target', 'debug', 'lsp'),
        options: { shell: true }
    };
    // Client options - define the languages to handle
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    }; // Create and start the client
    client = new node_1.LanguageClient('stylesense', 'StyleSense', serverOptions, {
        ...clientOptions,
        outputChannel: vscode_1.window.createOutputChannel('StyleSense Language Server')
    });
    // Start the client
    client.start();
    // Push the disposable to the context's subscriptions
    context.subscriptions.push(client);
}
// This method is called when your extension is deactivated
function deactivate() {
    return client ? client.stop() : undefined;
}
//# sourceMappingURL=extension.js.map