#!/usr/bin/env python3
"""Teach cargo-apk's manifest writer the `supports_picture_in_picture` activity attribute.

cargo-apk 0.10 cannot emit android:supportsPictureInPicture, which Android requires before an
activity may enter picture-in-picture. Usage: patch-cargo-apk.py <ndk-build source dir> <cargo-apk source dir>
"""
import sys, pathlib

ndk, apk = map(pathlib.Path, sys.argv[1:3])
m = ndk / "src" / "manifest.rs"
s = m.read_text()
field = '''    #[serde(rename(serialize = "android:alwaysRetainTaskState"))]
    pub always_retain_task_state: Option<bool>,
'''
assert field in s, "ndk-build manifest layout changed"
s = s.replace(field, field + '''    #[serde(rename(serialize = "android:supportsPictureInPicture"))]
    pub supports_picture_in_picture: Option<bool>,
''', 1)
dflt = "            always_retain_task_state: None,\n"
assert dflt in s, "ndk-build Activity::default changed"
s = s.replace(dflt, dflt + "            supports_picture_in_picture: None,\n", 1)
m.write_text(s)

t = apk / "Cargo.toml"
t.write_text(t.read_text() + f'\n[patch.crates-io]\nndk-build = {{ path = "{ndk.resolve()}" }}\n')
print("patched", m)
