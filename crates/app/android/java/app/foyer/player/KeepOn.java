package app.foyer.player;

import android.app.Activity;
import android.view.Window;
import android.view.WindowManager;

/**
 * Keeps the screen on (or lets it sleep) from the UI thread. This is done here and not through the native
 * activity glue's set_window_flags: that takes the glue's window lock for writing, and from the player thread it
 * deadlocked with the UI thread (which holds the lock for reading while it runs its callbacks), freezing the app
 * when a film ended.
 */
public class KeepOn implements Runnable {
    private final Activity activity;
    private final boolean on;

    private KeepOn(Activity activity, boolean on) {
        this.activity = activity;
        this.on = on;
    }

    public static void set(Activity activity, boolean on) {
        activity.runOnUiThread(new KeepOn(activity, on));
    }

    @Override public void run() {
        Window w = activity.getWindow();
        if (on) {
            w.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        } else {
            w.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        }
    }
}
