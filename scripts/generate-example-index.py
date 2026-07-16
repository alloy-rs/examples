#!/usr/bin/env python3
"""Generate the machine-readable example index from Cargo and source metadata."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
INDEX_PATH = ROOT / "examples-index.json"
RUNTIME_ALLOWLIST = ROOT / "scripts" / "runtime-examples.txt"

# Examples with fixed chain-specific addresses, block ranges, or transaction hashes. Endpoint-based
# examples default to user-supplied so generic provider and transport examples are not mislabeled.
NETWORK_OVERRIDES = {
    "address_lookup": "ethereum-mainnet",
    "any_network": "arbitrum-sepolia",
    "name_resolution": "ethereum-mainnet",
    "query_contract_storage": "ethereum-mainnet",
    "query_deployed_bytecode": "ethereum-mainnet",
    "query_logs": "ethereum-mainnet",
    "query_logs_chunked": "ethereum-mainnet",
    "subscribe_all_logs": "ethereum-mainnet",
    "subscribe_logs": "ethereum-mainnet",
    "ledger_signer": "ethereum-mainnet",
    "trezor_signer": "ethereum-mainnet",
    "yubi_signer": "ethereum-mainnet",
    "anvil_set_storage_at": "ethereum-mainnet-fork",
    "gas_price_usd": "ethereum-mainnet-fork",
    "interact_with_abi": "ethereum-mainnet-fork",
    "multicall": "ethereum-mainnet-fork",
    "multicall_batching": "ethereum-mainnet-fork",
    "permit2_signature_transfer": "ethereum-mainnet-fork",
    "simulation_uni_v2": "ethereum-mainnet-fork",
    "trace_transaction": "ethereum-mainnet-fork",
    "uniswap_u256_alloy_simulation": "ethereum-mainnet-fork",
}

CARGO_PROVIDED_ENV = {
    "DEBUG",
    "HOST",
    "NUM_JOBS",
    "OPT_LEVEL",
    "OUT_DIR",
    "PROFILE",
    "TARGET",
}


def load_metadata() -> dict:
    result = subprocess.run(
        [
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def load_runtime_allowlist() -> set[str]:
    return {
        line
        for raw_line in RUNTIME_ALLOWLIST.read_text().splitlines()
        if (line := raw_line.strip()) and not line.startswith("#")
    }


def summary_from(source: str) -> str:
    lines: list[str] = []
    for line in source.splitlines():
        if not line.startswith("//!"):
            break
        value = line.removeprefix("//!").strip()
        if not value:
            if lines:
                break
            continue
        lines.append(value)
    return " ".join(lines)


def environment_from(source: str) -> list[str]:
    names = set(
        re.findall(r'(?:required_env|std::env::var)\("([A-Z][A-Z0-9_]*)"\)', source)
    )
    names = {
        name
        for name in names
        if not name.startswith(("CARGO_", "DEP_", "RUSTC_"))
        and name not in CARGO_PROVIDED_ENV
    }
    helpers = {
        "rpc_url()": "RPC_URL",
        "rpc_urls()": "RPC_URLS",
        "ws_url()": "WS_URL",
        "ipc_path()": "IPC_PATH",
    }
    for call, name in helpers.items():
        if call in source:
            names.add(name)
    return sorted(names)


def runtime_metadata(name: str, source: str, offline: set[str]) -> dict:
    environment = environment_from(source)
    binaries = []
    hardware = []
    services = []
    arguments = []

    binary_patterns = {
        "anvil": ("Anvil::", "connect_anvil"),
        "geth": ("Geth::",),
        "reth": ("Reth::",),
    }
    for binary, patterns in binary_patterns.items():
        if any(pattern in source for pattern in patterns):
            binaries.append(binary)

    hardware_patterns = {
        "Ledger device": "LedgerSigner",
        "Trezor device": "TrezorSigner",
        "YubiHSM device": "YubiSigner",
    }
    for requirement, pattern in hardware_patterns.items():
        if pattern in source:
            hardware.append(requirement)

    if "api.blocknative.com" in source:
        services.append("Blocknative gas API")
    if "rpc.flashbots.net" in source:
        services.append("Flashbots Protect")
    if "AwsSigner" in source:
        services.append("AWS KMS")
    if "GcpSigner" in source:
        services.append("Google Cloud KMS")

    if name in {"compare_new_heads", "compare_pending_txs"}:
        arguments.append("-r <name>:<url> (repeat for each provider)")

    if name in offline:
        runtime_class = "offline"
    elif hardware:
        runtime_class = "hardware"
    elif any(service.endswith("KMS") for service in services):
        runtime_class = "cloud-credentials"
    elif arguments or environment:
        runtime_class = "configured-network"
    elif services:
        runtime_class = "external-service"
    elif binaries:
        runtime_class = "local-node"
    elif "std::fs::" in source or "read_to_string" in source:
        runtime_class = "local-filesystem"
    else:
        runtime_class = "local"

    network = NETWORK_OVERRIDES.get(name)
    endpoint_environment = {"RPC_URL", "RPC_URLS", "WS_URL"}.intersection(environment)
    if network is None:
        if arguments or endpoint_environment:
            network = "user-supplied"
        elif "Flashbots Protect" in services:
            network = "ethereum-mainnet"
        elif binaries:
            network = "local-development"
        elif "IPC_PATH" in environment:
            network = "configured-node"

    return {
        "class": runtime_class,
        "network": network,
        "environment": environment,
        "arguments": arguments,
        "binaries": binaries,
        "hardware": hardware,
        "services": services,
    }


def build_index() -> dict:
    metadata = load_metadata()
    offline = load_runtime_allowlist()
    examples = []

    for package in metadata["packages"]:
        for target in package["targets"]:
            if "example" not in target["kind"]:
                continue

            source_path = Path(target["src_path"])
            relative_path = source_path.relative_to(ROOT).as_posix()
            source = source_path.read_text()
            summary = summary_from(source)
            if not summary:
                raise SystemExit(f"Example is missing a leading //! summary: {relative_path}")

            package_name = package["name"]
            examples.append(
                {
                    "name": target["name"],
                    "category": package_name.removeprefix("examples-"),
                    "package": package_name,
                    "source": relative_path,
                    "summary": summary,
                    "command": (
                        f"cargo run --locked -p {package_name} --example {target['name']}"
                    ),
                    "runtime": runtime_metadata(target["name"], source, offline),
                }
            )

    examples.sort(key=lambda item: (item["category"], item["name"]))
    names = Counter(item["name"] for item in examples)
    duplicates = sorted(name for name, count in names.items() if count > 1)
    if duplicates:
        raise SystemExit(f"Duplicate example target names: {', '.join(duplicates)}")

    target_names = set(names)
    unknown_allowlist = sorted(offline - target_names)
    if unknown_allowlist:
        raise SystemExit(
            "Unknown examples in scripts/runtime-examples.txt: " + ", ".join(unknown_allowlist)
        )

    readme = (ROOT / "README.md").read_text()
    missing_from_readme = [
        item["source"] for item in examples if f"(./{item['source']})" not in readme
    ]
    if missing_from_readme:
        raise SystemExit("Examples missing from README.md: " + ", ".join(missing_from_readme))

    return {
        "schema_version": 1,
        "description": (
            "Generated Alloy example catalog. Use runtime fields to choose examples compatible "
            "with the available network, credentials, binaries, and hardware."
        ),
        "examples": examples,
    }


def serialized_index() -> str:
    return json.dumps(build_index(), indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check", action="store_true", help="fail if examples-index.json is not current"
    )
    args = parser.parse_args()
    generated = serialized_index()

    if args.check:
        current = INDEX_PATH.read_text() if INDEX_PATH.exists() else ""
        if current != generated:
            print(
                "examples-index.json is stale; run scripts/generate-example-index.py",
                file=sys.stderr,
            )
            return 1
        return 0

    INDEX_PATH.write_text(generated)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
