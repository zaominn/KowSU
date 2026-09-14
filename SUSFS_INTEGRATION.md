# KowSU SUSFS integration branch

The `susfs` branch is the KowSU kernel-driver baseline used by ZAOMIN kernel
workflows that enable `CONFIG_KSU_SUSFS`.

The integration is based on the GPL-3.0 SUSFS adaptation published in
[`cctv18/susfs4oki`](https://github.com/cctv18/susfs4oki), which in turn
integrates [`simonpunk/susfs4ksu`](https://gitlab.com/simonpunk/susfs4ksu).
Current ReSukiSU behavior was used as the compatibility reference for the
manager-side SUSFS version query and daemon-first fallback order.

This branch deliberately tracks commit `9537542c6aabc169ab34c27d38ba3b97cc09c16c`
as its KowSU base. The later KowSU supercall/sucompat rewrite is not compatible
with the 6.12 `susfs4oki` KernelSU patch; partially applying it produces a
driver that can compile incorrectly or fail at runtime.
