# distrovmm

**Run full-system Linux virtual machines as easily as containers.**

`distrovmm` is a tool, written in Rust, that lets you pull a container image for a Linux distribution and instantly boot it as a lightweight, fully isolated virtual machine.

Get the ephemeral, on-demand feel of `docker run -it --rm fedora bash`, but with the security and isolation of a real VM.

```

$ distrovmm run fedora
INFO  [distrovmm] Pulling container image: fedora:latest...
INFO  [distrovmm] Converting image to VM rootfs...
INFO  [distrovmm] Creating VM configuration...
INFO  [distrovmm] Booting VM...
INFO  [distrovmm] Attaching to console...
[root@distrovmm /]\#

````

---

## 🚀 Features

* **Container-like Experience:** Get a root shell in a VM in seconds.
* **Zero Container-Runtime Dependencies:** **No Docker or Podman required.** `distrovmm` pulls and converts container images directly.
* **True VM Isolation:** Runs a real, lightweight VM using the **[libkrun](https://github.com/containers/libkrun)** library.
* **Cross-Platform:** Works natively on **macOS** (using `Hypervisor.framework`) and **Linux** (using `KVM`).
* **Simple CLI:** `distrovmm run <distro>` is all you need.

---

## 📋 Prerequisites

Before you begin, ensure you have the following installed on your system.

### All Platforms
* **Rust** and **Cargo**
* **`libkrun`** and its dependencies. This is the core engine for the VMs.

---

## 🔧 Installation

Currently, `distrovmm` must be built from source.

1.  **Clone the repository:**
    ```sh
    git clone [https://github.com/username/distrovmm.git](https://github.com/username/distrovmm.git)
    cd distrovmm
    ```

2.  **Build and install:**
    ```sh
    cargo build --release

    # Optionally, install it to your system path
    cargo install --path .
    ```

---

## Usage

The primary command is `run`. It will handle pulling the image, converting it, and booting the VM.

### Run a Distribution

To run the latest Fedora image:
```sh
distrovmm run fedora
````

To run the latest Ubuntu image:

```sh
distrovmm run ubuntu
```

The first time you run a distribution, `distrovmm` will:

1.  Pull the container image from its default registry.
2.  Convert the OCI image layers into a bootable root filesystem.
3.  Generate a minimal configuration with a kernel and bootloader.
4.  Cache this VM image for future, instantaneous boots.

On subsequent runs, it will boot the cached VM image immediately.

### Supported Distributions

`distrovmm` is in early development. Initial support is focused on:

  * **Fedora** (e.g., `fedora`, `fedora:43`)
  * **Ubuntu** (e.g., `ubuntu`, `ubuntu:24.04`)

-----

## ⚙️ How It Works

`distrovmm` bridges the gap between OCI container images and full-system VMs.

1.  **Image Pull:** It uses a native Rust OCI client to pull the specified container image without needing Docker/Podman.
2.  **Rootfs Creation:** It unpacks the container image layers into a disk image that will serve as the VM's root filesystem.
3.  **VM Generation:** It will use the container image distro's kernel, bootloader, which can be pulled via the revelant package manager of the Linux distro. Using this, it creates a standard VM configuration for the given distro, complete with the default bootloader for the distro. These assets are not pre-packaged with `distrovmm` or and fetched on demand.
4.  **Boot:** It uses `libkrun` to configure and launch the VM, pointing it to the generated rootfs and kernel.
5.  **Auto-Login:** The default `init` process of the distro is used inside the VM to bypass login prompts, set up the console, and execute `/bin/bash` as the `root` user, attaching it directly to your terminal.

-----

## 🗺️ Roadmap

  * [ ] Add support for more distributions (Debian, Arch Linux, Alpine).
  * [ ] Support volume mounting (`-v /path/to/host:/path/to/guest`).
  * [ ] Add basic networking and port forwarding.
  * [ ] Publish to `crates.io`.
  * [ ] Provide pre-compiled binaries for releases.

-----

## 🤝 Contributing

Contributions are welcome\! Please open an issue to discuss your ideas or submit a pull request.
