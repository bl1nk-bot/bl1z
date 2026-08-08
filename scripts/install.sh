#!/usr/bin/env bash
# scripts/install.sh - Build and install the bl1z CLI for users.
#
# Termux (ลงใน $PREFIX/bin ที่อยู่ใน PATH อยู่แล้ว):
#     ./scripts/install.sh
# อื่น ๆ (ลงใน ~/.cargo/bin):
#     ./scripts/install.sh
#
# รองรับการกำหนด root เอง:
#     PREFIX=/usr/local ./scripts/install.sh
set -euo pipefail
cd "$(dirname "$0")/.."

ROOT="${PREFIX:-$HOME/.cargo}"
echo "📦 Installing bl1z CLI into ${ROOT}/bin ..."
cargo install --path . --root "$ROOT" --force
echo "✅ Done. Run: ${ROOT}/bin/bl1z --version"
