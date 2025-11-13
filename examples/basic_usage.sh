#!/bin/bash
# Example usage of distrovmm
# This script demonstrates the basic functionality of the distrovmm tool

echo "=== distrovmm Basic Usage Examples ==="
echo ""

echo "1. Show help:"
echo "   $ distrovmm --help"
./target/debug/distrovmm --help
echo ""

echo "2. Run Fedora (latest):"
echo "   $ distrovmm run fedora"
echo "   (This would pull fedora:latest and boot it as a VM)"
echo ""

echo "3. Run Ubuntu 24.04:"
echo "   $ distrovmm run ubuntu:24.04"
echo "   (This would pull ubuntu:24.04 and boot it as a VM)"
echo ""

echo "4. Run Alpine Linux:"
echo "   $ distrovmm run alpine"
echo "   (This would pull alpine:latest - a minimal distribution)"
echo ""

echo "5. Run with debug logging:"
echo "   $ RUST_LOG=debug distrovmm run fedora"
echo "   (This shows detailed information about the pulling and boot process)"
echo ""

echo "=== Implementation Status ==="
echo ""
echo "✅ CLI interface implemented"
echo "✅ OCI image pulling from Docker Hub"
echo "✅ Manifest list handling (multi-platform)"
echo "✅ Layer extraction and rootfs creation"
echo "✅ Rootfs preparation for boot"
echo "✅ VM configuration structure"
echo "✅ Image caching"
echo ""
echo "🔨 TODO: Full libkrun integration"
echo "   - Add libkrun Rust bindings"
echo "   - Implement kernel extraction/download"
echo "   - Configure VM with libkrun API"
echo "   - Set up console I/O"
echo "   - Add networking support"
echo ""
