import * as path from 'path';
import {
    ExtensionContext,
    window,
    commands,
    workspace
} from 'vscode';

import { 
    LanguageClient, 
    LanguageClientOptions, 
    ServerOptions 
} from 'vscode-languageclient/node';

// Single instance of the language client
let client: LanguageClient;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: ExtensionContext) {
    // Register the hello world command for testing
    let disposable = commands.registerCommand('stylesense.helloWorld', () => {
        window.showInformationMessage('Hello from StyleSense!');
    });
    
    context.subscriptions.push(disposable);
    
    // Server options - specify how to launch the server
    const serverOptions: ServerOptions = {
        command: path.join(__dirname, '..', '..', 'lsp', 'target', 'debug', 'lsp'),
        options: { shell: true }
    };

    // Client options - define the languages to handle
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    };    // Create and start the client
    client = new LanguageClient(
        'stylesense',
        'StyleSense',
        serverOptions,
        {
            ...clientOptions,
            outputChannel: window.createOutputChannel('StyleSense Language Server')
        }
    );
    
    // Start the client
    client.start();
    
    // Push the disposable to the context's subscriptions
    context.subscriptions.push(client);
}

// This method is called when your extension is deactivated
export function deactivate(): Thenable<void> | undefined {
    return client ? client.stop() : undefined;
}
