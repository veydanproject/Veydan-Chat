// SPDX-FileCopyrightText: 2026 Veydan Project
// SPDX-License-Identifier: LicenseRef-PolyForm-Perimeter-1.0.1

fn main() {
    // The Java side of the engine of calls, on Android (the product's build
    // script alone can ask the linker for it).
    veydan_build_cfg::android_webrtc_jni_exports();
    // Its Objective-C categories, on macOS and iOS (ROO-869: without -ObjC
    // the process dies before `main`).
    veydan_build_cfg::darwin_webrtc_objc();
    tauri_build::build()
}
