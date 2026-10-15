# Desktop workbench

The Tauri application embeds the web workbench and calls the shared Rust client.
The default local engine runs without an HTTP server. A remote connection sends
execution data to the selected server instead. Neither mode synchronizes local
project libraries with another client.

## Build and identify the application

From the repository root:

```sh
npm --prefix apps/web ci
npm --prefix apps/web run desktop:build
```

Platform dependencies are listed in [building](../development/building.md).
Development mode uses `npm --prefix apps/web run desktop:dev`; its executable and
storage must not be mistaken for an installed production build.

On macOS the release bundle is under `target/release/bundle/macos/` and the DMG
under `target/release/bundle/dmg/`. Locate other platform installers in the
bundle directory produced by that platform's native build. A cross-compile or
archive hash does not prove that an installer starts or uninstalls correctly.
Current evidence and untested platforms are in the
[RC readiness report](../development/0.2.0-rc.1-release-readiness.md).

## Start the built application

On macOS, open the production bundle from the repository root:

```sh
open -n "target/release/bundle/macos/rx-chef.app"
```

The local engine needs no separately started server. To develop with live
frontend reload instead, run `npm --prefix apps/web run desktop:dev`.
Use List mode for sequential recipes and Graph mode for multiple named inputs
and outputs. See [local saves and exact exports](local-storage.md) before
closing work you want to keep.

## Choose the engine

Open **Connection**. An empty **Server URL** selects the local engine. For remote
execution, enter the server URL and Bearer token and choose **Connect (this
session)**. Update protocol-2 clients and servers together. Credentials are
session state; saved projects contain neither the URL's authentication state nor
its token. Reconnect after restart.

Use a reachable HTTPS endpoint for remote deployment with the server's configured
Host/Origin policy. The [server guide](../server/configuration.md) explains proxy
requirements, scopes and limits. A read-only token can discover operations but
cannot execute or validate projects/graphs through the protected execution API.

## Files and exact bytes

Follow the [browser workflow](browser.md) for compact menus, recipe/graph editing,
Auto-Bake, panel docking, output comparisons and SVG transformation reports.
Floating result panels stay inside the current app window. Native
import/export actions use operating-system dialogs. Cancelling a dialog leaves
the existing session in place. Choose a complete destination path through the
dialog's folder selection and name controls, then verify the resulting file.
Native saves suggest a format-specific filename. Cancelling a save keeps the
export dialog open and does not report a successful export. Project import
shows a review before replacing the workspace. Native saves are explicit;
navigating or closing the app is not a save action.

Graph saves preserve definitions and positions. Project exports let you select multiple definitions and optionally include
layout, settings, external references and metadata history. The Team / Git
profile excludes settings, references and history by default. A validated JSON
preview shows the selected content before the native save dialog opens. Both exclude bound
input bytes, output payloads and credentials. Known sensitive literal arguments
are rejected; replace them with named input bindings before export. File
references in a project are metadata and are never automatically opened.

For binary output, use the exact export action, then compare the file's byte
length and hash with an independent expected result. HEX/text/Base64 previews
are display views. Copying text from a preview does not save the original bytes.

## Offline and installation acceptance

A proper offline check starts the installed production bundle, disables network
access in the test environment, runs a local recipe and a multi-input graph, and
saves/reopens definitions and exact outputs. The local architecture supports this
workflow; the current RC report records whether it has actually been executed.
A browser served from localhost and a debug app are different evidence.

Test installers in an isolated account or disposable system: install, launch,
run a known-byte fixture, close during active work, reopen, uninstall and
reinstall. Check for surviving worker processes and distinguish application
removal from deliberate removal of user project data. Preserve exported backups.
Unsigned builds require separate Gatekeeper/signing/notarization assessment;
build success alone makes none of those claims.
