#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import socket
import socketserver
import sys
import threading
import time
from pathlib import Path


def load_sdk(sdk_src: str):
    if sdk_src not in sys.path:
        sys.path.insert(0, sdk_src)
    from openai_codex import Codex, CodexConfig, Sandbox

    return Codex, CodexConfig, Sandbox


def read_json(path: Path) -> dict[str, object]:
    if not path.exists():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")


def load_request(path: Path | None, raw: str | None) -> dict[str, object]:
    if path is not None:
        return json.loads(path.read_text(encoding="utf-8"))
    if raw is not None:
        return json.loads(raw)
    raise SystemExit("--request-file or --request-json is required")


def response(ok: bool, **payload: object) -> dict[str, object]:
    payload["ok"] = ok
    return payload


def bridge_error_response(operation: str, details: str) -> dict[str, object]:
    return response(
        False,
        operation=operation,
        session_name="",
        project_slug="",
        thread_id=None,
        turn_id=None,
        state="error",
        details=details,
        active_flags=[],
        final_response=None,
    )


def status_summary(thread_read) -> tuple[str, str, list[str]]:
    status = thread_read.thread.status.root
    status_type = status.type
    if status_type == "idle":
        return "idle", "thread idle", []
    if status_type == "systemError":
        return "error", "thread system error", []
    if status_type == "notLoaded":
        return "busy", "thread not loaded", []
    flags = [str(flag) for flag in getattr(status, "active_flags", [])]
    return "busy", f"active flags: {','.join(flags)}", flags


class HeadlessBridge:
    def __init__(self, request: dict[str, object]) -> None:
        self.sdk_src = str(request["sdk_src"])
        self.codex_bin = str(request["codex_bin"])
        self.cwd = str(request["cwd"])
        self.project_slug = str(request["project_slug"])
        self.session_name = str(request["session_name"])
        self.state_file = Path(str(request["state_file"]))
        Codex, CodexConfig, Sandbox = load_sdk(self.sdk_src)
        self.sandbox = Sandbox
        self.codex = Codex(CodexConfig(codex_bin=self.codex_bin))
        self.thread = None
        self.last_turn = None
        self.closed = False

    def start(self) -> dict[str, object]:
        state = read_json(self.state_file)
        if self.thread is None:
            self._load_or_start_thread(state)
        status = self.thread.read(include_turns=False)
        state_label, details, flags = status_summary(status)
        self._persist("start", state_label, details, flags)
        return self._projection("start", state_label, details, flags, resumed=bool(state.get("thread_id")))

    def status(self) -> dict[str, object]:
        if self.thread is None:
            self._load_or_start_thread(read_json(self.state_file))
        status = self.thread.read(include_turns=False)
        state_label, details, flags = status_summary(status)
        self._persist("status", state_label, details, flags)
        return self._projection("status", state_label, details, flags)

    def run(self, prompt: str) -> dict[str, object]:
        if self.thread is None:
            self._load_or_start_thread(read_json(self.state_file))
        self.last_turn = self.thread.turn(prompt, cwd=self.cwd, sandbox=self.sandbox.workspace_write)
        result = self.last_turn.run()
        status = self.thread.read(include_turns=False)
        state_label, details, flags = status_summary(status)
        final_response = result.final_response
        self._persist("run", state_label, details, flags, prompt=prompt, final_response=final_response)
        projection = self._projection("run", state_label, details, flags)
        projection["turn_id"] = self.last_turn.id
        projection["final_response"] = final_response
        projection["usage"] = result.usage.model_dump(mode="json", by_alias=True) if result.usage else None
        return projection

    def seed(self, prompt: str) -> dict[str, object]:
        state = read_json(self.state_file)
        if state.get("thread_id"):
            self._load_or_start_thread(state)
            status = self.thread.read(include_turns=False)
            state_label, details, flags = status_summary(status)
            self._persist("seed", state_label, details, flags)
            return self._projection("seed", state_label, details, flags, resumed=True)
        self.thread = self.codex.thread_start(
            cwd=self.cwd, sandbox=self.sandbox.workspace_write
        )
        self.last_turn = self.thread.turn(prompt, cwd=self.cwd, sandbox=self.sandbox.workspace_write)
        result = self.last_turn.run()
        status = self.thread.read(include_turns=False)
        state_label, details, flags = status_summary(status)
        final_response = result.final_response
        self._persist("seed", state_label, details, flags, prompt=prompt, final_response=final_response)
        projection = self._projection("seed", state_label, details, flags)
        projection["turn_id"] = self.last_turn.id
        projection["final_response"] = final_response
        projection["usage"] = result.usage.model_dump(mode="json", by_alias=True) if result.usage else None
        return projection

    def interrupt(self) -> dict[str, object]:
        if self.thread is None:
            self._load_or_start_thread(read_json(self.state_file))
        if self.last_turn is None:
            return response(False, operation="interrupt", state="error", details="missing active turn")
        reply = self.last_turn.interrupt()
        self._persist("interrupt", "busy", "interrupt requested", [])
        projection = self._projection("interrupt", "busy", "interrupt requested", [])
        projection["turn_id"] = self.last_turn.id
        projection["details"] = json.dumps(reply.model_dump(mode="json", by_alias=True), ensure_ascii=False)
        return projection

    def close(self) -> dict[str, object]:
        if not self.closed:
            self.codex.close()
            self.closed = True
        state = read_json(self.state_file)
        state.update({"bridge_status": "stopped", "updated_at": int(time.time())})
        write_json(self.state_file, state)
        return self._projection("stop", "offline", "headless bridge stopped", [])

    def _load_or_start_thread(self, state: dict[str, object]) -> None:
        thread_id = state.get("thread_id")
        if thread_id:
            self.thread = self.codex.thread_resume(
                str(thread_id), cwd=self.cwd, sandbox=self.sandbox.workspace_write
            )
        else:
            self.thread = self.codex.thread_start(
                cwd=self.cwd, sandbox=self.sandbox.workspace_write
            )

    def _persist(
        self,
        action: str,
        state_label: str,
        details: str,
        flags: list[str],
        *,
        prompt: str | None = None,
        final_response: str | None = None,
    ) -> None:
        state = read_json(self.state_file)
        state.update(
            {
                "project_slug": self.project_slug,
                "session_name": self.session_name,
                "thread_id": self.thread.id if self.thread else state.get("thread_id"),
                "last_turn_id": self.last_turn.id if self.last_turn else state.get("last_turn_id"),
                "last_action": action,
                "last_state": state_label,
                "last_status_details": details,
                "last_active_flags": flags,
                "updated_at": int(time.time()),
                "bridge_status": "running",
            }
        )
        if prompt is not None:
            state["last_prompt"] = prompt
        if final_response is not None:
            state["last_final_response"] = final_response
        write_json(self.state_file, state)

    def _projection(
        self,
        operation: str,
        state_label: str,
        details: str,
        flags: list[str],
        *,
        resumed: bool = False,
    ) -> dict[str, object]:
        return response(
            True,
            operation=operation,
            session_name=self.session_name,
            project_slug=self.project_slug,
            thread_id=self.thread.id if self.thread else None,
            turn_id=self.last_turn.id if self.last_turn else None,
            state=state_label,
            details=details,
            active_flags=flags,
            final_response=None,
            resumed=resumed,
        )


class BridgeServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
    daemon_threads = True
    allow_reuse_address = True

    def __init__(self, address, handler, bridge: HeadlessBridge, state_file: Path) -> None:
        super().__init__(address, handler)
        self.bridge = bridge
        self.state_file = state_file
        self.lock = threading.Lock()


class RequestHandler(socketserver.StreamRequestHandler):
    def handle(self) -> None:
        line = self.rfile.readline().decode("utf-8")
        try:
            request = json.loads(line)
            operation = str(request.get("operation", ""))
            with self.server.lock:
                payload = self._dispatch(operation, request)
        except Exception as exc:  # noqa: BLE001
            payload = self.server.bridge._projection("bridge", "error", str(exc), [])
            payload["ok"] = False
        self.wfile.write((json.dumps(payload, ensure_ascii=False) + "\n").encode("utf-8"))

    def _dispatch(self, operation: str, request: dict[str, object]) -> dict[str, object]:
        bridge = self.server.bridge
        if operation == "start":
            return bridge.start()
        if operation == "ping":
            return bridge._projection("ping", "running", "headless bridge alive", [])
        if operation == "status":
            return bridge.status()
        if operation == "run":
            prompt = request.get("prompt")
            if not isinstance(prompt, str) or not prompt.strip():
                payload = bridge._projection("run", "error", "prompt is required", [])
                payload["ok"] = False
                return payload
            return bridge.run(prompt)
        if operation == "seed":
            prompt = request.get("prompt")
            if not isinstance(prompt, str) or not prompt.strip():
                payload = bridge._projection("seed", "error", "prompt is required", [])
                payload["ok"] = False
                return payload
            return bridge.seed(prompt)
        if operation == "interrupt":
            return bridge.interrupt()
        if operation == "stop":
            payload = bridge.close()
            threading.Thread(target=self.server.shutdown, daemon=True).start()
            return payload
        payload = bridge._projection("bridge", "error", f"unsupported operation {operation}", [])
        payload["ok"] = False
        return payload


def serve(request: dict[str, object]) -> int:
    state_file = Path(str(request["state_file"]))
    try:
        bridge = HeadlessBridge(request)
    except Exception as exc:  # noqa: BLE001
        state = read_json(state_file)
        state.update(
            {
                "bridge_status": "error",
                "last_action": "daemon",
                "last_state": "error",
                "last_status_details": str(exc),
                "updated_at": int(time.time()),
            }
        )
        write_json(state_file, state)
        print(json.dumps(bridge_error_response("daemon", str(exc)), ensure_ascii=False), flush=True)
        return 1
    host = "127.0.0.1"
    with BridgeServer((host, 0), RequestHandler, bridge, state_file) as server:
        port = int(server.server_address[1])
        state = read_json(state_file)
        state.update(
            {
                "bridge_pid": os.getpid(),
                "bridge_host": host,
                "bridge_port": port,
                "bridge_status": "running",
                "project_slug": bridge.project_slug,
                "session_name": bridge.session_name,
                "updated_at": int(time.time()),
            }
        )
        write_json(state_file, state)
        print(json.dumps(response(True, operation="daemon", state="running", port=port), ensure_ascii=False), flush=True)
        try:
            server.serve_forever()
        finally:
            if not bridge.closed:
                bridge.close()
    return 0


def request_once(request: dict[str, object]) -> int:
    host = str(request["bridge_host"])
    port = int(request["bridge_port"])
    with socket.create_connection((host, port), timeout=10) as conn:
        conn.settimeout(None)
        conn.sendall((json.dumps(request, ensure_ascii=False) + "\n").encode("utf-8"))
        payload = b""
        while not payload.endswith(b"\n"):
            chunk = conn.recv(65536)
            if not chunk:
                break
            payload += chunk
    print(payload.decode("utf-8").strip())
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--daemon", action="store_true")
    parser.add_argument("--request-file")
    parser.add_argument("--request-json")
    args = parser.parse_args()
    request = load_request(Path(args.request_file) if args.request_file else None, args.request_json)
    if args.daemon:
        return serve(request)
    return request_once(request)


if __name__ == "__main__":
    raise SystemExit(main())
