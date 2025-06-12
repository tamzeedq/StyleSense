import * as path from 'path';
import * as fs from 'fs';
import {
    ExtensionContext,
    window,
    commands,
    workspace,
    TextDocument
} from 'vscode';

import { 
    LanguageClient, 
    LanguageClientOptions, 
    ServerOptions 
} from 'vscode-languageclient/node';

// Single instance of the language client
let client: LanguageClient;

export function activate(context: ExtensionContext) {
    console.log('=== StyleSense extension activation started ===');
    
    // Show immediate feedback
    window.showInformationMessage('StyleSense: Extension activated!');
    console.log('=== Activation message shown ===');
    
    // Register the hello world command for testing
    let disposable = commands.registerCommand('stylesense.helloWorld', () => {
        console.log('=== Hello World command executed ===');
        window.showInformationMessage('Hello from StyleSense!');
    });
    
    context.subscriptions.push(disposable);
    console.log('=== Hello World command registered ===');
    
    // Add document opening event listener for debugging
    const docOpenDisposable = workspace.onDidOpenTextDocument((document: TextDocument) => {
        console.log('=== Document opened ===', document.fileName, document.languageId);
        if (document.languageId === 'cpp' || document.languageId === 'c') {
            console.log(`=== C/C++ file detected: ${document.languageId} file: ${document.fileName} ===`);
            window.showInformationMessage(`StyleSense: Opened ${document.languageId} file - check console for parsing output`);
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
        window.showErrorMessage(`StyleSense: ${errorMsg}`);
        return;
    }
    console.log('=== LSP server executable found ===');
    
    const serverOptions: ServerOptions = {
        command: serverCommand,
        options: { shell: true }
    };
    console.log('=== Server options configured ===');    // Client options - define the languages to handle
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cpp' }, { scheme: 'file', language: 'c' }],
    };
    console.log('=== Client options configured ===');
    
    // Create and start the client
    const outputChannel = window.createOutputChannel('StyleSense Language Server');
    console.log('=== Output channel created ===');
    
    client = new LanguageClient(
        'stylesense',
        'StyleSense',
        serverOptions,
        {
            ...clientOptions,
            outputChannel: outputChannel
        }
    );
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
        window.showInformationMessage('StyleSense: LSP server started - ready to parse C/C++ files');
    }).catch((error) => {
        console.error('=== Failed to start StyleSense LSP client ===', error);
        window.showErrorMessage(`StyleSense: Failed to start LSP server - ${error.message}`);
    });
    
    context.subscriptions.push(client);
    console.log('=== StyleSense extension activation completed ===');
}

export function deactivate(): Thenable<void> | undefined {
    return client ? client.stop() : undefined;
}
