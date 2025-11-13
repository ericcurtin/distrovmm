# distrovmm Implementation Details

## Overview

This document describes the implementation of `distrovmm`, a tool for running full-system Linux virtual machines from OCI container images.

## Architecture

The project is structured into four main modules:

### 1. CLI Module (`src/cli.rs`)

Implements the command-line interface using the `clap` crate with derive macros for parsing arguments.

**Key features:**
- Simple `distrovmm run <image>` command
- Extensible subcommand structure for future enhancements

### 2. OCI Module (`src/oci.rs`)

Handles pulling container images from OCI-compliant registries without requiring Docker or Podman.

**Key features:**
- Native Rust implementation using `reqwest` for HTTP requests
- Docker Hub authentication with bearer tokens
- Support for both single manifests and manifest lists (multi-platform images)
- Automatic platform selection (defaults to amd64/linux)
- Layer caching to avoid re-downloading

**Implementation details:**
- Authenticates with Docker Hub token service
- Fetches image manifests with proper content negotiation
- Handles OCI manifest lists by selecting the appropriate platform
- Downloads image layers as compressed tar archives
- Caches downloaded layers locally for reuse

### 3. Rootfs Module (`src/rootfs.rs`)

Extracts OCI image layers to create a bootable root filesystem.

**Key features:**
- Extracts gzip-compressed tar layers in order
- Preserves file permissions, timestamps, and extended attributes
- Prepares the filesystem for VM boot
- Creates necessary system directories (dev, proc, sys, tmp, etc.)
- Generates an init script for auto-login

**Implementation details:**
- Uses `flate2` for decompression and `tar` for extraction
- Layers are applied sequentially, mimicking container runtime behavior
- Creates a simple init script that:
  - Mounts essential pseudo-filesystems (proc, sys, dev, tmp)
  - Automatically logs in as root
  - Executes `/bin/bash` for interactive use

### 4. VM Module (`src/vm.rs`)

Manages the VM lifecycle including caching, configuration, and booting.

**Key features:**
- VM image caching to enable fast subsequent boots
- Rootfs validation and distribution detection
- VM configuration generation

**Current status:**
The VM module currently simulates the boot process. Full integration with `libkrun` requires:
1. Adding Rust bindings to libkrun (e.g., via `libkrun-sys` or FFI)
2. Kernel extraction from the rootfs or downloading from package repositories
3. Configuring libkrun with:
   - Kernel and initrd paths
   - Rootfs mount point
   - Console setup
   - Memory and CPU allocation
   - Network configuration

## Data Flow

1. **User runs:** `distrovmm run fedora`
2. **VM Module:** Checks cache for existing VM image
3. **OCI Module:** If not cached, pulls the `fedora:latest` image from Docker Hub
   - Authenticates with token service
   - Fetches manifest (or manifest list)
   - Downloads all layers
4. **Rootfs Module:** Extracts layers to create rootfs
   - Decompresses and unpacks each layer
   - Prepares boot environment
5. **VM Module:** Configures and boots VM
   - Detects distribution from `/etc/os-release`
   - Would configure libkrun (in full implementation)
   - Would attach to console for interactive use

## Dependencies

### Core Dependencies
- `clap`: Command-line argument parsing
- `tokio`: Async runtime for concurrent I/O
- `reqwest`: HTTP client for registry communication
- `serde` + `serde_json`: JSON serialization/deserialization
- `tar`: TAR archive extraction
- `flate2`: Gzip compression/decompression
- `anyhow`: Error handling with context
- `log` + `env_logger`: Structured logging

### Utility Dependencies
- `sha2` + `hex`: Checksum verification (for future use)
- `tempfile`: Temporary file management
- `dirs`: Cross-platform directory detection

## Security Considerations

1. **Image Verification**: Currently does not verify image signatures. Should add support for:
   - Content digests validation
   - Docker Content Trust / Notary integration
   - GPG signature verification

2. **Rootfs Isolation**: Extracted rootfs should be validated and sanitized:
   - Check for malicious symlinks
   - Validate file permissions
   - Consider using user namespaces

3. **Network Security**: Registry communication:
   - Uses HTTPS for all registry communication
   - Stores tokens in memory only (never persisted)
   - Should add support for private registries with authentication

## Future Enhancements

### libkrun Integration

To complete the implementation, integrate with libkrun:

```rust
// Example libkrun integration (pseudocode)
use libkrun_sys::*;

fn boot_vm(rootfs_dir: &Path, kernel_path: &Path) -> Result<()> {
    let ctx = libkrun_new_ctx()?;
    
    // Set up rootfs
    libkrun_set_root(ctx, rootfs_dir.to_str().unwrap())?;
    
    // Set kernel
    libkrun_set_kernel_path(ctx, kernel_path.to_str().unwrap())?;
    
    // Configure VM resources
    libkrun_set_vm_config(ctx, 2, 2048)?; // 2 CPUs, 2GB RAM
    
    // Set up console
    libkrun_set_console(ctx, CONSOLE_TYPE_SERIAL)?;
    
    // Start VM
    libkrun_start_enter(ctx)?;
    
    Ok(())
}
```

### Kernel Management

Add automatic kernel extraction or download:

1. Detect distribution from `/etc/os-release`
2. For Fedora: Extract kernel from `/boot` in rootfs or use DNF
3. For Ubuntu: Extract from `/boot` or use APT
4. Cache kernels separately from rootfs

### Network Configuration

Add basic networking support:

- Port forwarding: `-p 8080:80`
- Host networking mode
- Bridge networking for multi-VM communication

### Volume Mounting

Support host directory mounting:

- `-v /host/path:/container/path`
- Shared folders using virtio-9p or virtiofs

### Image Management

Add commands for managing cached images:

- `distrovmm images` - List cached images
- `distrovmm rmi <image>` - Remove cached image
- `distrovmm pull <image>` - Pre-pull image without running

## Testing

### Current Testing Status

The implementation has been tested to ensure:
- ✅ Project builds successfully
- ✅ CLI argument parsing works correctly
- ✅ OCI manifest fetching handles manifest lists
- ✅ Platform selection (amd64/linux) works
- ✅ Rootfs extraction preserves permissions

### Testing with Limited Network

In environments with network restrictions, the implementation gracefully handles:
- Connection failures with clear error messages
- Partial downloads are not cached
- Token authentication works when network allows

### Future Testing

To fully test the implementation:

1. **Integration Tests**: Test with real container images in a network-enabled environment
2. **Unit Tests**: Add tests for:
   - Manifest parsing
   - Layer extraction
   - Cache management
3. **End-to-End Tests**: With libkrun integration:
   - Boot Fedora, Ubuntu, Alpine
   - Verify console access
   - Test network connectivity
   - Validate file system state

## Building and Running

### Build

```bash
cargo build --release
```

### Run

```bash
# Run with default (latest) tag
distrovmm run fedora

# Run with specific tag
distrovmm run ubuntu:24.04

# Enable debug logging
RUST_LOG=debug distrovmm run alpine
```

### Cache Location

Images are cached in:
- Linux: `~/.cache/distrovmm/`
- macOS: `~/Library/Caches/distrovmm/`
- Windows: `C:\Users\<user>\AppData\Local\distrovmm\cache\`

## Code Quality

- **Error Handling**: Uses `anyhow::Result` with contextual error messages
- **Async/Await**: Leverages Tokio for efficient I/O operations
- **Logging**: Comprehensive logging at INFO, DEBUG levels
- **Code Organization**: Clean module separation with clear responsibilities
- **Documentation**: Inline documentation for public APIs

## License

Apache License 2.0 - See LICENSE file for details.
