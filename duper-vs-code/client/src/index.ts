import { workspace, ExtensionContext, Uri } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

async function openDocument(uri: Uri) {
  const doc = workspace.textDocuments.find(
    (d) => d.uri.toString() === uri.toString(),
  );
  if (doc === undefined) await workspace.openTextDocument(uri);
  return uri;
}

export async function activate(context: ExtensionContext) {
  let serverOptions: ServerOptions = {
    command: "/home/eric/git/duper/target/debug/duper_lsp",
  };

  const deleteWatcher = workspace.createFileSystemWatcher(
    "**/*.duper",
    true,
    true,
    false,
  );
  const createChangeWatcher = workspace.createFileSystemWatcher(
    "**/*.duper",
    false,
    false,
    true,
  );

  context.subscriptions.push(deleteWatcher);
  context.subscriptions.push(createChangeWatcher);

  let clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "duper", pattern: "**/*.duper" }],
    synchronize: {
      fileEvents: deleteWatcher,
    },
    diagnosticCollectionName: "Duper",
  };

  client = new LanguageClient("Duper", "Duper", serverOptions, clientOptions);

  context.subscriptions.push(client.start());
  context.subscriptions.push(createChangeWatcher.onDidCreate(openDocument));
  context.subscriptions.push(createChangeWatcher.onDidChange(openDocument));

  const uris = await workspace.findFiles("**/*.duper");
  await Promise.all(uris.map(openDocument));
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
