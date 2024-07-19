// Copyright 2020-2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::process::Command;

/// The p11ne enclave init
/// Spawns the p11ne enclave main applications
/// - the provisioning/rpc server
/// - the p11-kit server
fn main() {
    let mut cmd = Command::new("p11-kit");

    cmd.args(&[
            "server",
            "-n",
            "vsock:port=9999",
            "--provider",
            "/usr/lib/libvtok_p11.so",
            "-f",
            "-v",
            "pkcs11:",
        ]);

    cmd.env("P11_KIT_STRICT", "yes");

    cmd.spawn()
        .expect("p11-kit server failed to start.");

    Command::new("p11ne-server")
        .args(&["vsock", "10000"])
        .spawn()
        .expect("provisioning server failed to start.")
        .wait() // Block here. If the provisioning server dies, terminate the enclave.
        .expect("provisioning server has exited.");
}
