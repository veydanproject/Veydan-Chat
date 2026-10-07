// SPDX-FileCopyrightText: 2026 Veydan Project
// SPDX-License-Identifier: LicenseRef-PolyForm-Perimeter-1.0.1

fn main() {
    // The Java side of the engine of calls, on Android (the product's build
    // script alone can ask the linker for it).
    veydan_build_cfg::android_webrtc_jni_exports();
    tauri_build::build()
}
