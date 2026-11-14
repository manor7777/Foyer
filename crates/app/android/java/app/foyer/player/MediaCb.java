package app.foyer.player;

import android.media.session.MediaSession;

/**
 * Receives the system's media buttons (notification shade, lock screen, headset, Bluetooth) for
 * the playing video and hands them to Rust. Compiled to a dex by CI and loaded at runtime with an
 * InMemoryDexClassLoader, because cargo-apk packages no Java code.
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
