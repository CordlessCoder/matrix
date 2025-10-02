import com.googlecode.lanterna.TextCharacter;
import com.googlecode.lanterna.TextColor;
import com.googlecode.lanterna.screen.TerminalScreen;

import java.util.Locale;
import java.util.Random;

public class Line {
    private final long seed;
    public final float speed;
    public short x;
    public float y = 0f;
    public byte length;

    public Line(final Random rng, final short width) {
        seed = rng.nextLong();
        length = (byte) rng.nextInt(3, 15);
        speed = rng.nextFloat(0.5f, 1.5f);
        x = (short) rng.nextInt(width);
        y = -length;
    }

    public void draw(final TerminalScreen screen, final TextColor.RGB color) {
        final int row_bound = Math.min((int) y + length, screen.getTerminalSize().getRows());
        final var rng = new Random(seed);
        final int r = color.getRed();
        final int g = color.getGreen();
        final int b = color.getBlue();
        for (int row = (int) y; row < row_bound; row++) {
            final char c = RngHelper.randomChar(rng);
            if (row < 0) {
                continue;
            }
            final float value = (float) (row + 1 - y) / (float) length;
            final float brightness = 0.3f + (value * 0.7f * (speed / 1.5f * 0.8f));

            screen.setCharacter(x, row, TextCharacter.fromCharacter(
                    c,
                    new TextColor.RGB((int) (r * brightness), (int) (g * brightness), (int) (b * brightness)),
                    null)[0]);
        }
    }

    public void advance() {
        this.y += this.speed;
    }

    public boolean offScreen(final int height) {
        return this.y >= height;
    }
}

class RngHelper {
    public static final String upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    public static final String lower = upper.toLowerCase(Locale.ENGLISH);
    public static final String digits = "0123456789";
    public static final String alphanum = upper + lower + digits;

    public static char randomChar(final Random rng) {
        final int idx = rng.nextInt(alphanum.length());
        return alphanum.charAt(idx);
    }
}
