#!/usr/bin/env python3
"""Exercise configured transaction examples against local Anvil nodes and a mock bundle relay."""

import contextlib
import json
import os
from pathlib import Path
import socket
import subprocess
import threading
import time
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.request import Request, urlopen


ROOT = Path(__file__).resolve().parent.parent
# Public Anvil development key; never use this account on a live network.
PRIVATE_KEY = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
SENDER = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"


def rpc(url, method, params):
    body = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).encode()
    request = Request(url, data=body, headers={"Content-Type": "application/json"})
    with urlopen(request, timeout=10) as response:
        result = json.load(response)
    assert "error" not in result, result
    return result["result"]


@contextlib.contextmanager
def anvil(chain_id):
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    url = f"http://127.0.0.1:{port}"
    process = subprocess.Popen(
        ["anvil", "--port", str(port), "--chain-id", str(chain_id), "--silent"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        for _ in range(100):
            if process.poll() is not None:
                raise RuntimeError("Anvil exited before starting")
            try:
                rpc(url, "eth_chainId", [])
                break
            except OSError:
                time.sleep(0.1)
        else:
            raise TimeoutError("Anvil did not start")
        yield url
    finally:
        process.terminate()
        process.wait(timeout=10)


def run_example(name, environment, success=True):
    env = os.environ.copy()
    for key in ("PRIVATE_KEY", "FLASHBOTS_AUTH_KEY", "RPC_URL", "RPC_URLS", "RELAY_URL"):
        env.pop(key, None)
    env.update(environment)
    result = subprocess.run(
        [str(ROOT / "target" / "debug" / "examples" / name)],
        env=env, capture_output=True, text=True, timeout=60,
    )
    assert (result.returncode == 0) == success, result.stdout + result.stderr
    return result


def test_multiple_networks(first, second):
    env = {"PRIVATE_KEY": PRIVATE_KEY, "RPC_URLS": f"{first},{second}"}
    run_example("multi_network_signer", env)
    for url, chain_id in ((first, 31337), (second, 31338)):
        block = rpc(url, "eth_getBlockByNumber", ["latest", True])
        tx, = block["transactions"]
        assert int(tx["chainId"], 16) == chain_id
        assert tx["from"].lower() == SENDER
        assert tx["to"].lower() == SENDER
        assert int(tx["value"], 16) == 0
        assert int(tx["nonce"], 16) == 0
        assert int(rpc(url, "eth_getTransactionCount", [SENDER, "latest"]), 16) == 1

    # Duplicate chains must fail before either independent nonce filler sends a transaction.
    env["RPC_URLS"] = f"{first},{first}"
    run_example("multi_network_signer", env, success=False)
    assert int(rpc(first, "eth_getTransactionCount", [SENDER, "latest"]), 16) == 1
    run_example("multi_network_signer", {}, success=False)
    print("Multiple networks: distinct chain IDs, independent nonces, duplicate-chain rejection")


def test_bundle(node):
    requests = []
    errors = []
    reject = False

    class Relay(BaseHTTPRequestHandler):
        def do_POST(self):
            try:
                body = self.rfile.read(int(self.headers["Content-Length"]))
                request = json.loads(body)
                assert request["method"] == "eth_sendBundle"
                bundle, = request["params"]
                latest = int(rpc(node, "eth_blockNumber", []), 16)
                assert int(bundle["blockNumber"], 16) == latest + 1
                raw_tx, = bundle["txs"]
                address, signature = self.headers["X-Flashbots-Signature"].split(":")
                assert address.lower() == SENDER
                digest = subprocess.check_output(["cast", "keccak", body.decode()], text=True).strip()
                subprocess.run(
                    ["cast", "wallet", "verify", "--address", address, digest, signature],
                    check=True, capture_output=True, text=True, timeout=10,
                )
                requests.append(request)
                response = {"jsonrpc": "2.0", "id": request["id"]}
                if reject:
                    response["error"] = {"code": -32000, "message": "bundle rejected for test"}
                else:
                    # Anvil validates the signature, chain ID, funding, nonce, and gas parameters.
                    tx_hash = rpc(node, "eth_sendRawTransaction", [raw_tx])
                    receipt = rpc(node, "eth_getTransactionReceipt", [tx_hash])
                    assert int(receipt["status"], 16) == 1
                    assert receipt["from"].lower() == SENDER
                    assert receipt["to"].lower() == SENDER
                    response["result"] = {"bundleHash": tx_hash}
                data = json.dumps(response).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            except Exception as error:
                errors.append(error)
                self.send_error(500)

        def log_message(self, *_args):
            pass

    with HTTPServer(("127.0.0.1", 0), Relay) as relay:
        worker = threading.Thread(target=relay.serve_forever, daemon=True)
        worker.start()
        env = {
            "PRIVATE_KEY": PRIVATE_KEY,
            "FLASHBOTS_AUTH_KEY": PRIVATE_KEY,
            "RPC_URL": node,
            "RELAY_URL": f"http://127.0.0.1:{relay.server_port}",
        }
        try:
            run_example("send_bundle", env)
            assert not errors, errors
            assert len(requests) == 1
            reject = True
            nonce = rpc(node, "eth_getTransactionCount", [SENDER, "latest"])
            run_example("send_bundle", env, success=False)
            assert not errors, errors
            assert len(requests) == 2
            assert rpc(node, "eth_getTransactionCount", [SENDER, "latest"]) == nonce
            run_example("send_bundle", {}, success=False)
        finally:
            relay.shutdown()
            worker.join(timeout=10)
    print("Bundle: authenticated payload, next block, valid signed transaction, rejection propagated")


if __name__ == "__main__":
    with anvil(31337) as first, anvil(31338) as second:
        test_multiple_networks(first, second)
        test_bundle(first)
