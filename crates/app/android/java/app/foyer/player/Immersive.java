package app.foyer.player;

import android.app.Activity;
import android.os.Build;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;

/** Hides or shows the navigation bar (and Samsung's taskbar, which counts as one) from the UI thread. */
public class Immersive implements Runnable {
    private final Activity activity;
    private final boolean on;

    private Immersive(Activity activity, boolean on) {
        this.activity = activity;
        this.on = on;
    }

    public static void set(Activity activity, boolean on) {
        activity.runOnUiThread(new Immersive(activity, on));
    }

    @Override public void run() {
        Window w = activity.getWindow();
        if (Build.VERSION.SDK_INT >= 30) {
            WindowInsetsController c = w.getInsetsController();
            if (c == null) return;
            if (on) {
                c.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                c.hide(WindowInsets.Type.navigationBars());
            } else {
                c.show(WindowInsets.Type.navigationBars());
            }
        } else {
            // IMMERSIVE_STICKY | LAYOUT_STABLE | FULLSCREEN | HIDE_NAVIGATION | LAYOUT_HIDE_NAVIGATION | LAYOUT_FULLSCREEN
            w.getDecorView().setSystemUiVisibility(on ? 0x1706 : 0);
        }
    }
}
