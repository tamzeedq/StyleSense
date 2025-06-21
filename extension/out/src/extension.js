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
    const serverCommand = path.join(__dirname, '..', '..', '..', 'lsp', 'target', 'debug', 'lsp.exe');
    // Check if the server executable exists
    if (!fs.existsSync(serverCommand)) {
        const errorMsg = `LSP server executable not found at: ${serverCommand}`;
        vscode_1.window.showErrorMessage(`StyleSense: ${errorMsg}`);
        return;
    }
    const serverOptions = {
        command: serverCommand,
        options: { shell: true }
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    };
    const outputChannel = vscode_1.window.createOutputChannel('StyleSense Language Server');
    client = new node_1.LanguageClient('stylesense', 'StyleSense', serverOptions, {
        ...clientOptions,
        outputChannel: outputChannel
    });
    client.start().then(() => {
        // Success - no need for notification in production
    }).catch((error) => {
        vscode_1.window.showErrorMessage(`StyleSense: Failed to start LSP server - ${error.message}`);
    });
    context.subscriptions.push(client);
}
function deactivate() {
    return client ? client.stop() : undefined;
}
//# sourceMappingURL=extension.js.map