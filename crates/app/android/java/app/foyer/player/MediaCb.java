package app.foyer.player;

import android.media.session.MediaSession;

/**
 * Receives the system's media buttons (notification shade, lock screen, headset, Bluetooth) for
 * the playing video and hands them to Rust. Compiled to a dex by CI and added to the APK as classes.dex
 * (cargo-apk packages no Java code itself; see .github/scripts/patch-cargo-apk.py).
 */
public class MediaCb extends MediaSession.Callback {
    static native void nativeAction(int code, long arg);

    @Override public void onPlay() { nativeAction(1, 0); }
    @Override public void onPause() { nativeAction(2, 0); }
    @Override public void onSkipToNext() { nativeAction(3, 0); }
    @Override public void onSkipToPrevious() { nativeAction(4, 0); }
    @Override public void onStop() { nativeAction(5, 0); }
    @Override public void onSeekTo(long positionMs) { nativeAction(6, positionMs); }
    @Override public void onFastForward() { nativeAction(7, 0); }
    @Override public void onRewind() { nativeAction(8, 0); }
}
