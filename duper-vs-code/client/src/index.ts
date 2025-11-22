import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import {
  type ExtensionContext,
  ExtensionMode,
  type Uri,
  window,
  workspace,
} from "vscode";
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

const ID = "Duper";
const NAME = "Duper";
const CONFIG_KEY = "duper";
const DUPER_BLOB = "**/*.duper";

export const outputChannel = window.createOutputChannel(NAME);

function getBinary(name: string): string | null {
  const result = spawnSync(process.platform === "win32" ? "where" : "which", [
    name,
  ]);
  if (result.status === 0) {
    const path = result.stdout.toString().trim().split("\n")[0].trim();
    return path || null;
  } else {
    return null;
  }
}

function getServerOptions(context: ExtensionContext): ServerOptions | null {
  const ext: string = process.platform === "win32" ? ".exe" : "";
  if (context.extensionMode === ExtensionMode.Development) {
    return {
      command: context.asAbsolutePath(`../target/debug/duper_lsp${ext}`),
      args: ["--debug"],
    };
  }

  const config = workspace.getConfiguration(CONFIG_KEY);
  const lspArgs = config.get("lsp.args") as string[];
  const lspBin = config.get("lsp.bin") as string;
  if (lspBin) {
    return {
      command: lspBin,
      args: lspArgs,
    };
  }

  const lspSystemBinary = config.get("lsp.systemBinary") as boolean;

  let arch: string | null = null;
  switch (process.arch) {
    case "arm64": {
      arch = "aarch64";
      break;
    }
    case "x64": {
      arch = "x86_64";
      break;
    }
  }
  let triple: string | null = null;
  if (arch !== null) {
    switch (process.platform) {
      case "darwin": {
        triple = `${arch}-apple-darwin`;
        break;
      }
      case "linux": {
        triple = `${arch}-unknown-linux-gnu`;
        break;
      }
      case "win32": {
        triple = `${arch}-pc-windows-gnu`;
        break;
      }
    }
  }

  if (lspSystemBinary || triple === null) {
    // Try to find LSP in PATH
    const binaryPath = getBinary(`lsp_duper${ext}`);
    if (binaryPath) {
      return {
        command: binaryPath,
        args: lspArgs,
      };
    }
    if (lspSystemBinary) {
      outputChannel.appendLine(
        `[client] \`duper_lsp${ext}\` not found in \`PATH\`.`,
      );
      if (triple === null) {
        outputChannel.appendLine(
          "[client]   = hint: Consider using the `duper.lsp.bin` configuration instead, or run `cargo install --locked duper_lsp` to build the Duper LSP.",
        );
      } else {
        outputChannel.appendLine(
          "[client]   = hint: Consider removing the `duper.lsp.systemBinary` configuration to use the bundled binary instead.",
        );
      }
    } else {
      outputChannel.appendLine(
        `[client] Unsupported platform and/or architecture '${process.platform}/${process.arch}'`,
      );
      outputChannel.appendLine(
        "[client]   = hint: Run `cargo install --locked duper_lsp` to build the Duper LSP.",
      );
    }
    return null;
  } else {
    // Try to get bundled LSP
    const binary = context.asAbsolutePath(`bin/${triple}/duper_lsp${ext}`);
    if (existsSync(binary)) {
      return {
        command: binary,
        args: lspArgs,
      };
    }
    outputChannel.appendLine(
      `[client] Binary not found for platform/architecture '${process.platform}/${process.arch}'; attempting to get binary from PATH instead.`,
    );
    outputChannel.appendLine(
      "[client]   = note: If you're seeing this message, it likely means that the Duper extension was bundled incorrectly. " +
        "Please open an issue at https://github.com/EpicEric/duper/issues",
    );
    // Debug: Load from PATH instead
    const binaryPath = getBinary(`lsp_duper${ext}`);
    if (binaryPath) {
      return {
        command: binaryPath,
        args: lspArgs,
      };
    }
    outputChannel.appendLine("[client] Binary not found in PATH.");
  }

  return null;
}

async function openDocument(uri: Uri) {
  const doc = workspace.textDocuments.find(
    (d) => d.uri.toString() === uri.toString(),
  );
  if (doc === undefined) await workspace.openTextDocument(uri);
  return uri;
}

export async function activate(context: ExtensionContext) {
  const serverOptions = getServerOptions(context);
  if (serverOptions === null) {
    const choice = await window.showErrorMessage(
      "Unable to find Duper LSP binary.",
      "Check logs",
    );
    if (choice === "Check logs") {
      outputChannel.show();
    }
    return;
  }

  const deleteWatcher = workspace.createFileSystemWatcher(
    DUPER_BLOB,
    true,
    true,
    false,
  );
  const createChangeWatcher = workspace.createFileSystemWatcher(
    DUPER_BLOB,
    false,
    false,
    true,
  );

  context.subscriptions.push(deleteWatcher);
  context.subscriptions.push(createChangeWatcher);

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "duper", pattern: DUPER_BLOB }],
    synchronize: {
      fileEvents: deleteWatcher,
    },
    diagnosticCollectionName: NAME,
  };

  client = new LanguageClient(ID, NAME, serverOptions, clientOptions);

  context.subscriptions.push(client.start());
  context.subscriptions.push(createChangeWatcher.onDidCreate(openDocument));
  context.subscriptions.push(createChangeWatcher.onDidChange(openDocument));

  const uris = await workspace.findFiles(DUPER_BLOB);
  await Promise.all(uris.map(openDocument));
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
