package app.foyer.player;

import android.app.Activity;
import android.content.Intent;
import android.net.Uri;

/**
 * Opens the release APK's link in the browser. Foyer never downloads or installs an update itself on
 * Android: it has no install permission and no installer, because Google Play Protect blocks apps that
 * declare one. The browser downloads the file and the user opens it; Android's own installer then
 * takes over and refuses anything that is not signed with Foyer's key.
 */
public class Updater {
    public static void open(Activity act, String url) {
        act.startActivity(new Intent(Intent.ACTION_VIEW, Uri.parse(url)));
    }
}
