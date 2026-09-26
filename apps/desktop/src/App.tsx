import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type WorkspaceSummary = {
  path: string;
  format: string;
  schemaVersion: number;
};

export default function App() {
  const [path, setPath] = useState("example.synoema");
  const [status, setStatus] = useState("No workspace open");
  const [busy, setBusy] = useState(false);

  async function run(command: "create_workspace" | "open_workspace") {
    setBusy(true);
    try {
      const workspace = await invoke<WorkspaceSummary>(command, { path });
      setStatus(
        `${workspace.path} · ${workspace.format} schema ${workspace.schemaVersion}`,
      );
    } catch (error) {
      setStatus(String(error));
    } finally {
      setBusy(false);
    }
  }

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void run("open_workspace");
  }

  return (
    <main>
      <section className="hero" aria-labelledby="title">
        <p className="eyebrow">Synnoema 0.0.1</p>
        <h1 id="title">Build a space for connected knowledge.</h1>
        <p className="lede">
          A local-first foundation for portable knowledge workspaces. The graph
          canvas arrives next; your data model starts here.
        </p>

        <form onSubmit={submit}>
          <label htmlFor="workspace-path">Workspace path</label>
          <div className="controls">
            <input
              id="workspace-path"
              value={path}
              onChange={(event) => setPath(event.target.value)}
              spellCheck={false}
              disabled={busy}
            />
            <button
              type="button"
              onClick={() => void run("create_workspace")}
              disabled={busy}
            >
              Create
            </button>
            <button type="submit" className="secondary" disabled={busy}>
              Open
            </button>
          </div>
        </form>

        <output aria-live="polite">{status}</output>
      </section>
    </main>
  );
}
