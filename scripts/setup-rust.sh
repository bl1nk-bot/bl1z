#!/usr/bin/env bash
set -e

# ติดตั้ง rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# โหลด PATH ให้แน่ใจว่า cargo ใช้งานได้
export PATH="$HOME/.cargo/bin:$PATH"
source "$HOME/.cargo/env"

# ติดตั้ง component ที่ต้องใช้
rustup component add rustfmt clippy
