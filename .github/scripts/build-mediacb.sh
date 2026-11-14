#!/usr/bin/env bash
# Compiles crates/app/android/java/**/*.java to crates/app/android/mediacb.dex (embedded into the
# Android build with include_bytes!). Needs a JDK and an Android SDK (ANDROID_HOME).
set -euo pipefail
cd "$(dirname "$0")/../.."
SDK="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-/usr/local/lib/android/sdk}}"
BT="$(ls -d "$SDK"/build-tools/* | sort -V | tail -1)"
JAR="$(ls -d "$SDK"/platforms/android-*/android.jar | sort -V | tail -1)"
OUT="$(mktemp -d)"
javac --release 8 -cp "$JAR" -d "$OUT/classes" $(find crates/app/android/java -name '*.java')
"$BT/d8" --min-api 26 --lib "$JAR" --output "$OUT" $(find "$OUT/classes" -name '*.class')
cp "$OUT/classes.dex" crates/app/android/mediacb.dex
echo "wrote crates/app/android/mediacb.dex ($(stat -c %s crates/app/android/mediacb.dex) bytes)"
