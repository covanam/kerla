# Kerla
[![CI](https://github.com/nuta/kerla/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/nuta/kerla/actions/workflows/ci.yml)
[![Discord Chat](https://img.shields.io/discord/904705655095582752?logo=discord&style=social)](https://discord.gg/6Pu4ujpp6h)

![screenshot](screenshot.png)

Kerla is a monolithic operating system kernel written from scratch in Rust which aims to be
compatible with the Linux ABI, that is, it runs Linux binaries without any modifications.

- Implements *NIX process concepts: context switching, signals, `fork(2)`, `execve(2)`, `wait4(2)`, etc.
- Supports commonly used system calls like `write(2)`, `stat(2)`, `mmap(2)`, `pipe(2)`, `poll(2)`, ...
- No disk support for now: initramfs is mounted as the root file system.
- Pseudo file systems: tmpfs and devfs.
- [smoltcp](https://github.com/smoltcp-rs/smoltcp)-based TCP/IP support.
- Implements tty and pseudo terminal (pty).
- Supports QEMU and Firecracker (with virtio-net device driver).
- Supports x86_64.
- Docker-based initramfs build system.

***Check out [my blog post](https://seiya.me/writing-linux-clone-in-rust) for motivation and my thoughts on writing an OS kernel in Rust.***

## Demo: SSH into Kerla!

You can play with Kerla over ssh. Your login is not visible from others (except
me): we automatically launch a dedicated microVM on Firecracker for each TCP
connection.

```
$ ssh root@demo.kerla.dev
```

If you found bugs or missing features, let me know on GitHub issues :)

## Road Map

See [Quickstart](https://kerla.dev/docs/quickstart.html) for instructions on building from source, running on emulators, etc.

## Current Roadmap
[Roadmap - Run a Node.js Web Application on Kerla on Firecracker on AWS](https://github.com/nuta/kerla/projects/1)

## Compatibility

See [here](https://github.com/nuta/kerla/blob/main/Documentation/compatibility.md) for the current status.

## Contributing

Send me bug reports, feature requests, and patches on [GitHub](https://github.com/nuta/kerla) for example:

- **Implementing missing features:** majority of existing Linux applications won't work due to the lack of features.
- **Writing documentation:** I think Kerla could be good material to learn how an operating system kernel works.
- **Trying to experiment with Rust-y ideas:** for example currently I'm interested in [GhostCell](http://plv.mpi-sws.org/rustbelt/ghostcell/).

## License

See [LICENSE.md](https://github.com/nuta/kerla/blob/main/LICENSE.md).

## Related Work

Emulating Linux ABI is not a novel work. Some UNIX-like kernels like [FreeBSD](https://docs.freebsd.org/en_US.ISO8859-1/articles/linux-emulation/article.html) and [NetBSD](https://www.netbsd.org/docs/guide/en/chap-linux.html) already have their own Linux emulation layers. Windows has a well-known feature called [Windows Subsystem for Linux (WSL)](https://github.com/microsoft/WSL) which enables running Linux binaries seamlessly. WSL 1 implements the feature by ABI emulation. WSL 2 runs the real Linux kernel using the hardware-accelerated virtualization (Hyper-V).

Aside from general-purpose operating systems, there're some attractive projects related to the Linux ABI emualtion. [OSv](https://github.com/cloudius-systems/osv/wiki/OSv-Linux-ABI-Compatibility) is a unikernel which runs unmodified Linux binaries. [rCore](https://github.com/rcore-os/rCore) is a teaching operating system which implements the Linux ABI in Rust. [Noah](https://dl.acm.org/doi/10.1145/3381052.3381327) suggests an intriguing approach to run unmodified Linux binaries on macOS.
