package app.foyer.player;

import android.app.Activity;
import android.app.DownloadManager;
import android.content.Context;
import android.database.Cursor;
import android.net.Uri;
import android.os.Environment;
import android.os.ParcelFileDescriptor;

import java.io.FileInputStream;
import java.security.MessageDigest;

/**
 * Downloads an update with Android's own DownloadManager. Foyer never installs anything itself (it has no
 * install permission, which Google Play Protect objected to): when the download finishes, Android's
 * notification offers to open it, and the standard install screen takes over. Rust checks the file against
 * the release's signed checksum first (see {@link #sha256}), and Android refuses any app not signed with
 * Foyer's key.
 */
public class Updater {
    private static long id = -1;

    public static void start(Activity act, String url, String name) {
        DownloadManager dm = (DownloadManager) act.getSystemService(Context.DOWNLOAD_SERVICE);
        if (id >= 0) {
            dm.remove(id);
        }
        DownloadManager.Request r = new DownloadManager.Request(Uri.parse(url));
        r.setTitle("Foyer update");
        r.setDescription(name);
        r.setMimeType("application/vnd.android.package-archive");
        r.setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED);
        r.setDestinationInExternalPublicDir(Environment.DIRECTORY_DOWNLOADS, "Foyer/" + name);
        id = dm.enqueue(r);
    }

    /** "run:PCT" while downloading, "ok" when finished, "fail:reason" otherwise. */
    public static String poll(Activity act) {
        if (id < 0) {
            return "fail:The download was not started.";
        }
        DownloadManager dm = (DownloadManager) act.getSystemService(Context.DOWNLOAD_SERVICE);
        Cursor c = dm.query(new DownloadManager.Query().setFilterById(id));
        try {
            if (c == null || !c.moveToFirst()) {
                return "fail:The download disappeared.";
            }
            int status = c.getInt(c.getColumnIndexOrThrow(DownloadManager.COLUMN_STATUS));
            if (status == DownloadManager.STATUS_SUCCESSFUL) {
                return "ok";
            }
            if (status == DownloadManager.STATUS_FAILED) {
                return "fail:Android's download failed (code " + c.getInt(c.getColumnIndexOrThrow(DownloadManager.COLUMN_REASON)) + ").";
            }
            long got = c.getLong(c.getColumnIndexOrThrow(DownloadManager.COLUMN_BYTES_DOWNLOADED_SO_FAR));
            long total = c.getLong(c.getColumnIndexOrThrow(DownloadManager.COLUMN_TOTAL_SIZE_BYTES));
            return "run:" + (total > 0 ? got * 100 / total : 0);
        } finally {
            if (c != null) {
                c.close();
            }
        }
    }

    /** SHA-256 (hex) of the finished download, or "" if it can't be read. */
    public static String sha256(Activity act) {
        try {
            DownloadManager dm = (DownloadManager) act.getSystemService(Context.DOWNLOAD_SERVICE);
            ParcelFileDescriptor pfd = dm.openDownloadedFile(id);
            FileInputStream in = new FileInputStream(pfd.getFileDescriptor());
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            byte[] buf = new byte[64 * 1024];
            int n;
            while ((n = in.read(buf)) > 0) {
                md.update(buf, 0, n);
            }
            in.close();
            pfd.close();
            StringBuilder sb = new StringBuilder();
            for (byte b : md.digest()) {
                sb.append(String.format("%02x", b));
            }
            return sb.toString();
        } catch (Exception e) {
            return "";
        }
    }

    /** Removes the download (and its file) again, for example when the checksum does not match. */
    public static void cancel(Activity act) {
        if (id >= 0) {
            ((DownloadManager) act.getSystemService(Context.DOWNLOAD_SERVICE)).remove(id);
            id = -1;
        }
    }
}
