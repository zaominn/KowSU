# KowSU SUSFS integration

KowSU uses one `master` branch for both plain and SUSFS-capable kernels.
Kernel builds enable or disable the integrated SUSFS driver with
`CONFIG_KSU_SUSFS`; they do not select a different KowSU source branch.

The integration is based on the GPL-3.0 SUSFS adaptation published in
[`cctv18/susfs4oki`](https://github.com/cctv18/susfs4oki), which in turn
integrates [`simonpunk/susfs4ksu`](https://gitlab.com/simonpunk/susfs4ksu).
Current ReSukiSU behavior is the compatibility reference for the single-manager
runtime probe: the manager calls `ksud susfs show version`, exposes SUSFS
settings only when that succeeds, and otherwise continues as a plain KowSU
manager without changing APKs.
