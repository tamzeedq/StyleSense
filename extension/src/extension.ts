import * as path from 'path';
import * as fs from 'fs';
import {
    ExtensionContext,
    window
} from 'vscode';

import { 
    LanguageClient, 
    LanguageClientOptions, 
    ServerOptions 
} from 'vscode-languageclient/node';

// Single instance of the language client
let client: LanguageClient;

export function activate(context: ExtensionContext) {
    const serverCommand = path.join(__dirname, '..', '..', '..', 'lsp', 'target', 'debug', 'lsp.exe');
    
    // Check if the server executable exists
    if (!fs.existsSync(serverCommand)) {
        const errorMsg = `LSP server executable not found at: ${serverCommand}`;
        window.showErrorMessage(`StyleSense: ${errorMsg}`);
        return;
    }
    
    const serverOptions: ServerOptions = {
        command: serverCommand,
        options: { shell: true }
    };
    
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    };
    
    const outputChannel = window.createOutputChannel('StyleSense Language Server');
    
    client = new LanguageClient(
        'stylesense',
        'StyleSense',
        serverOptions,
        {
            ...clientOptions,
            outputChannel: outputChannel
        }
    );
    
    client.start().then(() => {
        // Success - no need for notification in production
    }).catch((error) => {
        window.showErrorMessage(`StyleSense: Failed to start LSP server - ${error.message}`);
    });
    
    context.subscriptions.push(client);
}

export function deactivate(): Thenable<void> | undefined {
    return client ? client.stop() : undefined;
}
