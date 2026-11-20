package app.foyer.player;

import android.app.Activity;
import android.app.PendingIntent;
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.pm.PackageInstaller;
import android.os.Build;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Installs an update with the system PackageInstaller (the standard session API), then lets Android
 * show its own confirm screen. The APK was already checked against the signed checksum by Rust, and
 * Android itself refuses an update that is not signed with the installed app's key.
 */
public class Installer {
    static final String ACTION = "app.foyer.player.INSTALL_RESULT";

    /** "" until Android reports back, then "ok" or a failure message. */
    public static volatile String result = "";

    public static void install(Activity act, String path) throws Exception {
        result = "";
        final Context app = act.getApplicationContext();
        BroadcastReceiver receiver = new BroadcastReceiver() {
            @Override public void onReceive(Context c, Intent i) {
                int status = i.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE);
                if (status == PackageInstaller.STATUS_PENDING_USER_ACTION) {
                    Intent confirm = i.getParcelableExtra(Intent.EXTRA_INTENT);
                    if (confirm != null) {
                        confirm.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                        c.startActivity(confirm);
                    }
                    return;
                }
                result = status == PackageInstaller.STATUS_SUCCESS ? "ok" : "failed: " + i.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE);
                try { c.unregisterReceiver(this); } catch (Exception ignored) { }
            }
        };
        IntentFilter filter = new IntentFilter(ACTION);
        if (Build.VERSION.SDK_INT >= 33) {
            app.registerReceiver(receiver, filter, Context.RECEIVER_NOT_EXPORTED);
        } else {
            app.registerReceiver(receiver, filter);
        }

        PackageInstaller installer = app.getPackageManager().getPackageInstaller();
        PackageInstaller.SessionParams params = new PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL);
        params.setAppPackageName(app.getPackageName());
        if (Build.VERSION.SDK_INT >= 31) {
            params.setRequireUserAction(PackageInstaller.SessionParams.USER_ACTION_NOT_REQUIRED);
        }
        int id = installer.createSession(params);
        PackageInstaller.Session session = installer.openSession(id);
        try {
            File apk = new File(path);
            InputStream in = new FileInputStream(apk);
            OutputStream out = session.openWrite("update.apk", 0, apk.length());
            try {
                byte[] buf = new byte[64 * 1024];
                int n;
                while ((n = in.read(buf)) > 0) {
                    out.write(buf, 0, n);
                }
                session.fsync(out);
            } finally {
                out.close();
                in.close();
            }
            Intent intent = new Intent(ACTION).setPackage(app.getPackageName());
            int flags = PendingIntent.FLAG_UPDATE_CURRENT | (Build.VERSION.SDK_INT >= 31 ? PendingIntent.FLAG_MUTABLE : 0);
            PendingIntent pending = PendingIntent.getBroadcast(app, id, intent, flags);
            session.commit(pending.getIntentSender());
        } finally {
            session.close();
        }
    }
}
